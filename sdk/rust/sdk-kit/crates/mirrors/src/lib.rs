#![doc = include_str!("../../../README.md")]
/// Derive `FieldNames` trait to extract struct field names as a slice.
///
///
/// ```
///use mirrors::FieldNames;
///
///#[derive(FieldNames)]
///struct Foo {
///    f0: i32,
///    #[field_names(rename = "f1")]
///    f_1: i32,
///    f_n: i32,
/// }
///
/// assert_eq!(Foo::field_names(),["f0","f1","f_n"])
/// ```
///
pub use mirrors_derive::FieldNames;

pub trait FieldNames {
    fn field_names() -> &'static [&'static str];
}
