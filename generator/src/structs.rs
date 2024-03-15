use crate::openapi::*;
use eyre::WrapErr;
use heck::ToPascalCase;
use std::fmt::Write;

pub fn create_structs(spec: &OpenApiV2) -> eyre::Result<String> {
    let mut s = String::new();
    s.push_str("use crate::StructureError;");
    if let Some(definitions) = &spec.definitions {
        for (name, schema) in definitions {
            let strukt = create_struct_for_definition(&spec, name, schema)?;
            s.push_str(&strukt);
        }
    }
    if let Some(responses) = &spec.responses {
        for (name, response) in responses {
            if let Some(headers) = &response.headers {
                let strukt = create_header_struct(name, headers)?;
                s.push_str(&strukt);
            }

            let tys = create_response_struct(spec, name, response)?;
            s.push_str(&tys);
        }
    }
    for (_, item) in &spec.paths {
        let strukt = create_query_structs_for_path(spec, item)?;
        s.push_str(&strukt);

        let strukt = create_response_structs(spec, item)?;
        s.push_str(&strukt);
    }
    Ok(s)
}

pub fn create_struct_for_definition(
    spec: &OpenApiV2,
    name: &str,
    schema: &Schema,
) -> eyre::Result<String> {
    if matches!(schema._type, Some(SchemaType::One(Primitive::Array))) {
        return Ok(String::new());
    }

    if schema._type == Some(SchemaType::One(Primitive::String)) {
        if let Some(_enum) = &schema._enum {
            return create_enum(name, schema.description.as_deref(), _enum, false);
        }
    }

    let mut subtypes = Vec::new();

    let docs = create_struct_docs(schema)?;
    let mut fields = String::new();
    let required = schema.required.as_deref().unwrap_or_default();
    if let Some(properties) = &schema.properties {
        for (prop_name, prop_schema) in properties {
            let prop_ty = crate::schema_ref_type_name(spec, prop_schema)?;
            let field_name = crate::sanitize_ident(prop_name);
            let mut field_ty = prop_ty.clone();
            if let MaybeRef::Value { value } = &prop_schema {
                crate::schema_subtype_name(spec, name, prop_name, value, &mut field_ty)?;
                crate::schema_subtypes(spec, name, prop_name, value, &mut subtypes)?;
            }
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
            if let MaybeRef::Value { value } = &prop_schema {
                if let Some(desc) = &value.description {
                    for line in desc.lines() {
                        fields.push_str("/// ");
                        fields.push_str(line);
                        fields.push_str("\n/// \n");
                    }
                    if fields.ends_with("/// \n") {
                        fields.truncate(fields.len() - 5);
                    }
                }
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
        let prop_ty = crate::schema_ref_type_name(spec, additonal_schema)?;
        fields.push_str("#[serde(flatten)]\n");
        fields.push_str("pub additional: std::collections::BTreeMap<String, ");
        fields.push_str(&prop_ty);
        fields.push_str(">,\n");
    }

    let mut out = format!("{docs}#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]\npub struct {name} {{\n{fields}}}\n\n");
    for subtype in subtypes {
        out.push_str(&subtype);
    }
    Ok(out)
}

pub fn create_enum(
    name: &str,
    desc: Option<&str>,
    _enum: &[serde_json::Value],
    imp_as_str: bool,
) -> eyre::Result<String> {
    let mut variants = String::new();
    let mut imp = String::new();
    imp.push_str("match self {");
    let docs = create_struct_docs_str(desc)?;
    for variant in _enum {
        match variant {
            serde_json::Value::String(s) => {
                let variant_name = s.to_pascal_case();
                variants.push_str("#[serde(rename = \"");
                variants.push_str(s);
                variants.push_str("\")]");
                variants.push_str(&variant_name);
                variants.push_str(",\n");

                writeln!(&mut imp, "{name}::{variant_name} => \"{s}\",")?;
            }
            x => eyre::bail!("cannot create enum variant. expected string, got {x:?}"),
        }
    }
    imp.push_str("}");

    let strukt = format!(
        "
{docs}
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum {name} {{
    {variants}
}}"
    );
    let out = if imp_as_str {
        let imp = format!(
            "\n\nimpl {name} {{
                fn as_str(&self) -> &'static str {{
                    {imp}
                }}
            }}"
        );
        format!("{strukt} {imp}")
    } else {
        strukt
    };
    Ok(out)
}

fn create_struct_docs(schema: &Schema) -> eyre::Result<String> {
    create_struct_docs_str(schema.description.as_deref())
}

fn create_struct_docs_str(description: Option<&str>) -> eyre::Result<String> {
    let doc = match description {
        Some(desc) => {
            let mut out = String::new();
            for line in desc.lines() {
                out.push_str("/// ");
                out.push_str(line);
                out.push_str("\n/// \n");
            }
            if out.ends_with("/// \n") {
                out.truncate(out.len() - 5);
            }
            out
        }
        None => String::new(),
    };
    Ok(doc)
}

