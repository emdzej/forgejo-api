use crate::openapi::*;
use eyre::{OptionExt, WrapErr};
use heck::ToPascalCase;
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write,
};

pub fn create_structs(spec: &OpenApiV2) -> eyre::Result<String> {
    let mut s = String::new();
    s.push_str("use crate::{StructureError, impl_from_response};");
    s.push_str("use std::collections::BTreeMap;");
    let response_types = get_response_types(spec)?;
    if let Some(definitions) = &spec.definitions {
        for (name, schema) in definitions {
            let strukt =
                create_struct_for_definition(spec, name, schema, response_types.contains(name))?;
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
    for item in spec.paths.values() {
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
    is_response: bool,
) -> eyre::Result<String> {
    match schema {
        Schema {
            _type: Some(SchemaType::One(Primitive::String)),
            _enum: Some(_enum),
            ..
        } => create_enum(
            name,
            schema.description.as_deref(),
            _enum,
            false,
            is_response,
        ),
        Schema {
            _type: Some(SchemaType::One(Primitive::Object)),
            ..
        } => create_object_struct(spec, name, schema, is_response),
        _ => Ok(String::new()),
    }
}

pub fn create_object_struct(
    spec: &OpenApiV2,
    name: &str,
    schema: &Schema,
    is_response: bool,
) -> eyre::Result<String> {
    let mut subtypes = Vec::new();

    let parse_with = schema
        .extensions
        .get("x-parse-with")
        .map(|ex| serde_json::from_value::<BTreeMap<String, String>>(ex.clone()))
        .transpose()?
        .unwrap_or_default();

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
                crate::schema_subtypes(spec, name, prop_name, value, &mut subtypes, false)?;
            }
            let is_required = required.contains(prop_name);
            if let Some(format) = &prop_schema.deref(spec)?.format {
                match &**format {
                    "url" => {
                        if !is_required {
                            fields.push_str(
                                "#[serde(deserialize_with = \"crate::none_if_blank_url\")]\n",
                            );
                        }
                    }
                    "ssh-url" => {
                        if is_required {
                            fields.push_str(
                                "#[serde(deserialize_with = \"crate::deserialize_ssh_url\")]\n",
                            );
                        } else {
                            fields.push_str(
                                "#[serde(deserialize_with = \"crate::deserialize_optional_ssh_url\")]\n",
                            );
                        }
                    }
                    "date-time" => {
                        if is_required {
                            fields.push_str("#[serde(with = \"time::serde::rfc3339\")]\n");
                        } else {
                            fields.push_str("#[serde(with = \"time::serde::rfc3339::option\")]\n");
                        }
                    }
                    "date" => {
                        if is_required {
                            fields.push_str("#[serde(with = \"time::serde::rfc3339\")]\n");
                        } else {
                            fields.push_str("#[serde(with = \"time::serde::rfc3339::option\")]\n");
                        }
                    }
                    _ => (),
                }
            }
            if field_ty == name {
                field_ty = format!("Box<{field_ty}>")
            }
            if !is_required {
                field_ty = format!("Option<{field_ty}>")
            }
            if let Some(parse_method) = parse_with.get(prop_name) {
                fields.push_str("#[serde(deserialize_with = \"crate::");
                fields.push_str(parse_method);
                fields.push_str("\")]\n");
            }
            if let MaybeRef::Value { value } = &prop_schema {
                if let Some(desc) = &value.description {
                    let desc = desc.replace("gitea", "Forgejo").replace("Gitea", "Forgejo");
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

    if let Some(additional_schema) = &schema.additional_properties {
        let prop_ty = crate::schema_ref_type_name(spec, additional_schema)?;
        fields.push_str("#[serde(flatten)]\n");
        fields.push_str("pub additional: BTreeMap<String, ");
        fields.push_str(&prop_ty);
        fields.push_str(">,\n");
    }

    let mut out = format!("{docs}#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]\npub struct {name} {{\n{fields}}}\n\n");

    if is_response {
        out.push_str(&impl_from_response(name));
    }

    if let Some(pagination_field) = schema
        .extensions
        .get("x-pagination-field")
        .and_then(|v| v.as_str())
    {
        writeln!(
            &mut out,
            "\
impl crate::sealed::Sealed for {name} {{}}
impl crate::PageSize for {name} {{
    fn page_size(&self) -> usize {{
        "
        )?;
        if required.iter().any(|field| field == pagination_field) {
            writeln!(
                &mut out,
                "\
        self.{pagination_field}.page_size()
            "
            )?;
        } else {
            writeln!(
                &mut out,
                "\
        self.{pagination_field}.as_ref().map(|x| x.page_size()).unwrap_or(0)
            "
            )?;
        }
        writeln!(
            &mut out,
            "\
    }}
}}
\
        "
        )?;
    }
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
    is_response: bool,
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
    imp.push('}');

    let strukt = format!(
        "
{docs}
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum {name} {{
    {variants}
}}"
    );
    let mut out = if imp_as_str {
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

    if is_response {
        out.push_str(&impl_from_response(name));
    }
    Ok(out)
}

fn impl_from_response(name: &str) -> String {
    format!("impl_from_response!({name});\n\n")
}

fn create_struct_docs(schema: &Schema) -> eyre::Result<String> {
    create_struct_docs_str(schema.description.as_deref())
}

fn create_struct_docs_str(description: Option<&str>) -> eyre::Result<String> {
    let doc = match description {
        Some(desc) => {
            let desc = desc.replace("gitea", "Forgejo").replace("Gitea", "Forgejo");
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
    let mut to_list_imp = String::new();
    // only derive default if every field is optional
    let mut can_derive_default = true;
    for param in params {
        let param = param.deref(spec)?;
        if matches!(&*param.name, "page" | "limit") {
            continue;
        }
        if let ParameterIn::Query { param: query_param } = &param._in {
            let field_name = crate::sanitize_ident(&param.name);
            let ty = match &query_param {
                NonBodyParameter {
                    _type: ParameterType::String,
                    _enum: Some(_enum),
                    ..
                } => {
                    let name = format!("{op_name}{}", param.name.to_pascal_case());
                    let enum_def = create_enum(&name, None, _enum, true, false)?;
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
                    let enum_def = create_enum(&name, None, _enum, true, false)?;
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
                can_derive_default = false;
                fields.push_str(&ty);
            } else {
                fields.push_str("Option<");
                fields.push_str(&ty);
                fields.push('>');
            }
            fields.push_str(",\n");

            let mut handler = String::new();
            if query_param.required {
                writeln!(&mut handler, "let {field_name} = self.{field_name};")?;
            } else {
                writeln!(
                    &mut handler,
                    "if let Some({field_name}) = self.{field_name} {{"
                )?;
            }
            match &query_param._type {
                ParameterType::String => {
                    if let Some(_enum) = &query_param._enum {
                        writeln!(
                            &mut handler,
                            "list.push((\"{}\", {}.as_str().to_string()));",
                            param.name, field_name,
                        )?;
                    } else {
                        match query_param.format.as_deref() {
                            Some("date-time" | "date") => {
                                writeln!(
                                    &mut handler,
                                    "list.push((\"{}\", {field_name}.format(&time::format_description::well_known::Rfc3339).unwrap()));",
                                    param.name)?;
                            }
                            _ => {
                                writeln!(
                                    &mut handler,
                                    "list.push((\"{}\", {}));",
                                    param.name, field_name,
                                )?;
                            }
                        }
                    }
                }
                ParameterType::Number | ParameterType::Integer | ParameterType::Boolean => {
                    writeln!(
                        &mut handler,
                        "list.push((\"{}\", {}.to_string()));",
                        param.name, field_name,
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
                    let item_mapper = match item._type {
                        ParameterType::String => {
                            if let Some(_enum) = &item._enum {
                                "item.as_str()"
                            } else {
                                match query_param.format.as_deref() {
                                    Some("date-time" | "date") => {
                                        "&item.format(&time::format_description::well_known::Rfc3339).unwrap()"
                                    },
                                    _ => {
                                        "item"
                                    }
                                }
                            }
                        }
                        ParameterType::Number | ParameterType::Integer | ParameterType::Boolean => {
                            "&format!(\"{item}\")"
                        }
                        ParameterType::Array => {
                            eyre::bail!("nested arrays not supported in query");
                        }
                        ParameterType::File => eyre::bail!("cannot send file in query"),
                    };
                    match format {
                        CollectionFormat::Csv => {
                            handler.push_str(&simple_query_array_as_list(
                                param,
                                item_mapper,
                                &field_name,
                                ",",
                            )?);
                        }
                        CollectionFormat::Ssv => {
                            handler.push_str(&simple_query_array_as_list(
                                param,
                                item_mapper,
                                &field_name,
                                " ",
                            )?);
                        }
                        CollectionFormat::Tsv => {
                            handler.push_str(&simple_query_array_as_list(
                                param,
                                item_mapper,
                                &field_name,
                                "\\t",
                            )?);
                        }
                        CollectionFormat::Pipes => {
                            handler.push_str(&simple_query_array_as_list(
                                param,
                                item_mapper,
                                &field_name,
                                "|",
                            )?);
                        }
                        CollectionFormat::Multi => {
                            let item_to_string = match item._type {
                                ParameterType::String => {
                                    if let Some(_enum) = &item._enum {
                                        "item.as_str().to_string()"
                                    } else {
                                        match query_param.format.as_deref() {
                                            Some("date-time" | "date") => {
                                                "item.format(&time::format_description::well_known::Rfc3339).unwrap()"
                                            },
                                            _ => {
                                                "item.to_string()"
                                            }
                                        }
                                    }
                                }
                                ParameterType::Number
                                | ParameterType::Integer
                                | ParameterType::Boolean => "format!(\"{item}\")",
                                ParameterType::Array => {
                                    eyre::bail!("nested arrays not supported in query");
                                }
                                ParameterType::File => eyre::bail!("cannot send file in query"),
                            };
                            writeln!(&mut handler)?;
                            writeln!(&mut handler, "if !{field_name}.is_empty() {{")?;
                            writeln!(&mut handler, "for item in {field_name} {{")?;
                            writeln!(
                                &mut handler,
                                "list.push((\"{}\", {}));",
                                param.name, item_to_string
                            )?;
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
            to_list_imp.push_str(&handler);
        }
    }

    let derives = if can_derive_default {
        "Debug, Clone, PartialEq, Default"
    } else {
        "Debug, Clone, PartialEq"
    };

    let result = if fields.is_empty() {
        String::new()
    } else {
        let mut out = format!(
            "
#[derive({derives})]
pub struct {op_name} {{
    {fields}
}}

impl {op_name} {{
    pub(crate) fn into_list(self) -> Vec<(&'static str, String)> {{
        let mut list = Vec::new();
        {to_list_imp}
        list
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

fn simple_query_array_as_list(
    param: &Parameter,
    item_mapper: &str,
    name: &str,
    sep: &str,
) -> eyre::Result<String> {
    let mut out = String::new();

    writeln!(
        &mut out,
        "
if !{name}.is_empty() {{
    let mut s = String::new();
    for (i, item) in {name}.iter().enumerate() {{
        s.push_str({item_mapper});
        if i < {name}.len() - 1 {{
            s.push('{sep}')
        }}
    }}
    list.push((\"{}\", s));
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
            let {field_name} = map.get(\"{}\").map(|s| -> Result<_, _> {{
                let s = s.to_str().map_err(|_| StructureError::HeaderNotAscii)?;
        ",
            header_name.to_lowercase(),
        )?;
        match &header._type {
            ParameterType::String => match header.format.as_deref() {
                Some("url" | "ssh-url") => imp.push_str(
                    "s.parse::<url::Url>().map_err(|_| StructureError::HeaderParseFailed)",
                ),
                _ => imp.push_str("Ok::<_, StructureError>(s.to_string())"),
            },
            ParameterType::Number => match header.format.as_deref() {
                Some("float") => {
                    imp.push_str("s.parse::<f32>().map_err(|_| StructureError::HeaderParseFailed)")
                }
                Some("double") | _ => {
                    imp.push_str("s.parse::<f64>()).map_err(|_| StructureError::HeaderParseFailed)")
                }
            },
            ParameterType::Integer => match header.format.as_deref() {
                Some("uint64") => {
                    imp.push_str("s.parse::<u64>().map_err(|_| StructureError::HeaderParseFailed)")
                }
                Some("uint32") => {
                    imp.push_str("s.parse::<u32>().map_err(|_| StructureError::HeaderParseFailed)")
                }
                Some("int64") => {
                    imp.push_str("s.parse::<i64>().map_err(|_| StructureError::HeaderParseFailed)")
                }
                Some("int32") | _ => {
                    imp.push_str("s.parse::<i32>().map_err(|_| StructureError::HeaderParseFailed)")
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
                        Some("uint64") => "s.parse::<u64>()).collect::<Result<Vec<_>, _>>().map_err(|_| StructureError::HeaderParseFailed)",
                        Some("uint32") => "s.parse::<u32>()).collect::<Result<Vec<_>, _>>().map_err(|_| StructureError::HeaderParseFailed)",
                        Some("int64") => "s.parse::<i64>()).collect::<Result<Vec<_>, _>>().map_err(|_| StructureError::HeaderParseFailed)",
                        Some("int32") | _ => "s.parse::<i32>()).collect::<Result<Vec<_>, _>>().map_err(|_| StructureError::HeaderParseFailed)",
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

    let count_impl = if headers.contains_key("X-Total-Count") {
        format!(
            "
impl crate::sealed::Sealed for {ty_name} {{}}
impl crate::CountHeader for {ty_name} {{
    fn count(&self) -> Option<usize> {{
        self.x_total_count.map(|x| x as usize)
    }}
}}
"
        )
    } else {
        String::new()
    };

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

{count_impl}
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
    for response in op.responses.http_codes.values() {
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
        crate::schema_subtypes(spec, name, "Response", value, &mut types, true)?;
    }
    let mut out = String::new();
    for ty in types {
        out.push_str(&ty);
    }
    Ok(out)
}

fn get_response_types(spec: &OpenApiV2) -> eyre::Result<BTreeSet<String>> {
    let mut set = BTreeSet::new();
    for item in spec.paths.values() {
        if let Some(op) = &item.get {
            get_response_types_for_op(spec, op, &mut set)?
        }
        if let Some(op) = &item.put {
            get_response_types_for_op(spec, op, &mut set)?
        }
        if let Some(op) = &item.post {
            get_response_types_for_op(spec, op, &mut set)?
        }
        if let Some(op) = &item.delete {
            get_response_types_for_op(spec, op, &mut set)?
        }
        if let Some(op) = &item.options {
            get_response_types_for_op(spec, op, &mut set)?
        }
        if let Some(op) = &item.head {
            get_response_types_for_op(spec, op, &mut set)?
        }
        if let Some(op) = &item.patch {
            get_response_types_for_op(spec, op, &mut set)?
        }
    }
    Ok(set)
}

fn get_response_types_for_op(
    spec: &OpenApiV2,
    op: &Operation,
    set: &mut BTreeSet<String>,
) -> eyre::Result<()> {
    for (_, response) in &op.responses.http_codes {
        if let Some(schema) = &response.deref(spec)?.schema {
            let schema = match schema.deref(spec)? {
                Schema {
                    _type: Some(SchemaType::One(Primitive::Object)),
                    ..
                }
                | Schema {
                    _type: Some(SchemaType::One(Primitive::String)),
                    _enum: Some(_),
                    ..
                } => Some(schema),
                Schema {
                    _type: Some(SchemaType::One(Primitive::Array)),
                    items: Some(items),
                    ..
                } => Some(items.as_ref()),
                _ => None,
            };
            if let Some(schema_ref) = &schema {
                let schema = schema_ref.deref(spec)?;
                let name = schema
                    .title
                    .as_deref()
                    .map(|title| title.split_whitespace().next().unwrap_or(title).to_owned())
                    .or_else(|| match schema_ref {
                        MaybeRef::Ref { _ref } => Some(_ref.rsplit_once("/")?.1.to_owned()),
                        MaybeRef::Value { .. } => None,
                    })
                    .or_else(|| Some(format!("{}Response", op.operation_id.as_deref()?)))
                    .ok_or_eyre("no name found")?;
                set.insert(name);
            }
        }
    }
    Ok(())
}
