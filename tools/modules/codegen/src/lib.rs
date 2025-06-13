mod convert;
mod extract;
mod pseudoscala;

use std::{
    fs::{File, OpenOptions},
    io::{BufWriter, Write},
    path::Path,
};

use syn::{
    punctuated::Punctuated,
    visit::{self, Visit},
    Attribute, Meta, Token,
};

use crate::{
    convert::from_rust_type_to_scala_type,
    extract::extract_semantics,
    pseudoscala::{Class, RenderingContext, ValueEnum, ValueEnumVariant},
};

pub fn generate(input_path: &str, output_path: &str) {
    let mut file = std::fs::read_to_string(input_path).unwrap();
    let ast = syn::parse_file(&mut file).unwrap();
    let mut inst = V::from_path(Path::new(output_path)).unwrap();
    inst.visit_file(&ast);
}

struct V<T: Write> {
    out: BufWriter<T>,
}

impl V<File> {
    pub fn from_path(out_path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let f = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(out_path)?;
        Ok(Self::new(f))
    }
}
impl<T: Write> V<T> {
    pub fn new(t: T) -> Self {
        Self {
            out: BufWriter::new(t),
        }
    }
}

impl<'ast, T: Write> Visit<'ast> for V<T> {
    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        let defn = from_struct_to_case_class(i);
        self.out.write(defn.as_bytes()).expect("OK");
        visit::visit_item_struct(self, i);
    }
    fn visit_item_enum(&mut self, i: &'ast syn::ItemEnum) {
        if is_value_enum(i) {
            let defn = transform_value_enum(i);
            if let Some(defn) = defn {
                self.out.write(defn.as_bytes()).expect("OK");
            }
        }
        visit::visit_item_enum(self, i);
    }
}

fn is_value_enum(e: &syn::ItemEnum) -> bool {
    e.variants.iter().all(|variant| variant.fields.is_empty())
}

fn transform_value_enum(e: &syn::ItemEnum) -> Option<String> {
    let variants: Vec<_> = e
        .variants
        .iter()
        .map(|v| {
            let mut semantics = FieldSemantics::new(v.ident.to_string().as_str());
            let naming = extract_semantics(&mut semantics, &v.attrs);
            ValueEnumVariant {
                semantics: naming.clone(),
            }
        })
        .collect::<Vec<_>>();
    let cls = ValueEnum {
        ident: e.ident.to_string(),
        variants,
    };
    let mut buf = String::new();
    let mut ctx = RenderingContext::new(&mut buf);
    cls.render(&mut ctx);
    cls.companion().render(&mut ctx);
    Some(buf)
}

pub fn has_derive(attrs: &[Attribute], trait_name: &str) -> bool {
    attrs
        .iter()
        .filter(|attr| attr.meta.path().is_ident("derive"))
        .any(|attr| {
            let nested = attr
                .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                .unwrap();
            nested.iter().any(|meta| meta.path().is_ident(trait_name))
        })
}

#[derive(Clone)]
struct FieldSemantics {
    rename: Option<String>,
    alias: Option<String>,
    ident_name: String,
    fill_with_default_value: bool,
}

impl FieldSemantics {
    pub fn new(ident_name: &str) -> Self {
        Self {
            ident_name: ident_name.to_string(),
            rename: None,
            alias: None,
            fill_with_default_value: false,
        }
    }
    pub fn serde_name(&self) -> String {
        self.rename
            .as_deref()
            .map(str::to_string)
            .unwrap_or(self.ident_name.to_string())
    }
}

fn from_struct_to_case_class<'a>(data: &'a syn::ItemStruct) -> String {
    let fields = data
        .fields
        .iter()
        .map(|field| {
            let mut semantics = FieldSemantics::new(
                field
                    .ident
                    .as_ref()
                    .expect("Ident must be present")
                    .to_string()
                    .as_str(),
            );
            let semantics = extract_semantics(&mut semantics, &field.attrs);
            let field_type = from_rust_type_to_scala_type(&field.ty);
            (semantics.clone(), field_type)
        })
        .collect::<Vec<_>>();
    let cls = Class {
        ident: data.ident.to_string(),
        modifiers: vec!["final".to_string(), "case".to_string()],
        fields,
        derives: vec![],
    };
    let mut buf = String::new();
    let mut ctx = RenderingContext::new(&mut buf);
    cls.render(&mut ctx);
    cls.companion().render(&mut ctx);
    buf
}

#[cfg(test)]
mod tests {
    use crate::generate;

    #[test]
    fn sanity_check_codegen() {
        let path = "res/sample.rs";
        generate(path, "schema.scala");
    }
}
