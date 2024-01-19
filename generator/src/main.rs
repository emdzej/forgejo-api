use std::ffi::OsString;

mod openapi;

use eyre::Context;
use heck::{ToPascalCase, ToSnakeCase};
use openapi::{
    CollectionFormat, Items, MaybeRef, OpenApiV2, Operation, Parameter, ParameterIn, ParameterType,
    Primitive, Response, Schema, SchemaType,
};
use std::fmt::Write;

fn main() -> eyre::Result<()> {
    let spec = get_spec()?;
    let mut s = String::new();
    s.push_str("use crate::ForgejoError;\n");
    s.push_str("impl crate::Forgejo {\n");
    for (path, item) in &spec.paths {
        s.push_str(&create_methods_for_path(&spec, path, item).wrap_err_with(|| path.clone())?);
    }
    s.push_str("}\n");

    s.push_str("use structs::*;\n");
    s.push_str("pub mod structs {\n");
    s.push_str("use std::fmt::Write;\n");
    if let Some(definitions) = &spec.definitions {
        for (name, schema) in definitions {
            let strukt = create_struct_for_definition(&spec, name, schema)?;
            s.push_str(&strukt);
        }
    }
    for (path, item) in &spec.paths {
        let strukt = create_query_structs_for_path(&spec, path, item)?;
        s.push_str(&strukt);
    }
    s.push_str("\n}");
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
    Ok(format!(
        "pub async fn {name}({args}) -> Result<{ty}, ForgejoError>"
    ))
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
                    let type_name = param_type(&param, false)?;
                    args.push_str(", ");
                    args.push_str(&sanitize_ident(&param.name));
                    args.push_str(": ");
                    args.push_str(&type_name);
                }
                ParameterIn::Query => has_query = true,
                ParameterIn::Header => has_headers = true,
                ParameterIn::Body => {
                    let schema_ref = param.schema.as_ref().unwrap();
                    let ty = schema_ref_type_name(spec, &schema_ref)?;
                    args.push_str(", ");
                    args.push_str(&sanitize_ident(&param.name));
                    args.push_str(": ");
                    args.push_str(&ty);
                }
                ParameterIn::FormData => {
                    args.push_str(", ");
                    args.push_str(&sanitize_ident(&param.name));
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

fn fn_return_from_op(spec: &OpenApiV2, op: &Operation) -> eyre::Result<ResponseType> {
    let mut responses = op
        .responses
        .http_codes
        .iter()
        .filter(|(k, _)| k.starts_with("2"))
        .map(|(_, v)| response_ref_type_name(spec, v))
        .collect::<Result<Vec<_>, _>>()?;
    let mut iter = responses.into_iter();
    let mut response = iter
        .next()
        .ok_or_else(|| eyre::eyre!("must have at least one response type"))?;
    for next in iter {
        response = response.merge(next)?;
    }

    Ok(response)
}

#[derive(Debug, Default)]
struct ResponseType {
    headers: Option<String>,
    body: Option<String>,
}

impl ResponseType {
    fn merge(self, other: Self) -> eyre::Result<Self> {
        let mut new = Self::default();
        match (self.headers, other.headers) {
            (Some(a), Some(b)) if a != b => eyre::bail!("incompatible header types in response"),
            (Some(a), None) => new.headers = Some(format!("Option<{a}>")),
            (None, Some(b)) => new.headers = Some(format!("Option<{b}>")),
            (a, b) => new.headers = a.or(b),
        };
        match (self.body.as_deref(), other.body.as_deref()) {
            (Some(a), Some(b)) if a != b => eyre::bail!("incompatible header types in response"),
            (Some(a), Some("()") | None) => new.body = Some(format!("Option<{a}>")),
            (Some("()") | None, Some(b)) => new.body = Some(format!("Option<{b}>")),
            (a, b) => new.body = self.body.or(other.body),
        };
        Ok(new)
    }
}

impl std::fmt::Display for ResponseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut tys = Vec::new();
        tys.extend(self.headers.as_deref());
        tys.extend(self.body.as_deref());
        match tys[..] {
            [single] => f.write_str(single),
            _ => {
                write!(f, "(")?;
                for (i, item) in tys.iter().copied().enumerate() {
                    f.write_str(item)?;
                    if i + 1 < tys.len() {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")")?;
                Ok(())
            }
        }
    }
}

