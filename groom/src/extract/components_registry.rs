use std::{any::TypeId, collections::{BTreeSet, HashMap, HashSet, hash_map}, sync::OnceLock};

use ::utoipa::openapi::{Ref, RefOr, schema::RefBuilder};
use utoipa::{PartialSchema, ToSchema, openapi::{Components, ComponentsBuilder, Schema}};

#[derive(Clone, PartialEq)]
pub struct ComponentEntry {
    pub schema: Schema,
    pub reference: Ref,
}

#[derive(Default)]
pub struct ComponentsRegistry {
    components: HashMap<String, ComponentEntry>,
    seen_types: HashSet<TypeId>
}

// these schemas will not be put under components.
// should be some kind of Set, but Schema doesn't implement Hash, Eq or Ord :(
static STD_TYPES_SCHEMAS: OnceLock<Vec<Schema>> = OnceLock::new();

impl ComponentsRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_components<T: ToSchema + 'static>(&mut self) -> RefOr<Schema> {
        let tid = TypeId::of::<T>();
        let name: String = T::name().into();

        if self.seen_types.contains(&tid) {
            return RefOr::Ref(
                self.components.get(&name)
                    .expect("component for a type that is already seen is expected to exist")
                    .reference
                    .clone()
            )
        }

        let mut schemas = Vec::<(String, RefOr<Schema>)>::new();
        T::schemas(&mut schemas);

        for (name, schema) in schemas {
            self.add_component(name.clone(), schema, None);
        }

        self.add_component(name, T::schema(), Some(tid))
    }

    fn add_component(&mut self, name: String, schema: RefOr<Schema>, tid: Option<TypeId>) -> RefOr<Schema> {
        let schema = match schema {
            RefOr::T(s) => s,
            RefOr::Ref(r) => return RefOr::Ref(r.clone()),
                // panic!(
                //     "ComponentsRegistry::add_component: schema for `{}` is a ref to `{}`, expected to be a schema!",
                //     name,
                //     r.ref_location
                // ),
        };

        if !Self::is_component(&schema) {
            return RefOr::T(schema.clone());
        }

        let entry = match self.components.entry(name.clone()) {
            hash_map::Entry::Occupied(e) => {
                if e.get().schema != schema {
                    panic!(
                        "ComponentsRegistry::add_component: schema with name `{}` is already defined for another type!",
                        name
                    );
                }
                e.get().reference.clone()
            },
            hash_map::Entry::Vacant(vacant_entry) => {
                let e = vacant_entry.insert(ComponentEntry{ 
                    reference: RefBuilder::new()
                        .ref_location(format!(
                            "#/components/schemas/{}",
                            crate::json_ptr::escape_json_pointer(
                                name.as_ref()
                            )
                        ))
                        .build(),
                    schema: schema.clone()
                });

                if let Some(tid) = tid {
                    self.seen_types.insert(tid);
                }

                e.reference.clone()
            },
        };

        return RefOr::<Schema>::Ref(entry)
    }

    fn is_component(schema: &Schema) -> bool {
        let std_types_schemas = STD_TYPES_SCHEMAS.get_or_init(|| {
            let mut set: Vec<Schema> = Vec::new();

            if let RefOr::T(s) = <String as PartialSchema>::schema() {
                set.push(s)
            };

            set
        });

        !std_types_schemas.contains(schema)
    }

    pub fn into_components(&self, c: Components) -> Components {
        for (k, this) in &self.components {
            let Some(RefOr::T(other)) = c.schemas.get(k) else {
                continue 
            };

            if this.schema == *other {
                continue;
            }

            panic!("Component `{k}` is defined more then once.");
        }

        self.components.iter().fold(ComponentsBuilder::from(c), |b, (name, entry)| {
            b.schema(name, entry.schema.clone())
        }).into()
    }
}

#[cfg(test)]
mod tests {
    use utoipa::openapi::{Ref, RefOr, Schema};

    fn reference_to(ref_location: &str) -> RefOr<Schema> {
        RefOr::Ref(Ref::new(ref_location))
    }

    mod add_components {
        use pretty_assertions::assert_eq;

        use crate::extract::{ComponentsRegistry, components_registry::tests::reference_to};

        mod submod1 {
            #[derive(utoipa::ToSchema)]
            pub struct SubStruct {
                pub v: i32,
            }
        }

