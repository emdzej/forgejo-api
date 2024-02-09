use crate::openapi::*;
use eyre::WrapErr;
use heck::ToPascalCase;
use std::fmt::Write;

pub fn create_structs(spec: &OpenApiV2) -> eyre::Result<String> {
    let mut s = String::new();
    s.push_str("use structs::*;\n");
    s.push_str("pub mod structs {\n");
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
        }
    }
    for (_, item) in &spec.paths {
        let strukt = create_query_structs_for_path(item)?;
        s.push_str(&strukt);
    }
    s.push_str("\n}");
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

    let docs = create_struct_docs(schema)?;
    let mut fields = String::new();
    let required = schema.required.as_deref().unwrap_or_default();
    if let Some(properties) = &schema.properties {
        for (prop_name, prop_schema) in properties {
            let prop_ty = crate::schema_ref_type_name(spec, prop_schema)?;
            let field_name = crate::sanitize_ident(prop_name);
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
            if let MaybeRef::Value { value } = &prop_schema {
                if let Some(desc) = &value.description {
                    for line in desc.lines() {
                        fields.push_str("/// ");
                        fields.push_str(line);
                        fields.push_str("\n/// \n");
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

pub fn create_query_structs_for_path(item: &PathItem) -> eyre::Result<String> {
    let mut s = String::new();
    if let Some(op) = &item.get {
        s.push_str(&create_query_struct(op).wrap_err("GET")?);
    }
    if let Some(op) = &item.put {
        s.push_str(&create_query_struct(op).wrap_err("PUT")?);
    }
    if let Some(op) = &item.post {
        s.push_str(&create_query_struct(op).wrap_err("POST")?);
    }
    if let Some(op) = &item.delete {
        s.push_str(&create_query_struct(op).wrap_err("DELETE")?);
    }
    if let Some(op) = &item.options {
        s.push_str(&create_query_struct(op).wrap_err("OPTIONS")?);
    }
    if let Some(op) = &item.head {
        s.push_str(&create_query_struct(op).wrap_err("HEAD")?);
    }
    if let Some(op) = &item.patch {
        s.push_str(&create_query_struct(op).wrap_err("PATCH")?);
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

fn create_query_struct(op: &Operation) -> eyre::Result<String> {
    let params = match &op.parameters {
        Some(params) => params,
        None => return Ok(String::new()),
    };

    let mut fields = String::new();
    let mut imp = String::new();
    for param in params {
        let param = match &param {
            MaybeRef::Value { value } => value,
            MaybeRef::Ref { _ref } => eyre::bail!("todo: add deref parameters"),
        };
        if let ParameterIn::Query { param: query_param } = &param._in {
            let ty = crate::methods::param_type(query_param, true)?;
            let field_name = crate::sanitize_ident(&param.name);
            if let Some(desc) = &param.description {
                for line in desc.lines() {
                    fields.push_str("/// ");
                    fields.push_str(line);
                    fields.push_str("\n/// \n");
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
                ParameterType::String => match query_param.format.as_deref() {
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
                },
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
                            match query_param.format.as_deref() {
                                Some("date-time" | "date") => {
                                    "write!(f, \"{{date}}\", item.format(&time::format_description::well_known::Rfc3339).unwrap())?;"
                                },
                                _ => {
                                    "write!(f, \"{item}\")?;"
                                }
                            }
                        },
                        ParameterType::Number |
                            ParameterType::Integer |
                            ParameterType::Boolean => {
                                "write!(f, \"{item}\")?;"
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
        let op_name = query_struct_name(op)?;
        format!(
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
        )
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