fn response_ref_type_name(
    spec: &OpenApiV2,
    schema: &MaybeRef<Response>,
) -> eyre::Result<ResponseType> {
    let (_, response) = deref_response(spec, schema)?;
    let mut ty = ResponseType::default();
    if response.headers.is_some() {
        ty.headers = Some("reqwest::header::HeaderMap".into());
    }
    if let Some(schema) = &response.schema {
        ty.body = Some(schema_ref_type_name(spec, schema)?);
    };
    Ok(ty)
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

fn param_type(param: &Parameter, owned: bool) -> eyre::Result<String> {
    let _type = param
        ._type
        .as_ref()
        .ok_or_else(|| eyre::eyre!("no type provided for path param"))?;
    param_type_inner(_type, param.format.as_deref(), param.items.as_ref(), owned)
}

fn param_type_inner(
    ty: &ParameterType,
    format: Option<&str>,
    items: Option<&Items>,
    owned: bool,
) -> eyre::Result<String> {
    let ty_name = match ty {
        ParameterType::String => match format.as_deref() {
            Some("date") => "time::Date",
            Some("date-time") => "time::OffsetDateTime",
            _ => {
                if owned {
                    "String"
                } else {
                    "&str"
                }
            }
        }
        .into(),
        ParameterType::Number => match format.as_deref() {
            Some("float") => "f32",
            Some("double") => "f64",
            _ => "f64",
        }
        .into(),
        ParameterType::Integer => match format.as_deref() {
            Some("int32") => "u32",
            Some("int64") => "u64",
            _ => "u32",
        }
        .into(),
        ParameterType::Boolean => "bool".into(),
        ParameterType::Array => {
            let item = items
                .as_ref()
                .ok_or_else(|| eyre::eyre!("array must have item type defined"))?;
            let item_ty_name = param_type_inner(
                &item._type,
                item.format.as_deref(),
                item.items.as_deref(),
                owned,
            )?;
            if owned {
                format!("Vec<{item_ty_name}>")
            } else {
                format!("&[{item_ty_name}]")
            }
        }
        ParameterType::File => {
            if owned {
                format!("Vec<u8>")
            } else {
                format!("&[u8]")
            }
        }
    };
    Ok(ty_name)
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
            let name = sanitize_ident(&param.name);
            match param._in {
                ParameterIn::Path => (/* do nothing */),
                ParameterIn::Query => has_query = true,
                ParameterIn::Header => has_headers = true,
                ParameterIn::Body => {
                    if !body_method.is_empty() {
                        eyre::bail!("cannot have more than one body parameter");
                    }
                    if param_is_string(spec, param)? {
                        body_method = format!(".body({name})");
                    } else {
                        body_method = format!(".json(&{name})");
                    }
                }
                ParameterIn::FormData => {
                    if !body_method.is_empty() {
                        eyre::bail!("cannot have more than one body parameter");
                    }
                    body_method = format!(".multipart(reqwest::multipart::Form::new().part(\"attachment\", reqwest::multipart::Part::bytes({name}).file_name(\"file\").mime_str(\"*/*\").unwrap()))");
                }
            }
        }
    }
    let mut fmt_str = sanitize_path_arg(path)?;
    let mut fmt_args = String::new();
    if has_query {
        fmt_str.push_str("?{}");
        fmt_args.push_str(", query.to_string()");
    }
    let path_arg = if fmt_str.contains("{") {
        format!("&format!(\"{fmt_str}\"{fmt_args})")
    } else {
        format!("\"{fmt_str}\"")
    };

    let out = format!("let request = self.{method}({path_arg}){body_method}.build()?;");
    Ok(out)
}

