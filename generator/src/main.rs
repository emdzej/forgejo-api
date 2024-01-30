use std::ffi::{OsStr, OsString};

mod methods;
mod openapi;
mod structs;

use heck::ToSnakeCase;
use openapi::*;

fn main() -> eyre::Result<()> {
    let spec = get_spec()?;
    let mut s = String::new();
    s.push_str(&methods::create_methods(&spec)?);
    s.push_str(&structs::create_structs(&spec)?);
    save_generated(&s)?;
    Ok(())
}

fn get_spec() -> eyre::Result<OpenApiV2> {
    let path = std::env::var_os("FORGEJO_API_SPEC_PATH")
        .unwrap_or_else(|| OsString::from("./swagger.v1.json"));
    let file = std::fs::read(path)?;
    let spec = serde_json::from_slice::<OpenApiV2>(&file)?;
    spec.validate()?;
    Ok(spec)
}

fn save_generated(contents: &str) -> eyre::Result<()> {
    let path = std::env::var_os("FORGEJO_API_GENERATED_PATH")
        .unwrap_or_else(|| OsString::from("./src/generated.rs"));
    std::fs::write(path.as_os_str(), contents)?;
    run_rustfmt_on(path.as_os_str());
    Ok(())
}

fn run_rustfmt_on(path: &OsStr) {
    let mut rustfmt = std::process::Command::new("rustfmt");

    rustfmt.arg(path);
    rustfmt.args(["--edition", "2021"]);

    if let Err(e) = rustfmt.status() {
        println!("Tried to format {path:?}, but failed to do so! :(");
        println!("Error:\n{e}");
    }
}

fn schema_ref_type_name(spec: &OpenApiV2, schema: &MaybeRef<Schema>) -> eyre::Result<String> {
    let (name, schema) = deref_definition(spec, &schema)?;
    schema_type_name(spec, name, schema)
}

fn schema_type_name(
    spec: &OpenApiV2,
    definition_name: Option<&str>,
    schema: &Schema,
) -> eyre::Result<String> {
    if let Some(ty) = &schema._type {
        match ty {
            SchemaType::One(prim) => {
                let name = match prim {
                    Primitive::String => match schema.format.as_deref() {
                        Some("date") => "time::Date",
                        Some("date-time") => "time::OffsetDateTime",
                        _ => "String",
                    }
                    .to_string(),
                    Primitive::Number => match schema.format.as_deref() {
                        Some("float") => "f32",
                        Some("double") => "f64",
                        _ => "f64",
                    }
                    .to_string(),
                    Primitive::Integer => match schema.format.as_deref() {
                        Some("int32") => "u32",
                        Some("int64") => "u64",
                        _ => "u32",
                    }
                    .to_string(),
                    Primitive::Boolean => "bool".to_string(),
                    Primitive::Array => {
                        let item_name = match &schema.items {
                            Some(item_schema) => schema_ref_type_name(spec, item_schema)?,
                            None => "serde_json::Value".into(),
                        };
                        format!("Vec<{item_name}>")
                    }
                    Primitive::Null => "()".to_string(),
                    Primitive::Object => {
                        match (&schema.title, definition_name) {
                            // Some of the titles are actually descriptions; not sure why
                            // Checking for a space filters that out
                            (Some(title), _) if !title.contains(' ') => title.to_string(),
                            (_, Some(definition_name)) => definition_name.to_string(),
                            (_, None) => "serde_json::Map<String, serde_json::Value>".to_string(),
                        }
                    }
                };
                Ok(name.to_owned())
            }
            SchemaType::List(list) => todo!(),
        }
    } else {
        Ok("()".into())
    }
}

fn deref_definition<'a>(
    spec: &'a OpenApiV2,
    r: &'a MaybeRef<Schema>,
) -> eyre::Result<(Option<&'a str>, &'a Schema)> {
    let r = match r {
        MaybeRef::Value { value } => return Ok((None, value)),
        MaybeRef::Ref { _ref } => _ref,
    };
    let name = r
        .strip_prefix("#/definitions/")
        .ok_or_else(|| eyre::eyre!("invalid definition reference"))?;
    let global_definitions = spec
        .definitions
        .as_ref()
        .ok_or_else(|| eyre::eyre!("no global definitions"))?;
    let definition = global_definitions
        .get(name)
        .ok_or_else(|| eyre::eyre!("referenced definition does not exist"))?;
    Ok((Some(name), definition))
}

fn schema_is_string(spec: &OpenApiV2, schema: &MaybeRef<Schema>) -> eyre::Result<bool> {
    let (_, schema) = deref_definition(spec, schema)?;
    let is_str = match schema._type {
        Some(SchemaType::One(Primitive::String)) => true,
        _ => false,
    };
    Ok(is_str)
}

fn sanitize_ident(s: &str) -> String {
    let mut s = s.to_snake_case();
    let keywords = [
        "as",
        "break",
        "const",
        "continue",
        "crate",
        "else",
        "enum",
        "extern",
        "false",
        "fn",
        "for",
        "if",
        "impl",
        "in",
        "let",
        "loop",
        "match",
        "mod",
        "move",
        "mut",
        "pub",
        "ref",
        "return",
        "self",
        "Self",
        "static",
        "struct",
        "super",
        "trait",
        "true",
        "type",
        "unsafe",
        "use",
        "where",
        "while",
        "abstract",
        "become",
        "box",
        "do",
        "final",
        "macro",
        "override",
        "priv",
        "typeof",
        "unsized",
        "virtual",
        "yield",
        "async",
        "await",
        "dyn",
        "try",
        "macro_rules",
        "union",
    ];
    if s == "self" {
        s = "this".into();
    }
    if keywords.contains(&&*s) {
        s.insert_str(0, "r#");
    }
    s
}
