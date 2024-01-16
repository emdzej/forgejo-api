use std::ffi::OsString;

mod openapi;

use eyre::Context;
use heck::{ToPascalCase, ToSnakeCase};
use openapi::{
    MaybeRef, OpenApiV2, Operation, Parameter, ParameterIn, ParameterType, Primitive, Response,
    Schema, SchemaType,
};
use std::fmt::Write;

fn main() -> eyre::Result<()> {
    let spec = get_spec()?;
    let mut s = String::new();
    s.push_str("use crate::ForgejoError;\n");
    s.push_str("impl Forgejo {\n");
    for (path, item) in &spec.paths {
        s.push_str(&create_methods_for_path(&spec, path, item).wrap_err_with(|| path.clone())?);
    }
    s.push('}');
    save_generated(&mut s)?;
    Ok(())
}

fn get_spec() -> eyre::Result<OpenApiV2> {
    let path = std::env::var_os("FORGEJO_API_SPEC_PATH")
        .unwrap_or_else(|| OsString::from("./swagger.v1.json"));
    let file = std::fs::read(path)?;
    let spec = serde_json::from_slice::<OpenApiV2>(&file)?;
    Ok(spec)
}

fn save_generated(contents: &str) -> eyre::Result<()> {
    let path = std::env::var_os("FORGEJO_API_GENERATED_PATH")
        .unwrap_or_else(|| OsString::from("./src/generated.rs"));
    std::fs::write(path, contents)?;
    Ok(())
}

fn create_methods_for_path(
    spec: &OpenApiV2,
    path: &str,
    item: &openapi::PathItem,
) -> eyre::Result<String> {
    let mut s = String::new();
    if let Some(op) = &item.get {
        s.push_str(&create_get_method(spec, path, op).wrap_err("GET")?);
    }
    if let Some(op) = &item.put {
        s.push_str(&create_put_method(spec, path, op).wrap_err("PUT")?);
    }
    if let Some(op) = &item.post {
        s.push_str(&create_post_method(spec, path, op).wrap_err("POST")?);
    }
    if let Some(op) = &item.delete {
        s.push_str(&create_delete_method(spec, path, op).wrap_err("DELETE")?);
    }
    if let Some(op) = &item.options {
        s.push_str(&create_options_method(spec, path, op).wrap_err("OPTIONS")?);
    }
    if let Some(op) = &item.head {
        s.push_str(&create_head_method(spec, path, op).wrap_err("HEAD")?);
    }
    if let Some(op) = &item.patch {
        s.push_str(&create_patch_method(spec, path, op).wrap_err("PATCH")?);
    }
    Ok(s)
}

fn fn_signature_from_op(spec: &OpenApiV2, op: &Operation) -> eyre::Result<String> {
    let name = op
        .operation_id
        .as_deref()
        .ok_or_else(|| eyre::eyre!("operation did not have id"))?
        .to_snake_case()
        .replace("o_auth2", "oauth2");
    let args = fn_args_from_op(spec, op)?;
    let ty = fn_return_from_op(spec, op)?;
    Ok(format!("fn {name}({args}) -> Result<{ty}, ForgejoError>"))
}

fn fn_args_from_op(spec: &OpenApiV2, op: &Operation) -> eyre::Result<String> {
    let mut args = "&self".to_string();
    let mut has_query = false;
    let mut has_headers = false;
    let mut has_form = false;
    if let Some(params) = &op.parameters {
        for param in params {
            let param = match &param {
                MaybeRef::Value { value } => value,
                MaybeRef::Ref { _ref } => eyre::bail!("todo: add deref parameters"),
            };
            match param._in {
                ParameterIn::Path => {
                    let type_name = path_param_type(&param)?;
                    args.push_str(", ");
                    args.push_str(&sanitize_ident(param.name.to_snake_case()));
                    args.push_str(": ");
                    args.push_str(type_name);
                }
                ParameterIn::Query => has_query = true,
                ParameterIn::Header => has_headers = true,
                ParameterIn::Body => {
                    let schema_ref = param.schema.as_ref().unwrap();
                    let ty = schema_ref_type_name(spec, &schema_ref)?;
                    args.push_str(", ");
                    args.push_str(&sanitize_ident(param.name.to_snake_case()));
                    args.push_str(": ");
                    args.push_str(&ty);
                }
                ParameterIn::FormData => {
                    args.push_str(", ");
                    args.push_str(&sanitize_ident(param.name.to_snake_case()));
                    args.push_str(": Vec<u8>");
                }
            }
        }
    }
    if has_query {
        let query_ty = query_struct_name(op)?;
        args.push_str(", query: ");
        args.push_str(&query_ty);
    }
    Ok(args)
}

