use crate::openapi::*;
use eyre::{OptionExt, WrapErr};
use heck::ToSnakeCase;
use std::fmt::Write;

pub fn create_methods(spec: &OpenApiV2) -> eyre::Result<String> {
    let mut s = String::new();
    s.push_str("use crate::ForgejoError;\n");
    s.push_str("impl crate::Forgejo {\n");
    for (path, item) in &spec.paths {
        s.push_str(&create_methods_for_path(&spec, path, item).wrap_err_with(|| path.clone())?);
    }
    s.push_str("}\n");
    Ok(s)
}

fn create_methods_for_path(spec: &OpenApiV2, path: &str, item: &PathItem) -> eyre::Result<String> {
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
                ParameterIn::Path { param: _ }
                | ParameterIn::Body { schema: _ }
                | ParameterIn::FormData { param: _ } => {
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
    // let mut has_headers = false;
    if let Some(params) = &op.parameters {
        for param in params {
            let full_param = param.deref(spec)?;
            match &full_param._in {
                ParameterIn::Path { param } => {
                    let type_name = param_type(&param, false)?;
                    args.push_str(", ");
                    args.push_str(&crate::sanitize_ident(&full_param.name));
                    args.push_str(": ");
                    args.push_str(&type_name);
                }
                ParameterIn::Query { param: _ } => has_query = true,
                ParameterIn::Header { param: _ } => (), // has_headers = true,
                ParameterIn::Body { schema } => {
                    let ty = crate::schema_ref_type_name(spec, schema)?;
                    args.push_str(", ");
                    args.push_str(&crate::sanitize_ident(&full_param.name));
                    args.push_str(": ");
                    args.push_str(&ty);
                }
                ParameterIn::FormData { param: _ } => {
                    args.push_str(", ");
                    args.push_str(&crate::sanitize_ident(&full_param.name));
                    args.push_str(": Vec<u8>");
                }
            }
        }
    }
    if has_query {
        let query_ty = crate::structs::query_struct_name(op)?;
        args.push_str(", query: ");
        args.push_str(&query_ty);
    }
    Ok(args)
}

pub fn param_type(param: &NonBodyParameter, owned: bool) -> eyre::Result<String> {
    param_type_inner(
        &param._type,
        param.format.as_deref(),
        param.items.as_ref(),
        owned,
    )
}

pub fn param_type_inner(
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

fn fn_return_from_op(spec: &OpenApiV2, op: &Operation) -> eyre::Result<ResponseType> {
    let responses = op
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

fn response_ref_type_name(
    spec: &OpenApiV2,
    schema: &MaybeRef<Response>,
) -> eyre::Result<ResponseType> {
    let response = schema.deref(spec)?;
    let mut ty = ResponseType::default();
    if response.headers.is_some() {
        let parent_name = match &schema {
            MaybeRef::Ref { _ref } => _ref
                .rsplit_once("/")
                .ok_or_else(|| eyre::eyre!("invalid ref"))?
                .1
                .to_string(),
            MaybeRef::Value { value: _ } => {
                eyre::bail!("could not find parent name for header type")
            }
        };
        ty.headers = Some(format!("{}Headers", parent_name));
    }
    if let Some(schema) = &response.schema {
        ty.body = Some(crate::schema_ref_type_name(spec, schema)?);
    };
    Ok(ty)
}

fn create_method_body(
    spec: &OpenApiV2,
    method: &str,
    path: &str,
    op: &Operation,
) -> eyre::Result<String> {
    let request = create_method_request(spec, method, path, op)?;
    let response = create_method_response(spec, op)?;
    Ok(format!("{request}\n    {response}"))
}

fn create_method_request(
    spec: &OpenApiV2,
    method: &str,
    path: &str,
    op: &Operation,
) -> eyre::Result<String> {
    let mut has_query = false;
    // let mut has_headers = false;
    let mut body_method = String::new();
    if let Some(params) = &op.parameters {
        for param in params {
            let param = match &param {
                MaybeRef::Value { value } => value,
                MaybeRef::Ref { _ref } => eyre::bail!("todo: add deref parameters"),
            };
            let name = crate::sanitize_ident(&param.name);
            match &param._in {
                ParameterIn::Path { param: _ } => (/* do nothing */),
                ParameterIn::Query { param: _ } => has_query = true,
                ParameterIn::Header { param: _ } => (), // _has_headers = true,
                ParameterIn::Body { schema: _ } => {
                    if !body_method.is_empty() {
                        eyre::bail!("cannot have more than one body parameter");
                    }
                    if param_is_string(spec, param)? {
                        body_method = format!(".body({name})");
                    } else {
                        body_method = format!(".json(&{name})");
                    }
                }
                ParameterIn::FormData { param: _ } => {
                    if !body_method.is_empty() {
                        eyre::bail!("cannot have more than one body parameter");
                    }
                    body_method = format!(".multipart(reqwest::multipart::Form::new().part(\"attachment\", reqwest::multipart::Part::bytes({name}).file_name(\"file\").mime_str(\"*/*\").unwrap()))");
                }
            }
        }
    }
    let mut fmt_str = sanitize_path_arg(path)?;
    if has_query {
        fmt_str.push_str("?{query}");
    }
    let path_arg = if fmt_str.contains("{") {
        format!("&format!(\"{fmt_str}\")")
    } else {
        format!("\"{fmt_str}\"")
    };

    let out = format!("let request = self.{method}({path_arg}){body_method}.build()?;");
    Ok(out)
}

fn param_is_string(spec: &OpenApiV2, param: &Parameter) -> eyre::Result<bool> {
    match &param._in {
        ParameterIn::Body { schema } => crate::schema_is_string(spec, schema),
        ParameterIn::Path { param }
        | ParameterIn::Query { param }
        | ParameterIn::Header { param }
        | ParameterIn::FormData { param } => {
            let is_str = match param._type {
                ParameterType::String => true,
                _ => false,
            };
            Ok(is_str)
        }
    }
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

fn create_method_response(spec: &OpenApiV2, op: &Operation) -> eyre::Result<String> {
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
        let res = res.deref(spec)?;
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
                    Some("Some(response.headers().try_into()?)")
                } else {
                    Some("response.headers().try_into()?")
                }
            }
            None => {
                if fn_ret.headers.is_some() {
                    Some("None")
                } else {
                    None
                }
            }
        };
        handlers.extend(header_handler);
        let body_handler = match &res.schema {
            Some(schema) if crate::schema_is_string(spec, schema)? => {
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
            (_, _) => new.body = self.body.or(other.body),
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
