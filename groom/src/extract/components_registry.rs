use std::{any::TypeId, collections::{BTreeSet, HashMap, HashSet, hash_map}};

use utoipa::{ToSchema, openapi::{Components, ComponentsBuilder, ContentBuilder, OpenApiBuilder, RefOr, Schema}};

#[derive(Clone, PartialEq)]
pub struct ComponentEntry {
    pub ref_location: String,
    pub schema: Schema,
}

#[derive(Default)]
pub struct ComponentsRegistry {
    components: HashMap<String, ComponentEntry>,
    seen_types: HashSet<TypeId>,
}

impl ComponentsRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_components<T: ToSchema + 'static>(&mut self) {
        let tid = TypeId::of::<T>();
        let is_new_type = self.seen_types.insert(tid);
        if !is_new_type {
            return
        }

        let mut schemas = Vec::<(String, RefOr<Schema>)>::new();
        T::schemas(&mut schemas);

        for (name, schema) in schemas {
            self.add_component(name.clone(), schema);
        }

        self.add_component(T::name().into(), T::schema());
    }

    fn add_component(&mut self, name: String, schema: RefOr<Schema>) {
        let schema = match schema {
            RefOr::T(s) => s,
            RefOr::Ref(r) => return,
                // panic!(
                //     "ComponentsRegistry::add_component: schema for `{}` is a ref to `{}`, expected to be a schema!",
                //     name,
                //     r.ref_location
                // ),
        };

        match self.components.entry(name.clone()) {
            hash_map::Entry::Occupied(_occupied_entry) => {
                if _occupied_entry.get().schema != schema {
                    panic!(
                        "ComponentsRegistry::add_component: schema with name `{}` is already defined for another type!",
                        name
                    );
                }
            },
            hash_map::Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(ComponentEntry{ 
                    ref_location: name.clone(),
                    schema: schema.clone()
                });
            },
        };
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
    mod add_components {
        use crate::extract::ComponentsRegistry;

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

            reg.add_components::<submod1::SubStruct>();
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

            use crate::extract::ComponentsRegistry;

            #[derive(utoipa::ToSchema)]
            pub struct Struct1 {
                pub v: i32,
            }

            #[test]
            fn allow_shared_components() {
                let mut reg1 = ComponentsRegistry::new();
                reg1.add_components::<Struct1>();

                let mut reg2 = ComponentsRegistry::new();
                reg2.add_components::<Struct1>();

                let c = ComponentsBuilder::new().build();
                let c = reg1.into_components(c);
                let c = reg2.into_components(c);
            }
        }

        mod overlapping_components {
            use utoipa::openapi::ComponentsBuilder;

            use crate::extract::ComponentsRegistry;

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
                reg1.add_components::<sub1::Struct1>();

                let mut reg2 = ComponentsRegistry::new();
                reg2.add_components::<sub2::Struct1>();

                let c = ComponentsBuilder::new().build();
                let c = reg1.into_components(c);
                let c = reg2.into_components(c);
            }
        }

        mod identical_components {
            use utoipa::openapi::ComponentsBuilder;

            use crate::extract::ComponentsRegistry;

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
                reg.add_components::<Struct1>();
                reg.add_components::<submod::Struct1>();

                let c = ComponentsBuilder::new().build();
                let c = reg.into_components(c);
            }
        }
    }
}

