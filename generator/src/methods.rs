use crate::{openapi::*, schema_ref_type_name};
use eyre::WrapErr;
use heck::{ToPascalCase, ToSnakeCase};
use std::fmt::Write;

pub fn create_methods(spec: &OpenApiV2, sync: bool) -> eyre::Result<String> {
    let mut s = String::new();
    s.push_str("use crate::ForgejoError;\n");
    s.push_str("use crate::Endpoint;\n");
    s.push_str("use crate::WrappedRequest;\n");
    s.push_str("use crate::generated::endpoints;\n");
    s.push_str("use std::collections::BTreeMap;");
    s.push_str("use super::structs::*;\n");
    s.push_str("use bytes::Bytes;\n");
    s.push('\n');
    if sync {
        s.push_str("impl crate::sync::Forgejo {\n");
    } else {
        s.push_str("impl crate::Forgejo {\n");
    }
    for (path, item) in &spec.paths {
        s.push_str(&create_methods_for_path(spec, item, sync).wrap_err_with(|| path.clone())?);
    }
    s.push_str("}\n");
    Ok(s)
}

fn create_methods_for_path(spec: &OpenApiV2, item: &PathItem, sync: bool) -> eyre::Result<String> {
    let mut s = String::new();
    if let Some(op) = &item.get {
        s.push_str(&create_method(spec, op, sync).wrap_err("GET")?);
    }
    if let Some(op) = &item.put {
        s.push_str(&create_method(spec, op, sync).wrap_err("PUT")?);
    }
    if let Some(op) = &item.post {
        s.push_str(&create_method(spec, op, sync).wrap_err("POST")?);
    }
    if let Some(op) = &item.delete {
        s.push_str(&create_method(spec, op, sync).wrap_err("DELETE")?);
    }
    if let Some(op) = &item.options {
        s.push_str(&create_method(spec, op, sync).wrap_err("OPTIONS")?);
    }
    if let Some(op) = &item.head {
        s.push_str(&create_method(spec, op, sync).wrap_err("HEAD")?);
    }
    if let Some(op) = &item.patch {
        s.push_str(&create_method(spec, op, sync).wrap_err("PATCH")?);
    }
    Ok(s)
}

fn create_method(spec: &OpenApiV2, op: &Operation, sync: bool) -> eyre::Result<String> {
    let doc = method_docs(spec, op)?;
    let sig = fn_signature_from_op(spec, op, sync)?;
    let body = create_method_body(spec, op, sync)?;
    Ok(format!("{doc}{sig} {{\n    {body}\n}}\n\n"))
}

fn method_docs(spec: &OpenApiV2, op: &Operation) -> eyre::Result<String> {
    let mut out = String::new();
    let mut prev = false;
    if let Some(summary) = &op.summary {
        let summary = summary
            .replace("gitea", "Forgejo")
            .replace("Gitea", "Forgejo");
        writeln!(&mut out, "/// {summary}")?;
        prev = true;
    }
    if let Some(params) = &op.parameters {
        if prev {
            out.push_str("///\n");
        }
        for param in params {
            let param = param.deref(spec)?;
            match &param._in {
                ParameterIn::Path { param: _ } | ParameterIn::FormData { param: _ } => {
                    write!(&mut out, "/// - `{}`", param.name)?;
                    if let Some(description) = &param.description {
                        write!(&mut out, ": {description}")?;
                    }
                    writeln!(&mut out)?;
                }
                ParameterIn::Body { schema } => {
                    write!(&mut out, "/// - `{}`", param.name)?;
                    let ty = schema_ref_type_name(spec, schema)?;
                    if let Some(description) = &param.description {
                        write!(&mut out, ": {description}\n\n///   See [`{ty}`]")?;
                    } else {
                        write!(&mut out, ": See [`{ty}`]")?;
                    }
                    writeln!(&mut out)?;
                }
                _ => (),
            }
        }
    }
    if out.ends_with("/// \n") {
        out.truncate(out.len() - 5);
    }
    Ok(out)
}

