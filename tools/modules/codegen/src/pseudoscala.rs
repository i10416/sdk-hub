use std::fmt::{Error, Write};

use crate::FieldSemantics;

pub struct RenderingContext<'a, T: Write> {
    level: usize,
    w: &'a mut T,
}

impl<'a, T: Write> RenderingContext<'a, T> {
    pub fn new(w: &'a mut T) -> Self {
        Self { level: 0, w }
    }

    fn indent(&self) -> String {
        (0..self.level).map(|_| "  ").collect::<String>()
    }
    pub fn start_block_with(&mut self, line: &str, open_mark: &str) -> Result<(), Error> {
        let _ = self.w.write_str(&self.indent())?;
        let _ = self.w.write_str(line.trim_end())?;
        let _ = self.w.write_str(open_mark)?;
        let _ = self.w.write_str("\n")?;
        Ok(self.level += 1)
    }

    pub fn add_line(&mut self, line: &str) -> Result<(), Error> {
        let _ = self.w.write_str(&self.indent())?;
        let _ = self.w.write_str(line.trim_end())?;
        let _ = self.w.write_str("\n")?;
        Ok(())
    }

    pub fn end_block_with(&mut self, end_block: &str) -> Result<(), Error> {
        self.level -= 1;
        let _ = self.w.write_str(&self.indent())?;
        let _ = self.w.write_str(end_block)?;
        let _ = self.w.write_str("\n")?;
        Ok(())
    }
}

enum Companion {
    Product(Class),
    ValueEnum(ValueEnum),
}

pub struct ValueEnum {
    pub ident: String,
    pub variants: Vec<ValueEnumVariant>,
}
impl ValueEnum {
    pub fn companion(self) -> Object {
        Object {
            name: self.ident.to_string(),
            modifiers: vec![],
            extends: None,
            companion: Some(Companion::ValueEnum(self)),
        }
    }
}
pub struct ValueEnumVariant {
    pub semantics: FieldSemantics,
}

impl ValueEnum {
    pub fn render<T: Write>(&self, ctx: &mut RenderingContext<T>) {
        ctx.start_block_with(&format!("sealed trait {}", self.ident), ":")
            .expect("OK");
        ctx.add_line("def value: String").expect("OK");
        ctx.end_block_with("").expect("OK");
    }
}

pub struct Class {
    pub ident: String,
    pub modifiers: Vec<String>,
    pub fields: Vec<(FieldSemantics, String)>,
    pub derives: Vec<String>,
}
impl Class {
    pub fn companion(self) -> Object {
        Object {
            name: self.ident.to_string(),
            modifiers: vec![],
            extends: None,
            companion: Some(Companion::Product(self)),
        }
    }
}

pub struct Object {
    pub name: String,
    pub modifiers: Vec<String>,
    pub extends: Option<String>,
    companion: Option<Companion>,
}
impl Object {
    pub fn render<T: Write>(&self, ctx: &mut RenderingContext<T>) {
        let modifiers = if self.modifiers.is_empty() {
            "".to_string()
        } else {
            format!("{} ", self.modifiers.join(" "))
        };
        let extends = match &self.extends {
            Some(ext) => format!(" extends {ext}"),
            None => "".to_string(),
        };

        let _ = ctx
            .start_block_with(&format!("{modifiers}object {}{}", self.name, extends), ":")
            .expect("OK");

        if let Some(Companion::Product(cls)) = &self.companion {
            let n = cls.fields.len();
            let field_types = cls
                .fields
                .iter()
                .map(|(_, t)| t.to_string())
                .collect::<Vec<_>>()
                .join(",");
            let field_names = cls
                .fields
                .iter()
                .map(|(semantics, _)| format!("\"{}\"", semantics.serde_name()))
                .collect::<Vec<_>>()
                .join(",");
            let field_selects = cls
                .fields
                .iter()
                .map(|(semantics, _)| format!("p.{}", semantics.ident_name))
                .collect::<Vec<_>>()
                .join(",");
            let dec = format!("implicit val de: Decoder[{ident}] = Decoder.forProduct{n}[{ident},{field_types}]({field_names})({ident}.apply)",ident = cls.ident,n = n,field_types = field_types,field_names = field_names);
            let enc = format!("implicit val en: Encoder[{ident}] = Encoder.forProduct{n}[{ident},{field_types}]({field_names})(p => ({field_selects}))",ident = cls.ident,n = n,field_types = field_types,field_names = field_names,field_selects = field_selects);
            ctx.add_line(&dec).expect("OK");
            ctx.add_line(&enc).expect("OK");
        };
        if let Some(Companion::ValueEnum(value_enum)) = &self.companion {
            ctx.add_line(&format!(
                "implicit val en: Encoder[{}] = Encoder[String].contramap[{}](_.value)",
                value_enum.ident, value_enum.ident
            ))
            .expect("OK");
            let decls = value_enum.variants.iter().map(|v| v).collect::<Vec<_>>();
            for decl in decls {
                ctx.start_block_with(
                    &format!(
                        "case object {} extends {}",
                        decl.semantics.ident_name, value_enum.ident
                    ),
                    ":",
                )
                .expect("OK");
                ctx.add_line(&format!(
                    "def value: String = \"{}\"",
                    decl.semantics.serde_name()
                ))
                .expect("OK");
                ctx.end_block_with("").expect("OK");
            }
        }
        ctx.end_block_with("").expect("OK");
    }
}

impl Class {
    pub fn render<T: Write>(&self, ctx: &mut RenderingContext<T>) {
        let modifiers = if self.modifiers.is_empty() {
            "".to_string()
        } else {
            format!("{} ", self.modifiers.join(" "))
        };
        let derives = if self.derives.is_empty() {
            "".to_string()
        } else {
            format!(" derives {}", self.derives.join(","))
        };

        let _ = ctx
            .start_block_with(&format!("{modifiers}class {}", self.ident), "(")
            .expect("OK");
        for (semantics, ty) in self.fields.iter() {
            let maybe_default = if semantics.fill_with_default_value {
                match default_value_for_type(ty) {
                    Some(v) => format!(" = {v}"),
                    None => "".to_string(),
                }
            } else {
                "".to_string()
            };
            let line = format!("{}: {}{},", semantics.ident_name, ty, maybe_default);
            ctx.add_line(&line).expect("OK");
        }
        ctx.end_block_with(&format!("){}", derives)).expect("OK");
    }
}

fn default_value_for_type(t: &str) -> Option<String> {
    if t.starts_with("Option[") {
        Some("None".to_string())
    } else if t.starts_with("List[") {
        Some("Nil".to_string())
    } else {
        None
    }
}
