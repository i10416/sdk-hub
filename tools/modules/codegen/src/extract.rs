use syn::{Attribute, LitStr};

use crate::FieldSemantics;

pub fn extract_semantics<'a>(
    semantics: &'a mut FieldSemantics,
    attrs: &[Attribute],
) -> &'a FieldSemantics {
    for attr in attrs
        .iter()
        .filter(|attr| attr.meta.path().is_ident("serde"))
    {
        let _ = attr
            .parse_nested_meta(|meta| {
                if meta.path.is_ident("rename") {
                    let value = meta.value()?;
                    let name: LitStr = value.parse()?;
                    semantics.rename = Some(name.value());
                    Ok(())
                } else if meta.path.is_ident("alias") {
                    let value = meta.value()?;
                    let name: LitStr = value.parse()?;
                    semantics.alias = Some(name.value());
                    Ok(())
                } else if meta.path.is_ident("default") || meta.path.is_ident("skip_serializing_if")
                {
                    semantics.fill_with_default_value = true;
                    Ok(())
                } else {
                    println!("{:?}", meta.path.get_ident());
                    Ok(())
                }
            })
            .ok();
    }
    semantics
}