pub fn create_query_structs_for_path(spec: &OpenApiV2, item: &PathItem) -> eyre::Result<String> {
    let mut s = String::new();
    if let Some(op) = &item.get {
        s.push_str(&create_query_struct(spec, op).wrap_err("GET")?);
    }
    if let Some(op) = &item.put {
        s.push_str(&create_query_struct(spec, op).wrap_err("PUT")?);
    }
    if let Some(op) = &item.post {
        s.push_str(&create_query_struct(spec, op).wrap_err("POST")?);
    }
    if let Some(op) = &item.delete {
        s.push_str(&create_query_struct(spec, op).wrap_err("DELETE")?);
    }
    if let Some(op) = &item.options {
        s.push_str(&create_query_struct(spec, op).wrap_err("OPTIONS")?);
    }
    if let Some(op) = &item.head {
        s.push_str(&create_query_struct(spec, op).wrap_err("HEAD")?);
    }
    if let Some(op) = &item.patch {
        s.push_str(&create_query_struct(spec, op).wrap_err("PATCH")?);
    }
    Ok(s)
}

pub fn query_struct_name(op: &Operation) -> eyre::Result<String> {
    let mut ty = op
        .operation_id
        .as_deref()
        .ok_or_else(|| eyre::eyre!("operation did not have id"))?
        .to_pascal_case()
        .replace("o_auth2", "oauth2");
    ty.push_str("Query");
    Ok(ty)
}

fn create_query_struct(spec: &OpenApiV2, op: &Operation) -> eyre::Result<String> {
    let params = match &op.parameters {
        Some(params) => params,
        None => return Ok(String::new()),
    };

    let op_name = query_struct_name(op)?;
    let mut enums = Vec::new();
    let mut fields = String::new();
    let mut imp = String::new();
    for param in params {
        let param = param.deref(spec)?;
        if let ParameterIn::Query { param: query_param } = &param._in {
            let field_name = crate::sanitize_ident(&param.name);
            let ty = match &query_param {
                NonBodyParameter {
                    _type: ParameterType::String,
                    _enum: Some(_enum),
                    ..
                } => {
                    let name = format!("{op_name}{}", param.name.to_pascal_case());
                    let enum_def = create_enum(&name, None, _enum, true)?;
                    enums.push(enum_def);
                    name
                }
                NonBodyParameter {
                    _type: ParameterType::Array,
                    items:
                        Some(Items {
                            _type: ParameterType::String,
                            _enum: Some(_enum),
                            ..
                        }),
                    ..
                } => {
                    let name = format!("{op_name}{}", param.name.to_pascal_case());
                    let enum_def = create_enum(&name, None, _enum, true)?;
                    enums.push(enum_def);
                    format!("Vec<{name}>")
                }
                _ => crate::methods::param_type(query_param, true)?,
            };
            if let Some(desc) = &param.description {
                for line in desc.lines() {
                    fields.push_str("/// ");
                    fields.push_str(line);
                    fields.push_str("\n/// \n");
                }
                if fields.ends_with("/// \n") {
                    fields.truncate(fields.len() - 5);
                }
            }
            fields.push_str("pub ");
            fields.push_str(&field_name);
            fields.push_str(": ");
            if query_param.required {
                fields.push_str(&ty);
            } else {
                fields.push_str("Option<");
                fields.push_str(&ty);
                fields.push_str(">");
            }
            fields.push_str(",\n");

            let mut handler = String::new();
            if query_param.required {
                writeln!(&mut handler, "let {field_name} = &self.{field_name};")?;
            } else {
                writeln!(
                    &mut handler,
                    "if let Some({field_name}) = &self.{field_name} {{"
                )?;
            }
            match &query_param._type {
                ParameterType::String => {
                    if let Some(_enum) = &query_param._enum {
                        writeln!(
                            &mut handler,
                            "write!(f, \"{}={{}}&\", {}.as_str())?;",
                            param.name, field_name,
                        )?;
                    } else {
                        match query_param.format.as_deref() {
                            Some("date-time" | "date") => {
                                writeln!(
                                    &mut handler,
                                    "write!(f, \"{}={{field_name}}&\", field_name = {field_name}.format(&time::format_description::well_known::Rfc3339).unwrap())?;",
                                    param.name)?;
                            }
                            _ => {
                                writeln!(
                                    &mut handler,
                                    "write!(f, \"{}={{{}}}&\")?;",
                                    param.name,
                                    field_name.strip_prefix("r#").unwrap_or(&field_name)
                                )?;
                            }
                        }
                    }
                }
                ParameterType::Number | ParameterType::Integer | ParameterType::Boolean => {
                    writeln!(
                        &mut handler,
                        "write!(f, \"{}={{{}}}&\")?;",
                        param.name,
                        field_name.strip_prefix("r#").unwrap_or(&field_name)
                    )?;
                }
                ParameterType::Array => {
                    let format = query_param
                        .collection_format
                        .unwrap_or(CollectionFormat::Csv);
                    let item = query_param
                        .items
                        .as_ref()
                        .ok_or_else(|| eyre::eyre!("array must have item type defined"))?;
                    let item_pusher = match item._type {
                        ParameterType::String => {
                            if let Some(_enum) = &item._enum {
                                "write!(f, \"{}\", item.as_str())?;"
                            } else {
                                match query_param.format.as_deref() {
                                    Some("date-time" | "date") => {
                                        "write!(f, \"{{date}}\", item.format(&time::format_description::well_known::Rfc3339).unwrap())?;"
                                    },
                                    _ => {
                                        "write!(f, \"{item}\")?;"
                                    }
                                }
                            }
                        }
                        ParameterType::Number | ParameterType::Integer | ParameterType::Boolean => {
                            "write!(f, \"{item}\")?;"
                        }
                        ParameterType::Array => {
                            eyre::bail!("nested arrays not supported in query");
                        }
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
                            writeln!(&mut handler)?;
                            writeln!(&mut handler, "if !{field_name}.is_empty() {{")?;
                            writeln!(&mut handler, "for item in {field_name} {{")?;
                            writeln!(&mut handler, "write!(f, \"{}=\")?;", param.name)?;
                            handler.push_str(item_pusher);
                            handler.push('\n');
                            writeln!(&mut handler, "write!(f, \"&\")?;")?;
                            writeln!(&mut handler, "}}")?;
                            writeln!(&mut handler, "}}")?;
                        }
                    }
                }
                ParameterType::File => eyre::bail!("cannot send file in query"),
            }
            if !query_param.required {
                writeln!(&mut handler, "}}")?;
            }
            imp.push_str(&handler);
        }
    }

    let result = if fields.is_empty() {
        String::new()
    } else {
        let mut out = format!(
            "
pub struct {op_name} {{
    {fields}
}}

impl std::fmt::Display for {op_name} {{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
        {imp}
        Ok(())
    }}
}}
"
        );

        for _enum in enums {
            out.push_str(&_enum);
        }

        out
    };

    Ok(result)
}

