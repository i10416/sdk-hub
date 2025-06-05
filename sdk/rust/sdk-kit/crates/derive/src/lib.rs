#[proc_macro_derive(FieldNames, attributes(field_names))]
pub fn derive_field_names(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    mirrors_derive_internal::derive_field_names(input.into()).into()
}
