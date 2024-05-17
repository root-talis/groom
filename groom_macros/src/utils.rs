use syn::{Attribute, Expr, ExprLit, Lit, Meta, Result};

pub(crate) fn get_description(attrs: &[Attribute]) -> Result<Option<String>> {
    let mut full_docs = String::new();
    for attr in attrs {
        if attr.path().is_ident("doc") {
            if let Meta::NameValue(nv) = &attr.meta {
                if let Expr::Lit(ExprLit {
                    lit: Lit::Str(doc), ..
                }) = &nv.value
                {
                    let doc = doc.value();
                    let doc_str = doc.trim();
                    if !full_docs.is_empty() {
                        full_docs += "\n";
                    }
                    full_docs += doc_str;
                }
            }
        }
    }
    Ok(if full_docs.is_empty() {
        None
    } else {
        Some(full_docs)
    })
}

pub(crate) fn remove_description(attrs: &mut Vec<Attribute>) {
    attrs.retain(|attr| !attr.path().is_ident("doc"));
}

pub(crate) fn get_summary_and_description(
    attrs: &[Attribute],
) -> Result<(Option<String>, Option<String>)> {
    let doc = get_description(attrs)?;

    match doc {
        Some(doc) => match doc.split_once("\n\n") {
            Some((summary, description)) => {
                Ok((Some(summary.to_string()), Some(description.to_string())))
            }
            None => Ok((Some(doc), None)),
        },
        None => Ok((None, None)),
    }
}
