use heck::{ToSnakeCase, ToUpperCamelCase};
use std::fmt::Write;

use eyre::{Context, OptionExt};

use crate::{
    openapi::{MaybeRef, Response},
    Items, NonBodyParameter, OpenApiV2, Operation, ParameterIn, ParameterType, PathItem,
};

pub fn create_endpoints(spec: &OpenApiV2) -> eyre::Result<String> {
    let mut s = String::new();
    s.push_str("use crate::{ForgejoError, StructureError};\n");
    s.push_str("use std::collections::BTreeMap;");
    s.push_str("use super::structs::*;\n");
    s.push_str("use crate::Endpoint;\n");
    s.push_str("use crate::{ApiRequest, RequestBody, ApiResponse};\n");
    s.push_str("use reqwest::Method;\n");
    s.push_str("use crate::sealed::Sealed;\n");
    s.push_str("use bytes::Bytes;\n");
    s.push_str("\n");
    for (path, item) in &spec.paths {
        s.push_str(&create_endpoints_for_path(&spec, path, item).wrap_err_with(|| path.clone())?);
    }
    Ok(s)
}

#[derive(Copy, Clone)]
enum Method {
    Get,
    Put,
    Post,
    Delete,
    Options,
    Head,
    Patch,
}

impl Method {
    fn code_string(self) -> &'static str {
        match self {
            Method::Get => "Method::GET",
            Method::Put => "Method::PUT",
            Method::Post => "Method::POST",
            Method::Delete => "Method::DELETE",
            Method::Options => "Method::OPTIONS",
            Method::Head => "Method::HEAD",
            Method::Patch => "Method::PATCH",
        }
    }
}

fn create_endpoints_for_path(
    spec: &OpenApiV2,
    path: &str,
    item: &PathItem,
) -> eyre::Result<String> {
    let mut s = String::new();
    if let Some(op) = &item.get {
        s.push_str(&create_endpoint(spec, path, op, Method::Get).wrap_err("GET")?);
    }
    if let Some(op) = &item.put {
        s.push_str(&create_endpoint(spec, path, op, Method::Put).wrap_err("PUT")?);
    }
    if let Some(op) = &item.post {
        s.push_str(&create_endpoint(spec, path, op, Method::Post).wrap_err("POST")?);
    }
    if let Some(op) = &item.delete {
        s.push_str(&create_endpoint(spec, path, op, Method::Delete).wrap_err("DELETE")?);
    }
    if let Some(op) = &item.options {
        s.push_str(&create_endpoint(spec, path, op, Method::Options).wrap_err("OPTIONS")?);
    }
    if let Some(op) = &item.head {
        s.push_str(&create_endpoint(spec, path, op, Method::Head).wrap_err("HEAD")?);
    }
    if let Some(op) = &item.patch {
        s.push_str(&create_endpoint(spec, path, op, Method::Patch).wrap_err("PATCH")?);
    }
    Ok(s)
}

fn create_endpoint(
    spec: &OpenApiV2,
    path: &str,
    op: &Operation,
    method: Method,
) -> eyre::Result<String> {
    let endpoint_struct = create_endpoint_struct(spec, op)?;
    let endpoint_impl = create_endpoint_impl(spec, path, op, method)?;
    Ok(format!("{endpoint_struct}\n{endpoint_impl}"))
}

fn create_endpoint_struct(spec: &OpenApiV2, op: &Operation) -> eyre::Result<String> {
    let mut s = String::new();
    let struct_name = op
        .operation_id
        .as_deref()
        .ok_or_eyre("op does not have id")?
        .to_upper_camel_case();
    if has_references(spec, op)? {
        writeln!(&mut s, "pub struct {struct_name}<'a> {{")?;
    } else {
        writeln!(&mut s, "pub struct {struct_name} {{")?;
    }
    writeln!(&mut s, "{}", struct_fields_from_op(spec, op)?)?;
    writeln!(&mut s, "}}")?;
    Ok(s)
}

