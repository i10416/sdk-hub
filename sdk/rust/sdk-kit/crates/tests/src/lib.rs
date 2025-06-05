#[cfg(test)]
mod tests {
    use mirrors::FieldNames;

    #[derive(FieldNames)]
    #[allow(unused)]
    struct Foo {
        f0: i32,
        #[field_names(rename = "f1")]
        f_1: i32,
        f_n: i32,
        r#type: i32,
    }
    #[derive(FieldNames)]
    struct Bar<'a> {
        #[allow(unused)]
        s: &'a str,
    }

    #[test]
    fn test_field_names_with_rename() {
        assert_eq!(Foo::field_names(), ["f0", "f1", "f_n", "type"]);
    }

    #[test]
    fn test_field_names_with_generics() {
        assert_eq!(Bar::field_names(), ["s"]);
    }
}
