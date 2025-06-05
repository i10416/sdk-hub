use quote::quote;
use syn::{ext::IdentExt, spanned::Spanned, Field};

pub fn derive_field_names(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let item = syn::parse2::<syn::ItemStruct>(input).unwrap();

    let struct_name = item.ident;
    let (impl_generics, type_generics, where_clause) = &item.generics.split_for_impl();
    let field_name_results = item.fields.iter().map(field_name).collect::<Vec<_>>();
    let mut field_names = vec![];
    let mut errors = vec![];
    for result in field_name_results {
        match result {
            Ok(name) => field_names.push(name),
            Err(err) => errors.push(err),
        }
    }
    if errors.is_empty() {
        let expr = quote! {
            impl  #impl_generics ::mirrors::FieldNames for #struct_name #type_generics #where_clause {
                fn field_names() -> &'static [&'static str] {
                    &[#(#field_names),*]
                }
            }
        };
        expr.into()
    } else {
        let the_error = match &mut errors[..] {
            [e, es @ ..] => es.iter().fold(e, |acc, next| {
                acc.combine(next.clone());
                acc
            }),
            [] => unreachable!("errors MUST NOT empty"),
        };
        the_error.to_compile_error().into()
    }
}

fn field_name(f: &Field) -> Result<String, syn::Error> {
    let field_name_attr = f
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("field_names"));
    match field_name_attr {
        Some(attr) => {
            let name_val: syn::MetaNameValue = attr.parse_args()?;
            match name_val.path {
                path if path.is_ident("rename") => match name_val.value {
                    syn::Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Str(name),
                        ..
                    }) => Ok(name.value()),
                    expr => Err(syn::Error::new(expr.span(), "expected literal string")),
                },
                path => {
                    let path_name =
                        path.get_ident()
                            .map(|id| id.to_string())
                            .ok_or(syn::Error::new(
                                path.span(),
                                "empty attribute argument name is invalid",
                            ))?;
                    Err(syn::Error::new(
                        path.span(),
                        format!("Unexpected path for field_names attribute: {path_name:?}. Expected one of [`rename`]"),
                    ))
                }
            }
        }
        None => f
            .ident
            .clone()
            .map(|id| id.unraw().to_string())
            .ok_or(syn::Error::new(f.span(), "tuple struct is not supported")),
    }
}