fn query_struct_name(op: &Operation) -> eyre::Result<String> {
    let mut ty = op
        .operation_id
        .as_deref()
        .ok_or_else(|| eyre::eyre!("operation did not have id"))?
        .to_pascal_case()
        .replace("o_auth2", "oauth2");
    ty.push_str("Query");
    Ok(ty)
}

fn fn_return_from_op(spec: &OpenApiV2, op: &Operation) -> eyre::Result<String> {
    let mut names = op
        .responses
        .http_codes
        .iter()
        .filter(|(k, _)| k.starts_with("2"))
        .map(|(_, v)| response_ref_type_name(spec, v))
        .collect::<Result<Vec<_>, _>>()?;

    names.sort();
    names.dedup();
    let name = match names.len() {
        0 => eyre::bail!("no type name found"),
        1 => {
            let name = names.pop().unwrap();
            if name == "empty" {
                "()".into()
            } else {
                name
            }
        }
        2 if names[0] == "empty" || names[1] == "empty" => {
            let name = if names[0] == "empty" {
                names.remove(1)
            } else {
                names.remove(0)
            };
            format!("Option<{name}>")
        }
        _ => eyre::bail!("too many possible return types"),
    };

    Ok(name)
}

fn response_ref_type_name(spec: &OpenApiV2, schema: &MaybeRef<Response>) -> eyre::Result<String> {
    let (name, response) = deref_response(spec, schema)?;
    if let Some(schema) = &response.schema {
        schema_ref_type_name(spec, schema)
    } else if let Some(name) = name {
        Ok(name.into())
    } else {
        Ok("()".into())
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
                            (_, None) => "serde_json::Map".to_string(),
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

fn path_param_type(param: &Parameter) -> eyre::Result<&'static str> {
    let _type = param
        ._type
        .as_ref()
        .ok_or_else(|| eyre::eyre!("no type provided for path param"))?;
    let type_name = match _type {
        ParameterType::String => match param.format.as_deref() {
            Some("date") => "time::Date",
            Some("date-time") => "time::OffsetDateTime",
            _ => "&str",
        },
        ParameterType::Number => match param.format.as_deref() {
            Some("float") => "f32",
            Some("double") => "f64",
            _ => "f64",
        },
        ParameterType::Integer => match param.format.as_deref() {
            Some("int32") => "u32",
            Some("int64") => "u64",
            _ => "u32",
        },
        ParameterType::Boolean => "bool",
        ParameterType::Array => eyre::bail!("todo: support returning arrays"),
        ParameterType::File => "Vec<u8>",
    };
    Ok(type_name)
}

fn method_docs(op: &Operation) -> eyre::Result<String> {
    let mut out = String::new();
    let mut prev = false;
    if let Some(summary) = &op.summary {
        write!(&mut out, "/// {summary}\n")?;
        prev = true;
    }
    if let Some(params) = &op.parameters {
        if prev {
            out.push_str("///\n");
        }
        for param in params {
            let param = match &param {
                MaybeRef::Value { value } => value,
                MaybeRef::Ref { _ref } => eyre::bail!("pipis"),
            };
            match param._in {
                ParameterIn::Path | ParameterIn::Body | ParameterIn::FormData => {
                    write!(&mut out, "/// - `{}`", param.name)?;
                    if let Some(description) = &param.description {
                        write!(&mut out, ": {}", description)?;
                    }
                    writeln!(&mut out)?;
                }
                _ => (),
            }
        }
    }
    Ok(out)
}

fn deref_response<'a>(
    spec: &'a OpenApiV2,
    r: &'a MaybeRef<Response>,
) -> eyre::Result<(Option<&'a str>, &'a Response)> {
    let r = match r {
        MaybeRef::Value { value } => return Ok((None, value)),
        MaybeRef::Ref { _ref } => _ref,
    };
    let name = r
        .strip_prefix("#/responses/")
        .ok_or_else(|| eyre::eyre!("invalid response reference"))?;
    let global_responses = spec
        .responses
        .as_ref()
        .ok_or_else(|| eyre::eyre!("no global responses"))?;
    let response = global_responses
        .get(name)
        .ok_or_else(|| eyre::eyre!("referenced response does not exist"))?;
    Ok((Some(name), response))
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

fn create_method_body(
    spec: &OpenApiV2,
    method: &str,
    path: &str,
    op: &Operation,
) -> eyre::Result<String> {
    let request = create_method_request(spec, method, path, op)?;
    let response = create_method_response(spec, method, path, op)?;
    Ok(format!("{request}\n    {response}"))
}

fn create_method_request(
    spec: &OpenApiV2,
    method: &str,
    path: &str,
    op: &Operation,
) -> eyre::Result<String> {
    let mut has_query = false;
    let mut has_headers = false;
    let mut body_method = String::new();
    if let Some(params) = &op.parameters {
        for param in params {
            let param = match &param {
                MaybeRef::Value { value } => value,
                MaybeRef::Ref { _ref } => eyre::bail!("todo: add deref parameters"),
            };
            match param._in {
                ParameterIn::Path => (/* do nothing */),
                ParameterIn::Query => has_query = true,
                ParameterIn::Header => has_headers = true,
                ParameterIn::Body => {
                    if !body_method.is_empty() {
                        eyre::bail!("cannot have more than one body parameter");
                    }
                    if param_is_string(spec, param)? {
                        body_method = format!(".body({})", param.name);
                    } else {
                        body_method = format!(".json({})?", param.name);
                    }
                }
                ParameterIn::FormData => {
                    if !body_method.is_empty() {
                        eyre::bail!("cannot have more than one body parameter");
                    }
                    body_method = format!(".multipart(reqwest::multipart::Form::new().part(\"attachment\", reqwest::multipart::Part::bytes({}).file_name(\"file\").mime_str(\"*/*\").unwrap()))", param.name);
                }
            }
        }
    }
    let mut fmt_str = path.to_string();
    let mut fmt_args = String::new();
    if has_query {
        fmt_str.push_str("?{}");
        fmt_args.push_str(", query");
    }
    let path_arg = if fmt_str.contains("{") {
        format!("&format!(\"{fmt_str}\"{fmt_args})")
    } else {
        format!("\"{fmt_str}\"")
    };

    let out = format!("let request = self.{method}({path_arg}){body_method}.build()?;");
    Ok(out)
}

fn create_method_response(
    spec: &OpenApiV2,
    method: &str,
    path: &str,
    op: &Operation,
) -> eyre::Result<String> {
    let mut has_empty = false;
    let mut only_empty = true;
    for (_, res) in &op.responses.http_codes {
        let name = response_ref_type_name(spec, res)?;
        if name == "()" || name == "empty" {
            has_empty = true;
        } else {
            only_empty = false;
        }
    }
    let optional = has_empty && !only_empty;
    let mut out = String::new();
    out.push_str("let response = self.execute(request).await?;\n");
    out.push_str("match response.status().as_u16() {\n");
    for (code, res) in &op.responses.http_codes {
        let (_, res) = deref_response(spec, res)?;
        if code.starts_with("4") {
            continue;
        }
        out.push_str(code);
        out.push_str(" => ");
        let handler = match &res.schema {
            Some(schema) if schema_is_string(spec, schema)? => {
                if optional {
                    "Ok(Some(response.text().await?))"
                } else {
                    "Ok(response.text().await?)"
                }
            }
            Some(_) => {
                if optional {
                    "Ok(Some(response.json().await?))"
                } else {
                    "Ok(response.json().await?)"
                }
            }
            None => {
                if optional {
                    "Ok(None)"
                } else {
                    "Ok(())"
                }
            }
        };
        out.push_str(handler);
        out.push_str(",\n");
    }
    out.push_str("_ => Err(ForgejoError::UnexpectedStatusCode)\n");
    out.push_str("}\n");

    Ok(out)
}

fn schema_is_string(spec: &OpenApiV2, schema: &MaybeRef<Schema>) -> eyre::Result<bool> {
    let (_, schema) = deref_definition(spec, schema)?;
    let is_str = match schema._type {
        Some(SchemaType::One(Primitive::String)) => true,
        _ => false,
    };
    Ok(is_str)
}

fn param_is_string(spec: &OpenApiV2, param: &Parameter) -> eyre::Result<bool> {
    match param._in {
        ParameterIn::Body => {
            let schema_ref = param
                .schema
                .as_ref()
                .ok_or_else(|| eyre::eyre!("body param did not have schema"))?;
            schema_is_string(spec, schema_ref)
        }
        _ => {
            let is_str = match param._type {
                Some(ParameterType::String) => true,
                _ => false,
            };
            Ok(is_str)
        }
    }
}

fn create_get_method(spec: &OpenApiV2, path: &str, op: &Operation) -> eyre::Result<String> {
    let doc = method_docs(op)?;
    let sig = fn_signature_from_op(spec, op)?;
    let body = create_method_body(spec, "get", path, op)?;
    Ok(format!("{doc}{sig} {{\n    {body}\n}}\n\n"))
}

fn create_put_method(spec: &OpenApiV2, path: &str, op: &Operation) -> eyre::Result<String> {
    let doc = method_docs(op)?;
    let sig = fn_signature_from_op(spec, op)?;
    let body = create_method_body(spec, "put", path, op)?;
    Ok(format!("{doc}{sig} {{\n    {body}\n}}\n\n"))
}

fn create_post_method(spec: &OpenApiV2, path: &str, op: &Operation) -> eyre::Result<String> {
    let doc = method_docs(op)?;
    let sig = fn_signature_from_op(spec, op)?;
    let body = create_method_body(spec, "post", path, op)?;
    Ok(format!("{doc}{sig} {{\n    {body}\n}}\n\n"))
}

fn create_delete_method(spec: &OpenApiV2, path: &str, op: &Operation) -> eyre::Result<String> {
    let doc = method_docs(op)?;
    let sig = fn_signature_from_op(spec, op)?;
    let body = create_method_body(spec, "delete", path, op)?;
    Ok(format!("{doc}{sig} {{\n    {body}\n}}\n\n"))
}

fn create_options_method(spec: &OpenApiV2, path: &str, op: &Operation) -> eyre::Result<String> {
    let doc = method_docs(op)?;
    let sig = fn_signature_from_op(spec, op)?;
    let body = create_method_body(spec, "options", path, op)?;
    Ok(format!("{doc}{sig} {{\n    {body}\n}}\n\n"))
}

fn create_head_method(spec: &OpenApiV2, path: &str, op: &Operation) -> eyre::Result<String> {
    let doc = method_docs(op)?;
    let sig = fn_signature_from_op(spec, op)?;
    let body = create_method_body(spec, "head", path, op)?;
    Ok(format!("{doc}{sig} {{\n    {body}\n}}\n\n"))
}

fn create_patch_method(spec: &OpenApiV2, path: &str, op: &Operation) -> eyre::Result<String> {
    let doc = method_docs(op)?;
    let sig = fn_signature_from_op(spec, op)?;
    let body = create_method_body(spec, "patch", path, op)?;
    Ok(format!("{doc}{sig} {{\n    {body}\n}}\n\n"))
}

fn sanitize_ident(mut s: String) -> String {
    let keywords = [
        "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn",
        "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
        "return", "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe",
        "use", "where", "while",
    ];
    if keywords.contains(&&*s) {
        s.insert_str(0, "r#");
    }
    s
}
