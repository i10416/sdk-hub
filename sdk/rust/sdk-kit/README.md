# mirrors

## About

Extract product types and sum types information using reflection.

## Install

```toml
mirrors = {git = "ssh://git@github.com/i10416/mirrors.git", branch = "main", version = "0.1"}
```

## Example


```rust
use mirrors::FieldNames;

#[derive(FieldNames)]
struct Foo {
    f0: i32,
    #[field_names(rename = "f1")]
    f_1: i32,
    f_n: i32,
 }

 assert_eq!(Foo::field_names(),["f0","f1","f_n"])
```

