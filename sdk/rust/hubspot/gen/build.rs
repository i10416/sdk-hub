use std::{fs::OpenOptions, io::Write, path::{Path, PathBuf}, str::FromStr};

use config::Config;
use serde::Deserialize;


type Error = Box<dyn std::error::Error + Send + Sync>;
type BuildResult<T> = Result<T, Error>;

#[derive(Debug, Deserialize)]
struct Settings {}

fn main() {
    let settings = Config::builder()
      .add_source(config::File::with_name("codegen").required(true)).build().expect("OK");
    let src = "./simple.json";
    let src = Path::new(src);
    println!("cargo:rerun-if-changed={}", src.to_str().expect("Invalid Path"));
    let spec = load_spec(src).expect("Unable to load spec");
    let mut generator = progenitor::Generator::default();
    let tokens = generator.generate_tokens(&spec).unwrap();
    let ast = syn::parse2(tokens).unwrap();
    let content = prettyplease::unparse(&ast);
    let path = std::env::var("OUT_DIR").unwrap();
    let mut path = PathBuf::from_str(&path).unwrap();
    let _ = std::fs::create_dir_all(path.parent().unwrap());
    path.push("simple.rs");
    let _ = std::fs::write(path, content).unwrap();
}


fn load_spec(src: &Path) -> BuildResult<openapiv3::OpenAPI> {
    let file = std::fs::File::open(src).unwrap();
    let spec = serde_json::from_reader(file)?;
    Ok(spec)
}