fn simple_query_array(
    param: &Parameter,
    item_pusher: &str,
    name: &str,
    sep: &str,
) -> eyre::Result<String> {
    let mut out = String::new();

    writeln!(
        &mut out,
        "
if !{name}.is_empty() {{
    write!(f, \"{}=\")?;
    for (item, i) in {name}.iter().enumerate() {{
        {item_pusher}
        if i < {name}.len() - 1 {{
            write!(f, \"{sep}\")?;
        }}
    }}
    write!(f, \"&\")?;
}}",
        param.name
    )?;

    Ok(out)
}

fn create_header_struct(
    name: &str,
    headers: &std::collections::BTreeMap<String, Header>,
) -> eyre::Result<String> {
    let ty_name = format!("{name}Headers").to_pascal_case();
    let mut fields = String::new();
    let mut imp = String::new();
    let mut imp_ret = String::new();
    for (header_name, header) in headers {
        let ty = header_type(header)?;
        let field_name = crate::sanitize_ident(header_name);
        fields.push_str("pub ");
        fields.push_str(&field_name);
        fields.push_str(": Option<");
        fields.push_str(&ty);
        fields.push_str(">,\n");

        write!(
            &mut imp,
            "
            let {field_name} = map.get(\"{header_name}\").map(|s| -> Result<_, _> {{
                let s = s.to_str().map_err(|_| StructureError::HeaderNotAscii)?;
        "
        )
        .unwrap();
        match &header._type {
            ParameterType::String => imp.push_str("Ok(s.to_string())"),
            ParameterType::Number => match header.format.as_deref() {
                Some("float") => {
                    imp.push_str("s.parse::<f32>().map_err(|_| StructureError::HeaderParseFailed)")
                }
                Some("double") | _ => {
                    imp.push_str("s.parse::<f64>()).map_err(|_| StructureError::HeaderParseFailed)")
                }
            },
            ParameterType::Integer => match header.format.as_deref() {
                Some("int64") => {
                    imp.push_str("s.parse::<u64>().map_err(|_| StructureError::HeaderParseFailed)")
                }
                Some("int32") | _ => {
                    imp.push_str("s.parse::<u32>().map_err(|_| StructureError::HeaderParseFailed)")
                }
            },
            ParameterType::Boolean => {
                imp.push_str("s.parse::<bool>().map_err(|_| StructureError::HeaderParseFailed)")
            }
            ParameterType::Array => {
                let sep = match header.collection_format {
                    Some(CollectionFormat::Csv) | None => ",",
                    Some(CollectionFormat::Ssv) => " ",
                    Some(CollectionFormat::Tsv) => "\\t",
                    Some(CollectionFormat::Pipes) => "|",
                    Some(CollectionFormat::Multi) => {
                        eyre::bail!("multi format not supported in headers")
                    }
                };
                let items = header
                    .items
                    .as_ref()
                    .ok_or_else(|| eyre::eyre!("items property must be set for arrays"))?;
                if items._type == ParameterType::String {
                    imp.push_str("Ok(");
                }
                imp.push_str("s.split(\"");
                imp.push_str(sep);
                imp.push_str("\").map(|s| ");
                imp.push_str(match items._type {
                    ParameterType::String => "s.to_string()).collect::<Vec<_>>())",
                    ParameterType::Number => match items.format.as_deref() {
                        Some("float") => "s.parse::<f32>()).collect::<Result<Vec<_>, _>>().map_err(|_| StructureError::HeaderParseFailed)",
                        Some("double") | _ => "s.parse::<f64>()).collect::<Result<Vec<_>, _>>().map_err(|_| StructureError::HeaderParseFailed)",
                    },
                    ParameterType::Integer => match items.format.as_deref() {
                        Some("int64") => "s.parse::<u64>()).collect::<Result<Vec<_>, _>>().map_err(|_| StructureError::HeaderParseFailed)",
                        Some("int32") | _ => "s.parse::<u32>()).collect::<Result<Vec<_>, _>>().map_err(|_| StructureError::HeaderParseFailed)",
                    },
                    ParameterType::Boolean => "s.parse::<bool>()).collect::<Result<Vec<_>, _>>().map_err(|_| StructureError::HeaderParseFailed)",
                    ParameterType::Array => eyre::bail!("nested arrays not supported in headers"),
                    ParameterType::File => eyre::bail!("files not supported in headers"),
                });
            }
            ParameterType::File => eyre::bail!("files not supported in headers"),
        }
        imp.push_str("}).transpose()?;");

        imp_ret.push_str(&field_name);
        imp_ret.push_str(", ");
    }

    Ok(format!(
        "
pub struct {ty_name} {{
    {fields}
}}

impl TryFrom<&reqwest::header::HeaderMap> for {ty_name} {{
    type Error = StructureError;

    fn try_from(map: &reqwest::header::HeaderMap) -> Result<Self, Self::Error> {{
        {imp}
        Ok(Self {{ {imp_ret} }})
    }}
}}
"
    ))
}

pub fn header_type(header: &Header) -> eyre::Result<String> {
    crate::methods::param_type_inner(
        &header._type,
        header.format.as_deref(),
        header.items.as_ref(),
        true,
    )
}

pub fn create_response_structs(spec: &OpenApiV2, item: &PathItem) -> eyre::Result<String> {
    let mut s = String::new();
    if let Some(op) = &item.get {
        s.push_str(&create_response_structs_for_op(spec, op).wrap_err("GET")?);
    }
    if let Some(op) = &item.put {
        s.push_str(&create_response_structs_for_op(spec, op).wrap_err("PUT")?);
    }
    if let Some(op) = &item.post {
        s.push_str(&create_response_structs_for_op(spec, op).wrap_err("POST")?);
    }
    if let Some(op) = &item.delete {
        s.push_str(&create_response_structs_for_op(spec, op).wrap_err("DELETE")?);
    }
    if let Some(op) = &item.options {
        s.push_str(&create_response_structs_for_op(spec, op).wrap_err("OPTIONS")?);
    }
    if let Some(op) = &item.head {
        s.push_str(&create_response_structs_for_op(spec, op).wrap_err("HEAD")?);
    }
    if let Some(op) = &item.patch {
        s.push_str(&create_response_structs_for_op(spec, op).wrap_err("PATCH")?);
    }
    Ok(s)
}

pub fn create_response_structs_for_op(spec: &OpenApiV2, op: &Operation) -> eyre::Result<String> {
    let mut out = String::new();
    let op_name = op
        .operation_id
        .as_deref()
        .ok_or_else(|| eyre::eyre!("no operation id"))?
        .to_pascal_case();
    for (_, response) in &op.responses.http_codes {
        let response = response.deref(spec)?;
        let tys = create_response_struct(spec, &op_name, response)?;
        out.push_str(&tys);
    }
    Ok(out)
}

pub fn create_response_struct(
    spec: &OpenApiV2,
    name: &str,
    res: &Response,
) -> eyre::Result<String> {
    let mut types = Vec::new();
    if let Some(MaybeRef::Value { value }) = &res.schema {
        crate::schema_subtypes(spec, name, "Response", value, &mut types)?;
    }
    let mut out = String::new();
    for ty in types {
        out.push_str(&ty);
    }
    Ok(out)
}