        mod submod2{
            #[derive(utoipa::ToSchema)]
            pub struct SubStruct {
                pub v: String,
            }
        }

        #[derive(utoipa::ToSchema)]
        struct Struct1 {
            pub a: submod1::SubStruct,
            pub b: submod2::SubStruct,
        }

        #[derive(utoipa::ToSchema)]
        struct StructWithSub1 {
            pub a: submod1::SubStruct,
        }

        #[derive(utoipa::ToSchema)]
        struct StructWithSub2 {
            pub a: submod2::SubStruct,
        }

        #[test]
        #[should_panic(expected = "ComponentsRegistry::add_component: schema with name `SubStruct` is already defined for another type!")]
        fn detect_overlap_in_neighbours() {
            let mut reg = ComponentsRegistry::new();

            assert_eq!(reg.add_components::<submod1::SubStruct>(), reference_to("#/components/schemas/SubStruct"));
            reg.add_components::<submod2::SubStruct>();
        }

        #[test]
        #[should_panic(expected = "ComponentsRegistry::add_component: schema with name `SubStruct` is already defined for another type!")]
        fn detect_overlap_in_substructs() {
            let mut reg = ComponentsRegistry::new();

            reg.add_components::<Struct1>();
        }

        #[test]
        #[should_panic(expected = "ComponentsRegistry::add_component: schema with name `SubStruct` is already defined for another type!")]
        fn detect_overlap_in_substructs_of_neighbours() {
            let mut reg = ComponentsRegistry::new();

            reg.add_components::<StructWithSub1>();
            reg.add_components::<StructWithSub2>();
        }
    }

    mod into_components {
        mod shared_components {
            use utoipa::openapi::ComponentsBuilder;

            use crate::extract::{ComponentsRegistry, components_registry::tests::reference_to};

            #[derive(utoipa::ToSchema)]
            pub struct Struct1 {
                pub v: i32,
            }

            #[test]
            fn allow_shared_components() {
                let mut reg1 = ComponentsRegistry::new();
                assert_eq!(reg1.add_components::<Struct1>(), reference_to("#/components/schemas/Struct1"));

                let mut reg2 = ComponentsRegistry::new();
                assert_eq!(reg2.add_components::<Struct1>(), reference_to("#/components/schemas/Struct1"));

                let c = ComponentsBuilder::new().build();
                let c = reg1.into_components(c);
                let c = reg2.into_components(c);
            }
        }

        mod overlapping_components {
            use utoipa::openapi::ComponentsBuilder;

            use crate::extract::{ComponentsRegistry, components_registry::tests::reference_to};

            mod sub1 {
                #[derive(utoipa::ToSchema)]
                pub struct Struct1 {
                    pub v: i32,
                }
            }

            mod sub2 {
                #[derive(utoipa::ToSchema)]
                pub struct Struct1 {
                    pub v: String,
                }
            }

            #[test]
            #[should_panic(expected = "Component `Struct1` is defined more then once.")]
            fn disallow_different_components_with_same_name() {
                let mut reg1 = ComponentsRegistry::new();
                assert_eq!(reg1.add_components::<sub1::Struct1>(), reference_to("#/components/schemas/Struct1"));

                let mut reg2 = ComponentsRegistry::new();
                assert_eq!(reg2.add_components::<sub2::Struct1>(), reference_to("#/components/schemas/Struct1"));

                let c = ComponentsBuilder::new().build();
                let c = reg1.into_components(c);
                let _ = reg2.into_components(c);
            }
        }

        mod identical_components {
            use utoipa::openapi::ComponentsBuilder;

            use crate::extract::{ComponentsRegistry, components_registry::tests::reference_to};

            #[derive(utoipa::ToSchema)]
            pub struct Struct1 {
                pub v: i32,
            }
            
            mod submod {
                #[derive(utoipa::ToSchema)]
                pub struct Struct1 {
                    pub v: i32,
                }
                
            }

            #[test]
            fn allow_identical_components() {
                let mut reg = ComponentsRegistry::new();
                assert_eq!(reg.add_components::<Struct1>(), reference_to("#/components/schemas/Struct1"));
                assert_eq!(reg.add_components::<submod::Struct1>(), reference_to("#/components/schemas/Struct1"));

                let c = ComponentsBuilder::new().build();
                let c = reg.into_components(c);
            }
        }
    }
}