fn has_references(spec: &OpenApiV2, op: &Operation) -> eyre::Result<bool> {
    if let Some(params) = &op.parameters {
        for param in params {
            let full_param = param.deref(spec)?;
            match &full_param._in {
                ParameterIn::Path { param } => {
                    if param_is_reference(param) {
                        return Ok(true);
                    }
                }
                _ => (),
            }
        }
    }
    Ok(false)
}

fn has_path_params(spec: &OpenApiV2, op: &Operation) -> eyre::Result<bool> {
    if let Some(params) = &op.parameters {
        for param in params {
            let full_param = param.deref(spec)?;
            match &full_param._in {
                ParameterIn::Path { param: _ } => {
                    return Ok(true);
                }
                _ => (),
            }
        }
    }
    Ok(false)
}

fn param_is_reference(param: &NonBodyParameter) -> bool {
    match &param._type {
        ParameterType::String => match &param.format.as_deref() {
            Some("date") | Some("date-time") | Some("url" | "ssh-url") => false,
            _ => true,
        },
        ParameterType::Array => true,
        ParameterType::File => true,
        _ => false,
    }
}

fn struct_fields_from_op(spec: &OpenApiV2, op: &Operation) -> eyre::Result<String> {
    let mut has_query = false;
    let mut fields = String::new();
    // let mut has_headers = false;
    if let Some(params) = &op.parameters {
        for param in params {
            let full_param = param.deref(spec)?;
            match &full_param._in {
                ParameterIn::Path { param } => {
                    let type_name = param_type(&param, false)?;
                    fields.push_str("pub ");
                    fields.push_str(&crate::sanitize_ident(&full_param.name));
                    fields.push_str(": ");
                    fields.push_str(&type_name);
                    fields.push_str(", ");
                }
                ParameterIn::Query { .. } => {
                    if !matches!(&*full_param.name, "page" | "limit") {
                        has_query = true
                    }
                }
                ParameterIn::Header { param: _ } => (),
                ParameterIn::Body { schema } => {
                    let ty = crate::schema_ref_type_name(spec, schema)?;
                    //fields.push_str(&crate::sanitize_ident(&full_param.name));
                    fields.push_str("pub body: ");
                    fields.push_str(&ty);
                    fields.push_str(", ");
                }
                ParameterIn::FormData { param } => {
                    let type_name = param_type(&param, false)?;
                    fields.push_str("pub ");
                    fields.push_str(&crate::sanitize_ident(&full_param.name));
                    if param.required {
                        fields.push_str(": ");
                        fields.push_str(&type_name);
                        fields.push_str(", ");
                    } else {
                        fields.push_str(": Option<");
                        fields.push_str(&type_name);
                        fields.push_str(">, ");
                    }
                }
            }
        }
    }
    if has_query {
        let query_ty = crate::structs::query_struct_name(op)?;
        fields.push_str("pub query: ");
        fields.push_str(&query_ty);
        fields.push_str(",\n");
    }
    Ok(fields)
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
            Some("url" | "ssh-url") => "url::Url",
            _ => {
                if owned {
                    "String"
                } else {
                    "&'a str"
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
                format!("&'a [{item_ty_name}]")
            }
        }
        ParameterType::File => {
            if owned {
                format!("Vec<u8>")
            } else {
                format!("&'a [u8]")
            }
        }
    };
    Ok(ty_name)
}

fn create_endpoint_impl(
    spec: &OpenApiV2,
    path: &str,
    op: &Operation,
    method: Method,
) -> eyre::Result<String> {
    let mut s = String::new();
    let base_path = spec.base_path.as_deref().unwrap_or_default();
    let struct_name = op
        .operation_id
        .as_deref()
        .ok_or_eyre("op does not have id")?
        .to_upper_camel_case();
    if has_references(spec, op)? {
        writeln!(&mut s, "impl Sealed for {struct_name}<'_> {{}}")?;
        writeln!(&mut s, "impl Endpoint for {struct_name}<'_> {{")?;
    } else {
        writeln!(&mut s, "impl Sealed for {struct_name} {{}}")?;
        writeln!(&mut s, "impl Endpoint for {struct_name} {{")?;
    }
    let response_type = response_type_from_op(spec, op)?;
    writeln!(&mut s, "    type Response = {response_type};")?;
    let make_request = endpoint_make_request(spec, base_path, path, op, method)?;
    writeln!(&mut s, "{make_request}")?;
    let handle_response = endpoint_handle_response(spec, op, &response_type)?;
    writeln!(&mut s, "{handle_response}")?;
    Ok(s)
}