fn fn_signature_from_op(spec: &OpenApiV2, op: &Operation, sync: bool) -> eyre::Result<String> {
    let name = op
        .operation_id
        .as_deref()
        .ok_or_else(|| eyre::eyre!("operation did not have id"))?
        .to_snake_case()
        .replace("o_auth2", "oauth2");
    let struct_name = op
        .operation_id
        .as_deref()
        .ok_or_else(|| eyre::eyre!("operation did not have id"))?
        .to_pascal_case();
    let args = fn_args_from_op(spec, op)?;
    let prefix = if sync { "crate::sync::" } else { "" };
    let code = format!("pub fn {name}({args}) -> {prefix}WrappedRequest<'_, endpoints::{struct_name}, <endpoints::{struct_name} as Endpoint>::Response>");
    Ok(code)
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
                    let type_name = param_type(param, false)?;
                    args.push_str(", ");
                    args.push_str(&crate::sanitize_ident(&full_param.name));
                    args.push_str(": ");
                    args.push_str(&type_name);
                }
                ParameterIn::Query { param: _ } => {
                    if !matches!(&*full_param.name, "page" | "limit") {
                        has_query = true
                    }
                }
                ParameterIn::Header { param: _ } => (), // has_headers = true,
                ParameterIn::Body { schema } => {
                    let ty = crate::schema_ref_type_name(spec, schema)?;
                    args.push_str(", ");
                    args.push_str(&crate::sanitize_ident(&full_param.name));
                    args.push_str(": ");
                    args.push_str(&ty);
                }
                ParameterIn::FormData { param } => {
                    let type_name = param_type(param, false)?;
                    args.push_str(", ");
                    args.push_str(&crate::sanitize_ident(&full_param.name));
                    if param.required {
                        args.push_str(": ");
                        args.push_str(&type_name);
                    } else {
                        args.push_str(": Option<");
                        args.push_str(&type_name);
                        args.push('>');
                    }
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
        ParameterType::String => match format {
            Some("date") => "time::Date",
            Some("date-time") => "time::OffsetDateTime",
            Some("url" | "ssh-url") => "url::Url",
            _ => {
                if owned {
                    "String"
                } else {
                    "&str"
                }
            }
        }
        .into(),
        ParameterType::Number => match format {
            Some("float") => "f32",
            Some("double") => "f64",
            _ => "f64",
        }
        .into(),
        ParameterType::Integer => match format {
            Some("uint32") => "u32",
            Some("uint64") => "u64",
            Some("int32") => "i32",
            Some("int64") => "i64",
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
                "Vec<u8>".to_string()
            } else {
                "&[u8]".to_string()
            }
        }
    };
    Ok(ty_name)
}

fn create_method_body(spec: &OpenApiV2, op: &Operation, sync: bool) -> eyre::Result<String> {
    let mut s = String::new();
    let struct_name = op
        .operation_id
        .as_deref()
        .ok_or_else(|| eyre::eyre!("operation did not have id"))?
        .to_pascal_case();
    writeln!(&mut s, "endpoints::{struct_name} {{")?;
    if let Some(params) = &op.parameters {
        let mut has_query = false;
        for param in params {
            let full_param = param.deref(spec)?;
            match &full_param._in {
                ParameterIn::Query { .. } => {
                    if !matches!(&*full_param.name, "page" | "limit") {
                        has_query = true
                    }
                }
                ParameterIn::Body { .. } => {
                    let name = crate::sanitize_ident(&full_param.name);
                    writeln!(&mut s, "body: {name}, ")?;
                }
                ParameterIn::FormData { param } => {
                    let name = crate::sanitize_ident(&full_param.name);
                    if param.required {
                        writeln!(&mut s, "{name}: &{name}, ")?;
                    } else {
                        writeln!(&mut s, "{name}: {name}.as_deref(), ")?;
                    }
                }
                _ => {
                    let name = crate::sanitize_ident(&full_param.name);
                    writeln!(&mut s, "{name}, ")?;
                }
            }
        }
        if has_query {
            writeln!(&mut s, "query, ")?;
        }
    }
    if sync {
        writeln!(&mut s, "}}.make_request().wrap_sync::<_, _>(self)")?;
    } else {
        writeln!(&mut s, "}}.make_request().wrap::<_, _>(self)")?;
    }
    Ok(s)
}
