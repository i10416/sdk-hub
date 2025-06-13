use syn::{
    AngleBracketedGenericArguments, GenericArgument, PathArguments, TypePath, TypeReference,
};

pub fn from_rust_type_to_scala_type(t: &syn::Type) -> String {
    let best_effort_scala_type = match t {
        syn::Type::Path(TypePath { path, .. }) if path.segments.len() == 1 => {
            let segment = path.segments.first().unwrap();
            match segment.ident.to_string().as_str() {
                "str" | "String" => "String".to_string(),
                "bool" => "Boolean".to_string(),
                "i64" => "Long".to_string(),
                "i32" => "Int".to_string(),
                "usize" => "Int".to_string(),
                "Vec" => match &segment.arguments {
                    PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                        args, ..
                    }) if args.len() == 1 => {
                        let it = match args.first().unwrap() {
                            GenericArgument::Type(t) => {
                                format!("List[{}]", from_rust_type_to_scala_type(t))
                            }
                            _ => panic!("Unexpected GenericArgument for Vec"),
                        };
                        it
                    }
                    _ => panic!("Vec MUST take exact one type argument"),
                },
                "Option" => match &segment.arguments {
                    PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                        args, ..
                    }) if args.len() == 1 => match args.first().unwrap() {
                        GenericArgument::Type(t) => {
                            format!("Option[{}]", from_rust_type_to_scala_type(t))
                        }
                        _ => panic!("Unexpected GenericArgument for Option"),
                    },
                    _ => panic!("Option MUST take exact one type argument"),
                },
                t => t.to_string(),
            }
        }
        syn::Type::Path(TypePath { path, .. }) => path
            .segments
            .iter()
            .map(|seg| seg.ident.to_string())
            .collect::<Vec<_>>()
            .join("."),
        syn::Type::Reference(TypeReference { elem, .. }) => {
            // ignore reference type because there's no such thing as pointer in JVM world(at least superficially)
            from_rust_type_to_scala_type(elem)
        }
        _ => "Any".to_string(),
    };
    best_effort_scala_type
}