fn endpoint_make_request(
    spec: &OpenApiV2,
    base_path: &str,
    path: &str,
    op: &Operation,
    method: Method,
) -> eyre::Result<String> {
    let mut s = String::new();
    writeln!(&mut s, "    fn make_request(self) -> ApiRequest {{")?;
    writeln!(&mut s, "        ApiRequest {{")?;
    writeln!(&mut s, "            method: {},", method.code_string())?;
    if has_path_params(spec, op)? {
        let (fmt_str, path_fields) = path_to_fmt_string(&mut &*path);
        write!(
            &mut s,
            "            path: format!(\"{base_path}{}\"",
            fmt_str
        )?;
        for field in path_fields {
            write!(&mut s, ", self.{}", crate::sanitize_ident(&field))?;
        }
        writeln!(&mut s, ").into(),")?;
    } else {
        writeln!(&mut s, "            path: \"{base_path}{path}\".into(),")?;
    }

    let mut has_query = false;
    let mut has_body = false;
    let mut has_form = false;
    if let Some(params) = &op.parameters {
        for param in params {
            let full_param = param.deref(spec)?;
            match &full_param._in {
                ParameterIn::Query { .. } => {
                    if !matches!(&*full_param.name, "page" | "limit") {
                        has_query = true
                    }
                }
                ParameterIn::Body { .. } => has_body = true,
                ParameterIn::FormData { .. } => has_form = true,
                _ => (),
            }
            if has_query && (has_body || has_form) {
                break;
            }
        }
    }
    if has_query {
        writeln!(&mut s, "            query: Some(self.query.into_list()),")?;
    } else {
        writeln!(&mut s, "            query: None,")?;
    }
    if has_body {
        writeln!(&mut s, "            body: RequestBody::Json(Bytes::from(serde_json::to_vec(&self.body).expect(\"failed to serialize value. This is probably a bug in forgejo-api\"))),")?;
    } else if has_form {
        write!(&mut s, "            body: {{")?;
        write!(&mut s, "                let mut list = Vec::new();")?;
        for param in op.parameters.as_ref().unwrap() {
            let full_param = param.deref(spec)?;
            match &full_param._in {
                ParameterIn::FormData { param } => {
                    let field_name = &crate::sanitize_ident(&full_param.name);
                    if param.required {
                        writeln!(&mut s, "let {field_name} = self.{field_name};")?;
                    } else {
                        writeln!(&mut s, "if let Some({field_name}) = self.{field_name} {{")?;
                    }
                    write!(&mut s, "list.push((\"{}\", ", full_param.name)?;
                    match param._type {
                        ParameterType::String => write!(&mut s, "{field_name}.as_bytes().into()")?,
                        ParameterType::File => write!(&mut s, "{field_name}.to_vec()")?,
                        _ => eyre::bail!("unsupported form"),
                    }
                    writeln!(&mut s, "));")?;
                    if !param.required {
                        write!(&mut s, "}}")?;
                    }
                }
                _ => (),
            }
        }
        writeln!(&mut s, "RequestBody::Form(list)")?;
        writeln!(&mut s, "}},")?;
    } else {
        writeln!(&mut s, "            body: RequestBody::None,")?;
    }
    writeln!(&mut s, "            page: None,")?;
    writeln!(&mut s, "            limit: None,")?;
    writeln!(&mut s, "        }}")?;
    writeln!(&mut s, "    }}")?;

    Ok(s)
}

