use darling::FromMeta;
use syn::Attribute;

pub(crate) fn parse_attr<T: FromMeta>(name: &str, attrs: &[Attribute]) -> Result<Option<T>, darling::Error> {
    for attr in attrs {
        if attr.path().is_ident(name) {
            return Ok(Some(T::from_meta(&attr.meta)?));
        }
    }
    Ok(None)
}

pub(crate) fn remove_attrs(name: &str, attrs: &mut Vec<Attribute>) {
    if let Some((idx, _)) = attrs
        .iter()
        .enumerate()
        .find(|(_, a)| a.path().is_ident(name))
    {
        attrs.remove(idx);
    }
}