fn sanitize_path_arg(mut path: &str) -> eyre::Result<String> {
    let mut out = String::new();
    loop {
        let (head, tail) = match path.split_once("{") {
            Some(i) => i,
            None => {
                out.push_str(path);
                break;
            }
        };
        path = tail;
        out.push_str(head);
        out.push('{');
        let (head, tail) = match path.split_once("}") {
            Some(i) => i,
            None => {
                eyre::bail!("unmatched bracket");
            }
        };
        path = tail;
        out.push_str(&head.to_snake_case());
        out.push('}');
    }
    if out.starts_with("/") {
        out.remove(0);
    }
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
    for (code, res) in &op.responses.http_codes {
        let response = response_ref_type_name(spec, res)?;
        if !code.starts_with("2") {
            continue;
        }
        if matches!(response.body.as_deref(), Some("()") | None) {
            has_empty = true;
        } else {
            only_empty = false;
        }
    }
    let fn_ret = fn_return_from_op(spec, op)?;
    let optional = has_empty && !only_empty;
    let mut out = String::new();
    out.push_str("let response = self.execute(request).await?;\n");
    out.push_str("match response.status().as_u16() {\n");
    for (code, res) in &op.responses.http_codes {
        let (_, res) = deref_response(spec, res)?;
        if !code.starts_with("2") {
            continue;
        }
        out.push_str(code);
        out.push_str(" => Ok(");
        let mut handlers = Vec::new();
        let header_handler = match &res.headers {
            Some(_) => {
                if fn_ret
                    .headers
                    .as_ref()
                    .map(|s| s.starts_with("Option<"))
                    .unwrap()
                {
                    Some("Some(response.headers().clone())")
                } else {
                    Some("response.headers().clone()")
                }
            }
            None => {
                if fn_ret.headers.is_some() {
                    dbg!(&fn_ret);
                    panic!();
                    Some("None")
                } else {
                    None
                }
            }
        };
        handlers.extend(header_handler);
        let body_handler = match &res.schema {
            Some(schema) if schema_is_string(spec, schema)? => {
                if optional {
                    Some("Some(response.text().await?)")
                } else {
                    Some("response.text().await?")
                }
            }
            Some(_) => {
                if optional {
                    Some("Some(response.json().await?)")
                } else {
                    Some("response.json().await?")
                }
            }
            None => {
                if optional {
                    Some("None")
                } else {
                    None
                }
            }
        };
        handlers.extend(body_handler);
        match handlers[..] {
            [single] => out.push_str(single),
            _ => {
                out.push('(');
                for (i, item) in handlers.iter().copied().enumerate() {
                    out.push_str(item);
                    if i + 1 < handlers.len() {
                        out.push_str(", ");
                    }
                }
                out.push(')');
            }
        }
        out.push_str("),\n");
    }
    out.push_str("_ => Err(ForgejoError::UnexpectedStatusCode(response.status()))\n");
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

fn create_struct_for_definition(
    spec: &OpenApiV2,
    name: &str,
    schema: &Schema,
) -> eyre::Result<String> {
    if matches!(schema._type, Some(SchemaType::One(Primitive::Array))) {
        return Ok(String::new());
    }

    let docs = create_struct_docs(schema)?;
    let mut fields = String::new();
    let required = schema.required.as_deref().unwrap_or_default();
    if let Some(properties) = &schema.properties {
        for (prop_name, prop_schema) in properties {
            let prop_ty = schema_ref_type_name(spec, prop_schema)?;
            let field_name = sanitize_ident(prop_name);
            let mut field_ty = prop_ty.clone();
            if field_name.ends_with("url") && field_name != "ssh_url" && field_ty == "String" {
                field_ty = "url::Url".into()
            }
            if field_ty == name {
                field_ty = format!("Box<{field_ty}>")
            }
            if !required.contains(prop_name) {
                field_ty = format!("Option<{field_ty}>")
            }
            if field_ty == "Option<url::Url>" {
                fields.push_str("#[serde(deserialize_with = \"crate::none_if_blank_url\")]\n");
            }
            if field_ty == "time::OffsetDateTime" {
                fields.push_str("#[serde(with = \"time::serde::rfc3339\")]\n");
            }
            if field_ty == "Option<time::OffsetDateTime>" {
                fields.push_str("#[serde(with = \"time::serde::rfc3339::option\")]\n");
            }
            if &field_name != prop_name {
                fields.push_str("#[serde(rename = \"");
                fields.push_str(prop_name);
                fields.push_str("\")]\n");
            }
            fields.push_str("pub ");
            fields.push_str(&field_name);
            fields.push_str(": ");
            fields.push_str(&field_ty);
            fields.push_str(",\n");
        }
    }

    if let Some(additonal_schema) = &schema.additional_properties {
        let prop_ty = schema_ref_type_name(spec, additonal_schema)?;
        fields.push_str("#[serde(flatten)]\n");
        fields.push_str("pub additional: std::collections::BTreeMap<String, ");
        fields.push_str(&prop_ty);
        fields.push_str(">,\n");
    }

    let out = format!("{docs}#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]\npub struct {name} {{\n{fields}}}\n\n");
    Ok(out)
}

fn create_struct_docs(schema: &Schema) -> eyre::Result<String> {
    let doc = match &schema.description {
        Some(desc) => {
            let mut out = String::new();
            for line in desc.lines() {
                out.push_str("/// ");
                out.push_str(line);
                out.push_str("\n/// \n");
            }
            out
        }
        None => String::new(),
    };
    Ok(doc)
}

fn create_query_structs_for_path(
    spec: &OpenApiV2,
    path: &str,
    item: &openapi::PathItem,
) -> eyre::Result<String> {
    let mut s = String::new();
    if let Some(op) = &item.get {
        s.push_str(&create_query_struct(spec, path, op).wrap_err("GET")?);
    }
    if let Some(op) = &item.put {
        s.push_str(&create_query_struct(spec, path, op).wrap_err("PUT")?);
    }
    if let Some(op) = &item.post {
        s.push_str(&create_query_struct(spec, path, op).wrap_err("POST")?);
    }
    if let Some(op) = &item.delete {
        s.push_str(&create_query_struct(spec, path, op).wrap_err("DELETE")?);
    }
    if let Some(op) = &item.options {
        s.push_str(&create_query_struct(spec, path, op).wrap_err("OPTIONS")?);
    }
    if let Some(op) = &item.head {
        s.push_str(&create_query_struct(spec, path, op).wrap_err("HEAD")?);
    }
    if let Some(op) = &item.patch {
        s.push_str(&create_query_struct(spec, path, op).wrap_err("PATCH")?);
    }
    Ok(s)
}

fn create_query_struct(spec: &OpenApiV2, path: &str, op: &Operation) -> eyre::Result<String> {
    let params = match &op.parameters {
        Some(params) => params,
        None => return Ok(String::new()),
    };

    let mut fields = String::new();
    let mut imp = String::new();
    imp.push_str("let mut s = String::new();\n");
    for param in params {
        let param = match &param {
            MaybeRef::Value { value } => value,
            MaybeRef::Ref { _ref } => eyre::bail!("todo: add deref parameters"),
        };
        if param._in == ParameterIn::Query {
            let ty = param_type(param, true)?;
            let field_name = sanitize_ident(&param.name);
            let required = param.required.unwrap_or_default();
            fields.push_str("pub ");
            fields.push_str(&field_name);
            fields.push_str(": ");
            if required {
                fields.push_str(&ty);
            } else {
                fields.push_str("Option<");
                fields.push_str(&ty);
                fields.push_str(">");
            }
            fields.push_str(",\n");

            let mut handler = String::new();
            let ty = param
                ._type
                .as_ref()
                .ok_or_else(|| eyre::eyre!("no type provided for query field"))?;
            if required {
                writeln!(&mut handler, "let {field_name} = self.{field_name};")?;
            } else {
                writeln!(
                    &mut handler,
                    "if let Some({field_name}) = self.{field_name} {{"
                )?;
            }
            match ty {
                ParameterType::String => match param.format.as_deref() {
                    Some("date-time" | "date") => {
                        writeln!(&mut handler, "s.push_str(\"{}=\");", param.name)?;
                        writeln!(&mut handler, "s.push_str(&{field_name}.format(&time::format_description::well_known::Rfc3339).unwrap());")?;
                        writeln!(&mut handler, "s.push('&');")?;
                    }
                    _ => {
                        writeln!(&mut handler, "s.push_str(\"{}=\");", param.name)?;
                        writeln!(&mut handler, "s.push_str(&{field_name});")?;
                        writeln!(&mut handler, "s.push('&');")?;
                    }
                },
                ParameterType::Number | ParameterType::Integer | ParameterType::Boolean => {
                    writeln!(
                        &mut handler,
                        "write!(&mut s, \"{}={{}}&\", {field_name}).unwrap();",
                        param.name
                    )?;
                }
                ParameterType::Array => {
                    let format = param.collection_format.unwrap_or(CollectionFormat::Csv);
                    let item = param
                        .items
                        .as_ref()
                        .ok_or_else(|| eyre::eyre!("array must have item type defined"))?;
                    let item_pusher = match item._type {
                        ParameterType::String => {
                            match param.format.as_deref() {
                                Some("date-time" | "date") => {
                                    "s.push_str(&item.format(&time::format_description::well_known::Rfc3339).unwrap());"
                                },
                                _ => {
                                    "s.push_str(&item);"
                                }
                            }
                        },
                        ParameterType::Number |
                            ParameterType::Integer |
                            ParameterType::Boolean => {
                                "write!(&mut s, \"{item}\").unwrap();"
                            },
                        ParameterType::Array => {
                            eyre::bail!("nested arrays not supported in query");
                        },
                        ParameterType::File => eyre::bail!("cannot send file in query"),
                    };
                    match format {
                        CollectionFormat::Csv => {
                            handler.push_str(&simple_query_array(
                                param,
                                item_pusher,
                                &field_name,
                                ",",
                            )?);
                        }
                        CollectionFormat::Ssv => {
                            handler.push_str(&simple_query_array(
                                param,
                                item_pusher,
                                &field_name,
                                " ",
                            )?);
                        }
                        CollectionFormat::Tsv => {
                            handler.push_str(&simple_query_array(
                                param,
                                item_pusher,
                                &field_name,
                                "\\t",
                            )?);
                        }
                        CollectionFormat::Pipes => {
                            handler.push_str(&simple_query_array(
                                param,
                                item_pusher,
                                &field_name,
                                "|",
                            )?);
                        }
                        CollectionFormat::Multi => {
                            writeln!(&mut handler, "")?;
                            writeln!(&mut handler, "if !{field_name}.is_empty() {{")?;
                            writeln!(&mut handler, "for item in {field_name} {{")?;
                            writeln!(&mut handler, "s.push_str(\"{}=\");", param.name)?;
                            handler.push_str(item_pusher);
                            handler.push('\n');
                            writeln!(&mut handler, "s.push('&')")?;
                            writeln!(&mut handler, "}}")?;
                            writeln!(&mut handler, "}}")?;
                        }
                    }
                }
                ParameterType::File => eyre::bail!("cannot send file in query"),
            }
            if !required {
                writeln!(&mut handler, "}}")?;
            }
            imp.push_str(&handler);
        }
    }
    if fields.is_empty() {
        return Ok(String::new());
    } else {
        let op_name = op
            .operation_id
            .as_ref()
            .ok_or_else(|| eyre::eyre!("no op id found"))?
            .to_pascal_case();
        return Ok(format!("pub struct {op_name}Query {{\n{fields}\n}}\n\nimpl {op_name}Query {{\npub(crate) fn to_string(self) -> String {{\n{imp}\n}}\n}}"));
    }
}

fn simple_query_array(
    param: &Parameter,
    item_pusher: &str,
    name: &str,
    sep: &str,
) -> eyre::Result<String> {
    let mut out = String::new();
    writeln!(&mut out, "s.push_str(\"{}=\");", param.name)?;
    writeln!(&mut out, "")?;
    writeln!(&mut out, "if !{name}.is_empty() {{")?;
    writeln!(&mut out, "for (i, item) in {name}.iter().enumerate() {{")?;
    out.push_str(item_pusher);
    out.push('\n');
    writeln!(&mut out, "if i < {name}.len() - 1 {{")?;
    writeln!(&mut out, "s.push('{sep}')")?;
    writeln!(&mut out, "}}")?;
    writeln!(&mut out, "}}")?;
    writeln!(&mut out, "s.push('&')")?;
    writeln!(&mut out, "}}")?;
    Ok(out)
}
