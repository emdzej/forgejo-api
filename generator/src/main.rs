use std::{ffi::OsString, path::PathBuf};

mod endpoints;
pub mod methods;
mod openapi;
mod structs;

use heck::{ToPascalCase, ToSnakeCase};
use openapi::*;

fn main() -> eyre::Result<()> {
    let spec = get_spec()?;
    let files = [
        (
            "mod.rs",
            "pub mod structs;\npub mod methods;\n#[cfg(feature = \"sync\")]\npub mod methods_sync;\npub mod endpoints;".into(),
        ),
        ("methods.rs", methods::create_methods(&spec, false)?),
        ("methods_sync.rs", methods::create_methods(&spec, true)?),
        ("structs.rs", structs::create_structs(&spec)?),
        ("endpoints.rs", endpoints::create_endpoints(&spec)?),
    ];
    save_generated(&files)?;
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

fn save_generated(files: &[(&'static str, String)]) -> eyre::Result<()> {
    let root_path = PathBuf::from(
        std::env::var_os("FORGEJO_API_GENERATED_PATH")
            .unwrap_or_else(|| OsString::from("./src/generated/")),
    );
    for (path, file) in files {
        let path = root_path.join(path);
        std::fs::create_dir_all(path.parent().ok_or_else(|| eyre::eyre!("no parent dir"))?)?;
        std::fs::write(&path, file)?;
        run_rustfmt_on(&path);
    }
    Ok(())
}

fn run_rustfmt_on(path: &std::path::Path) {
    let mut rustfmt = std::process::Command::new("rustfmt");

    rustfmt.arg(path);
    rustfmt.args(["--edition", "2021"]);

    if let Err(e) = rustfmt.status() {
        println!("Tried to format {path:?}, but failed to do so! :(");
        println!("Error:\n{e}");
    }
}

fn schema_ref_type_name(spec: &OpenApiV2, schema: &MaybeRef<Schema>) -> eyre::Result<String> {
    let name = if let MaybeRef::Ref { _ref } = schema {
        _ref.rsplit_once("/").map(|(_, b)| b)
    } else {
        None
    };
    let schema = schema.deref(spec)?;
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
                    Primitive::String if schema._enum.is_none() => match schema.format.as_deref() {
                        Some("date") => "time::Date",
                        Some("date-time") => "time::OffsetDateTime",
                        Some("url" | "ssh-url") => "url::Url",
                        _ => "String",
                    }
                    .to_string(),
                    Primitive::String => {
                        match (&schema.title, definition_name) {
                            // Some of the titles are actually descriptions; not sure why
                            // Checking for a space filters that out
                            (Some(title), _) if !title.contains(' ') => title.to_string(),
                            (_, Some(definition_name)) => definition_name.to_string(),
                            (_, None) => "String".to_string(),
                        }
                    }
                    Primitive::Number => match schema.format.as_deref() {
                        Some("float") => "f32",
                        Some("double") => "f64",
                        _ => "f64",
                    }
                    .to_string(),
                    Primitive::Integer => match schema.format.as_deref() {
                        Some("uint32") => "u32",
                        Some("uint64") => "u64",
                        Some("int32") => "i32",
                        Some("int64") => "i64",
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
                            (_, None) => "BTreeMap<String, serde_json::Value>".to_string(),
                        }
                    }
                    Primitive::File => "Vec<u8>".into(),
                };
                Ok(name.to_owned())
            }
            SchemaType::List(_) => todo!(),
        }
    } else {
        Ok("serde_json::Value".into())
    }
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

fn schema_subtype_name(
    spec: &OpenApiV2,
    parent_name: &str,
    name: &str,
    schema: &Schema,
    ty: &mut String,
) -> eyre::Result<bool> {
    let b = match schema {
        Schema {
            _type: Some(SchemaType::One(Primitive::Object)),
            properties: Some(_),
            ..
        }
        | Schema {
            _type: Some(SchemaType::One(Primitive::String)),
            _enum: Some(_),
            ..
        } => {
            *ty = format!("{parent_name}{}", name.to_pascal_case());
            true
        }
        Schema {
            _type: Some(SchemaType::One(Primitive::Object)),
            properties: None,
            additional_properties: Some(additional),
            ..
        } => {
            let additional = additional.deref(spec)?;
            let mut additional_ty = crate::schema_type_name(spec, None, additional)?;
            schema_subtype_name(spec, parent_name, name, additional, &mut additional_ty)?;
            *ty = format!("BTreeMap<String, {additional_ty}>");
            true
        }
        Schema {
            _type: Some(SchemaType::One(Primitive::Array)),
            items: Some(items),
            ..
        } => {
            if let MaybeRef::Value { value } = &**items {
                if schema_subtype_name(spec, parent_name, name, value, ty)? {
                    *ty = format!("Vec<{ty}>");
                    true
                } else {
                    false
                }
            } else {
                false
            }
        }
        _ => false,
    };
    Ok(b)
}
fn schema_subtypes(
    spec: &OpenApiV2,
    parent_name: &str,
    name: &str,
    schema: &Schema,
    subtypes: &mut Vec<String>,
) -> eyre::Result<()> {
    match schema {
        Schema {
            _type: Some(SchemaType::One(Primitive::Object)),
            properties: Some(_),
            ..
        } => {
            let name = format!("{parent_name}{}", name.to_pascal_case());
            let subtype = structs::create_struct_for_definition(spec, &name, schema)?;
            subtypes.push(subtype);
        }
        Schema {
            _type: Some(SchemaType::One(Primitive::String)),
            _enum: Some(_enum),
            ..
        } => {
            let name = format!("{parent_name}{}", name.to_pascal_case());
            let subtype = structs::create_enum(&name, schema.description.as_deref(), _enum, false)?;
            subtypes.push(subtype);
        }
        Schema {
            _type: Some(SchemaType::One(Primitive::Array)),
            items: Some(items),
            ..
        } => {
            if let MaybeRef::Value { value } = &**items {
                schema_subtypes(spec, parent_name, name, value, subtypes)?;
            }
        }
        _ => (),
    };
    Ok(())
}
