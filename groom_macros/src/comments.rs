use syn::{Attribute, Expr, ExprLit, Lit, Meta, Result};

/// Utility function to acquire text of docblock of any element.
pub(crate) fn get_docblock(attrs: &[Attribute]) -> Result<Option<String>> {
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

    if full_docs.is_empty() {
        Ok(None)
    } else {
        Ok(Some(full_docs))
    }
}

/// Utility function to acquire docblock of any element and split it into two parts:
///   - 0: summary (first paragraph);
///   - 1: description (the rest of it).
pub(crate) fn get_docblock_parts(attrs: &[Attribute]) -> Result<(Option<String>, Option<String>)> {
    let doc = get_docblock(attrs)?;

    match doc {
        Some(doc) => match doc.split_once("\n\n") {
            Some((summary, description)) =>
                Ok((Some(summary.to_string()), Some(description.to_string()))),

            None =>
                Ok((Some(doc), None)),
        },
        None =>
            Ok((None, None)),
    }
}


/// Utility function to remove docblock from any element.
pub(crate) fn remove_docblock(attrs: &mut Vec<Attribute>) {
    attrs.retain(|attr| !attr.path().is_ident("doc"));
}