fn endpoint_handle_response(
    spec: &OpenApiV2,
    op: &Operation,
    response_type: &ResponseType,
) -> eyre::Result<String> {
    let mut s = String::new();

    writeln!(
        &mut s,
        "    fn handle_response(response: ApiResponse) -> Result<Self::Response, ForgejoError> {{"
    )?;

    let mut has_empty = false;
    let mut only_empty = true;
    for (code, res) in &op.responses.http_codes {
        let response = response_ref_type_name(spec, res, op)?;
        if !code.starts_with("2") {
            continue;
        }
        if matches!(response.body.as_deref(), Some("()") | None) {
            has_empty = true;
        } else {
            only_empty = false;
        }
    }
    let optional = has_empty && !only_empty;
    writeln!(&mut s, "match response.status_code.as_u16() {{")?;
    for (code, res) in &op.responses.http_codes {
        let branch_ret = response_ref_type_name(spec, res, op)?;
        let res = res.deref(spec)?;
        if !code.starts_with("2") {
            continue;
        }
        write!(&mut s, "{code}")?;
        write!(&mut s, " => Ok(")?;
        let mut handlers = Vec::new();
        let header_handler = match &res.headers {
            Some(_) => {
                if response_type
                    .headers
                    .as_ref()
                    .map(|s| s.starts_with("Option<"))
                    .unwrap()
                {
                    Some("Some((&response.headers).try_into()?)")
                } else {
                    Some("(&response.headers).try_into()?")
                }
            }
            None => {
                if response_type.headers.is_some() {
                    Some("None")
                } else {
                    None
                }
            }
        };
        handlers.extend(header_handler);
        let body_handler = match branch_ret.kind {
            Some(ResponseKind::Text) => {
                if optional {
                    Some("Some(std::str::from_utf8(&*response.body)?.to_owned())")
                } else {
                    Some("std::str::from_utf8(&*response.body)?.to_owned()")
                }
            }
            Some(ResponseKind::Json) => {
                if optional {
                    Some("Some(serde_json::from_slice(&*response.body).map_err(|e| StructureError::Serde { e, contents: response.body.clone() })?)")
                } else {
                    Some("serde_json::from_slice(&*response.body).map_err(|e| StructureError::Serde { e, contents: response.body.clone() })?")
                }
            }
            Some(ResponseKind::Bytes) => {
                if optional {
                    Some("Some(response.body.clone())")
                } else {
                    Some("response.body.clone()")
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
            [single] => write!(&mut s, "{single}")?,
            _ => {
                write!(&mut s, "(")?;
                for (i, item) in handlers.iter().copied().enumerate() {
                    write!(&mut s, "{item}")?;
                    if i + 1 < handlers.len() {
                        write!(&mut s, ", ")?;
                    }
                }
                write!(&mut s, ")")?;
            }
        }
        writeln!(&mut s, "),")?;
    }
    writeln!(
        &mut s,
        "_ => Err(ForgejoError::UnexpectedStatusCode(response.status_code.clone()))"
    )?;
    writeln!(&mut s, "}}")?;
    writeln!(&mut s, "    }}")?;

    writeln!(&mut s, "}}")?;

    Ok(s)
}

#[derive(Debug, Default)]
pub struct ResponseType {
    pub headers: Option<String>,
    pub body: Option<String>,
    pub kind: Option<ResponseKind>,
}

impl ResponseType {
    fn merge(self, other: Self) -> eyre::Result<Self> {
        let headers = match (self.headers, other.headers) {
            (Some(a), Some(b)) if a != b => eyre::bail!("incompatible header types in response"),
            (Some(a), None) if !a.starts_with("Option<") => Some(format!("Option<{a}>")),
            (None, Some(b)) if !b.starts_with("Option<") => Some(format!("Option<{b}>")),
            (a, b) => a.or(b),
        };
        let body = match (self.body.as_deref(), other.body.as_deref()) {
            (Some(a), Some(b)) if a != b => eyre::bail!("incompatible header types in response"),
            (Some(a), Some("()") | None) if !a.starts_with("Option<") => {
                Some(format!("Option<{a}>"))
            }
            (Some("()") | None, Some(b)) if !b.starts_with("Option<") => {
                Some(format!("Option<{b}>"))
            }
            (_, _) => self.body.or(other.body),
        };
        use ResponseKind::*;
        let kind = match (self.kind, other.kind) {
            (a, None) => a,
            (None, b) => b,
            (Some(Json), Some(Json)) => Some(Json),
            (Some(Bytes), Some(Bytes)) => Some(Bytes),
            (Some(Bytes), Some(Text)) => Some(Bytes),
            (Some(Text), Some(Bytes)) => Some(Bytes),
            (Some(Text), Some(Text)) => Some(Text),
            _ => eyre::bail!("incompatible response kinds"),
        };
        let new = Self {
            headers,
            body,
            kind,
        };
        Ok(new)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ResponseKind {
    Json,
    Text,
    Bytes,
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

pub fn response_type_from_op(spec: &OpenApiV2, op: &Operation) -> eyre::Result<ResponseType> {
    let responses = op
        .responses
        .http_codes
        .iter()
        .filter(|(k, _)| k.starts_with("2"))
        .map(|(_, v)| response_ref_type_name(spec, v, op))
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

pub fn response_ref_type_name(
    spec: &OpenApiV2,
    response_ref: &MaybeRef<Response>,
    op: &Operation,
) -> eyre::Result<ResponseType> {
    let response = response_ref.deref(spec)?;
    let mut ty = ResponseType::default();
    if response.headers.is_some() {
        let parent_name = match &response_ref {
            MaybeRef::Ref { _ref } => _ref
                .rsplit_once("/")
                .ok_or_else(|| eyre::eyre!("invalid ref"))?
                .1
                .to_string(),
            MaybeRef::Value { value: _ } => {
                eyre::bail!("could not find parent name for header type")
            }
        };
        ty.headers = Some(format!("{}Headers", parent_name.to_upper_camel_case()));
    }
    let produces = op
        .produces
        .as_deref()
        .or_else(|| spec.produces.as_deref())
        .unwrap_or_default();
    // can't use .contains() because Strings
    let produces_json = produces.iter().any(|i| matches!(&**i, "application/json"));
    let produces_text = produces.iter().any(|i| i.starts_with("text/"));
    let produces_other = produces
        .iter()
        .any(|i| !matches!(&**i, "application/json") && !i.starts_with("text/"));
    match (produces_json, produces_text, produces_other) {
        (true, false, false) => {
            if let Some(schema) = &response.schema {
                ty.kind = Some(ResponseKind::Json);
                let mut body = crate::schema_ref_type_name(spec, schema)?;
                if let MaybeRef::Value { value } = schema {
                    let op_name = op.operation_id.as_deref().ok_or_else(|| eyre::eyre!("no operation id"))?.to_upper_camel_case();
                    crate::schema_subtype_name(spec, &op_name, "Response", value, &mut body)?;
                }
                ty.body = Some(body);
            };
        }
        (false, _, true) => {
            ty.kind = Some(ResponseKind::Bytes);
            ty.body = Some("Bytes".into());
        }
        (false, true, false) => {
            ty.kind = Some(ResponseKind::Text);
            ty.body = Some("String".into());
        }
        (false, false, false) => {
            ty.kind = None;
            ty.body = None;
        }
        _ => eyre::bail!("produces value unsupported. json: {produces_json}, text: {produces_text}, other: {produces_other}"),
    };
    Ok(ty)
}

fn path_to_fmt_string<'a>(path: &mut &'a str) -> (String, Vec<String>) {
    let mut out = String::new();
    let mut fields = Vec::new();
    while let Some((head, tail)) = path.split_once("{") {
        let (field, tail) = tail.split_once("}").expect("unterminated value in path");
        fields.push(field.to_snake_case());
        out.push_str(head);
        out.push_str("{}");
        *path = tail;
    }
    out.push_str(path);
    (out, fields)
}
