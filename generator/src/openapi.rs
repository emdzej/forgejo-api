use std::collections::{BTreeMap, BTreeSet};

use eyre::WrapErr;
use url::Url;

#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct OpenApiV2 {
    pub swagger: String,
    pub info: SpecInfo,
    pub host: Option<String>,
    pub base_path: Option<String>,
    pub schemes: Option<Vec<String>>,
    pub consumes: Option<Vec<String>>,
    pub produces: Option<Vec<String>>,
    pub paths: BTreeMap<String, PathItem>,
    pub definitions: Option<BTreeMap<String, Schema>>,
    pub parameters: Option<BTreeMap<String, Parameter>>,
    pub responses: Option<BTreeMap<String, Response>>,
    pub security_definitions: Option<BTreeMap<String, SecurityScheme>>,
    pub security: Option<Vec<BTreeMap<String, Vec<String>>>>,
    pub tags: Option<Vec<Tag>>,
    pub external_docs: Option<ExternalDocs>,
}

impl OpenApiV2 {
    pub fn validate(&self) -> eyre::Result<()> {
        eyre::ensure!(self.swagger == "2.0", "swagger version must be 2.0");
        if let Some(host) = &self.host {
            eyre::ensure!(!host.contains("://"), "openapi.host cannot contain scheme");
            eyre::ensure!(!host.contains("/"), "openapi.host cannot contain path");
        }
        if let Some(base_path) = &self.base_path {
            eyre::ensure!(
                base_path.starts_with("/"),
                "openapi.base_path must start with a forward slash"
            );
        }
        if let Some(schemes) = &self.schemes {
            for scheme in schemes {
                eyre::ensure!(
                    matches!(&**scheme, "http" | "https" | "ws" | "wss"),
                    "openapi.schemes must only be http, https, ws, or wss"
                );
            }
        }
        for (path, path_item) in &self.paths {
            eyre::ensure!(
                path.starts_with("/"),
                "members of openapi.paths must start with a forward slash; {path} does not"
            );
            let mut operation_ids = BTreeSet::new();
            path_item
                .validate(&mut operation_ids)
                .wrap_err_with(|| format!("OpenApiV2.paths[\"{path}\"]"))?;
        }
        if let Some(definitions) = &self.definitions {
            for (name, schema) in definitions {
                schema
                    .validate()
                    .wrap_err_with(|| format!("OpenApiV2.definitions[\"{name}\"]"))?;
            }
        }
        if let Some(params) = &self.parameters {
            for (name, param) in params {
                param
                    .validate()
                    .wrap_err_with(|| format!("OpenApiV2.parameters[\"{name}\"]"))?;
            }
        }
        if let Some(responses) = &self.responses {
            for (name, responses) in responses {
                responses
                    .validate()
                    .wrap_err_with(|| format!("OpenApiV2.responses[\"{name}\"]"))?;
            }
        }
        Ok(())
    }
}

#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct SpecInfo {
    pub title: String,
    pub description: Option<String>,
    pub terms_of_service: Option<String>,
    pub contact: Option<Contact>,
    pub license: Option<License>,
    pub version: String,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Contact {
    pub name: Option<String>,
    pub url: Option<String>,
    pub email: Option<String>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct License {
    pub name: String,
    pub url: Option<Url>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct PathItem {
    #[serde(rename = "$ref")]
    pub _ref: Option<String>,
    pub get: Option<Operation>,
    pub put: Option<Operation>,
    pub post: Option<Operation>,
    pub delete: Option<Operation>,
    pub options: Option<Operation>,
    pub head: Option<Operation>,
    pub patch: Option<Operation>,
    pub parameters: Option<Vec<MaybeRef<Parameter>>>,
}

impl PathItem {
    fn validate<'a>(&'a self, ids: &mut BTreeSet<&'a str>) -> eyre::Result<()> {
        if let Some(op) = &self.get {
            op.validate(ids).wrap_err("PathItem.get")?;
        }
        if let Some(op) = &self.put {
            op.validate(ids).wrap_err("PathItem.patch")?;
        }
        if let Some(op) = &self.post {
            op.validate(ids).wrap_err("PathItem.patch")?;
        }
        if let Some(op) = &self.delete {
            op.validate(ids).wrap_err("PathItem.patch")?;
        }
        if let Some(op) = &self.options {
            op.validate(ids).wrap_err("PathItem.patch")?;
        }
        if let Some(op) = &self.head {
            op.validate(ids).wrap_err("PathItem.patch")?;
        }
        if let Some(op) = &self.patch {
            op.validate(ids).wrap_err("PathItem.patch")?;
        }
        if let Some(params) = &self.parameters {
            for param in params {
                if let MaybeRef::Value { value } = param {
                    value.validate().wrap_err("PathItem.parameters")?;
                }
            }
        }
        Ok(())
    }
}

#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Operation {
    pub tags: Option<Vec<String>>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub external_docs: Option<ExternalDocs>,
    pub operation_id: Option<String>,
    pub consumes: Option<Vec<String>>,
    pub produces: Option<Vec<String>>,
    pub parameters: Option<Vec<MaybeRef<Parameter>>>,
    pub responses: Responses,
    pub schemes: Option<Vec<String>>,
    pub deprecated: Option<bool>,
    pub security: Option<Vec<BTreeMap<String, Vec<String>>>>,
}

impl Operation {
    fn validate<'a>(&'a self, ids: &mut BTreeSet<&'a str>) -> eyre::Result<()> {
        if let Some(operation_id) = self.operation_id.as_deref() {
            let is_new = ids.insert(operation_id);
            eyre::ensure!(is_new, "duplicate operation id");
        }
        if let Some(params) = &self.parameters {
            for param in params {
                if let MaybeRef::Value { value } = param {
                    value.validate().wrap_err("Operation.parameters")?;
                }
            }
        }
        self.responses.validate().wrap_err("operation response")?;
        if let Some(schemes) = &self.schemes {
            for scheme in schemes {
                eyre::ensure!(
                    matches!(&**scheme, "http" | "https" | "ws" | "wss"),
                    "openapi.schemes must only be http, https, ws, or wss"
                );
            }
        }
        Ok(())
    }
}

#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ExternalDocs {
    pub description: Option<String>,
    pub url: Url,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Parameter {
    pub name: String,
    pub description: Option<String>,
    #[serde(flatten)]
    pub _in: ParameterIn,
}

impl Parameter {
    fn validate(&self) -> eyre::Result<()> {
        self._in.validate()
    }
}

#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(rename_all(deserialize = "camelCase"))]
#[serde(tag = "in")]
pub enum ParameterIn {
    Body {
        schema: MaybeRef<Schema>,
    },
    Path {
        #[serde(flatten)]
        param: NonBodyParameter,
    },
    Query {
        #[serde(flatten)]
        param: NonBodyParameter,
    },
    Header {
        #[serde(flatten)]
        param: NonBodyParameter,
    },
    FormData {
        #[serde(flatten)]
        param: NonBodyParameter,
    },
}

impl ParameterIn {
    fn validate(&self) -> eyre::Result<()> {
        match self {
            ParameterIn::Path { param } => {
                eyre::ensure!(param.required, "path parameters must be required");
                param.validate().wrap_err("path param")
            }
            ParameterIn::Query { param } => param.validate().wrap_err("query param"),
            ParameterIn::Header { param } => param.validate().wrap_err("header param"),
            ParameterIn::Body { schema } => {
                if let MaybeRef::Value { value } = schema {
                    value.validate().wrap_err("body param")
                } else {
                    Ok(())
                }
            }
            ParameterIn::FormData { param } => param.validate().wrap_err("form param"),
        }
    }
}

#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct NonBodyParameter {
    #[serde(default)]
    pub required: bool,
    #[serde(rename = "type")]
    pub _type: ParameterType,
    pub format: Option<String>,
    pub allow_empty_value: Option<bool>,
    pub items: Option<Items>,
    pub collection_format: Option<CollectionFormat>,
    pub default: Option<serde_json::Value>,
    pub maximum: Option<f64>,
    pub exclusive_maximum: Option<bool>,
    pub minimum: Option<f64>,
    pub exclusive_minimum: Option<bool>,
    pub max_length: Option<u64>,
    pub min_length: Option<u64>,
    pub pattern: Option<String>, // should be regex
    pub max_items: Option<u64>,
    pub min_items: Option<u64>,
    pub unique_items: Option<bool>,
    #[serde(rename = "enum")]
    pub _enum: Option<Vec<serde_json::Value>>,
    pub multiple_of: Option<u64>,
}

impl NonBodyParameter {
    fn validate(&self) -> eyre::Result<()> {
        if self._type == ParameterType::Array {
            eyre::ensure!(
                self.items.is_some(),
                "array paramters must define their item types"
            );
        }
        if let Some(items) = &self.items {
            items.validate()?;
        }
        if let Some(default) = &self.default {
            eyre::ensure!(
                self._type.matches_value(default),
                "param's default must match its type"
            );
        }
        if let Some(_enum) = &self._enum {
            for variant in _enum {
                eyre::ensure!(
                    self._type.matches_value(variant),
                    "header enum variant must match its type"
                );
            }
        }
        if self.exclusive_maximum.is_some() {
            eyre::ensure!(
                self.maximum.is_some(),
                "presence of `exclusiveMaximum` requires `maximum` be there too"
            );
        }
        if self.exclusive_minimum.is_some() {
            eyre::ensure!(
                self.minimum.is_some(),
                "presence of `exclusiveMinimum` requires `minimum` be there too"
            );
        }
        if let Some(multiple_of) = self.multiple_of {
            eyre::ensure!(multiple_of > 0, "multipleOf must be greater than 0");
        }
        Ok(())
    }
}

#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(rename_all(deserialize = "camelCase"))]
pub enum ParameterType {
    String,
    Number,
    Integer,
    Boolean,
    Array,
    File,
}

impl ParameterType {
    fn matches_value(&self, value: &serde_json::Value) -> bool {
        match (self, value) {
            (ParameterType::String, serde_json::Value::String(_))
            | (ParameterType::Number, serde_json::Value::Number(_))
            | (ParameterType::Integer, serde_json::Value::Number(_))
            | (ParameterType::Boolean, serde_json::Value::Bool(_))
            | (ParameterType::Array, serde_json::Value::Array(_)) => true,
            _ => false,
        }
    }
}

#[derive(serde::Deserialize, Debug, PartialEq, Clone, Copy)]
#[serde(rename_all(deserialize = "camelCase"))]
pub enum CollectionFormat {
    Csv,
    Ssv,
    Tsv,
    Pipes,
    Multi,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Items {
    #[serde(rename = "type")]
    pub _type: ParameterType,
    pub format: Option<String>,
    pub items: Option<Box<Items>>,
    pub collection_format: Option<CollectionFormat>,
    pub default: Option<serde_json::Value>,
    pub maximum: Option<f64>,
    pub exclusive_maximum: Option<bool>,
    pub minimum: Option<f64>,
    pub exclusive_minimum: Option<bool>,
    pub max_length: Option<u64>,
    pub min_length: Option<u64>,
    pub pattern: Option<String>, // should be regex
    pub max_items: Option<u64>,
    pub min_items: Option<u64>,
    pub unique_items: Option<bool>,
    #[serde(rename = "enum")]
    pub _enum: Option<Vec<serde_json::Value>>,
    pub multiple_of: Option<u64>,
}

impl Items {
    fn validate(&self) -> eyre::Result<()> {
        if self._type == ParameterType::Array {
            eyre::ensure!(
                self.items.is_some(),
                "array paramters must define their item types"
            );
        }
        if let Some(items) = &self.items {
            items.validate()?;
        }
        if let Some(default) = &self.default {
            match (&self._type, default) {
                (ParameterType::String, serde_json::Value::String(_))
                | (ParameterType::Number, serde_json::Value::Number(_))
                | (ParameterType::Integer, serde_json::Value::Number(_))
                | (ParameterType::Boolean, serde_json::Value::Bool(_))
                | (ParameterType::Array, serde_json::Value::Array(_)) => (),
                (ParameterType::File, _) => eyre::bail!("file params cannot have default value"),
                _ => eyre::bail!("param's default must match its type"),
            };
        }
        if let Some(_enum) = &self._enum {
            for variant in _enum {
                eyre::ensure!(
                    self._type.matches_value(variant),
                    "header enum variant must match its type"
                );
            }
        }
        if self.exclusive_maximum.is_some() {
            eyre::ensure!(
                self.maximum.is_some(),
                "presence of `exclusiveMaximum` requires `maximum` be there too"
            );
        }
        if self.exclusive_minimum.is_some() {
            eyre::ensure!(
                self.minimum.is_some(),
                "presence of `exclusiveMinimum` requires `minimum` be there too"
            );
        }
        if let Some(multiple_of) = self.multiple_of {
            eyre::ensure!(multiple_of > 0, "multipleOf must be greater than 0");
        }
        Ok(())
    }
}

#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Responses {
    pub default: Option<MaybeRef<Response>>,
    #[serde(flatten)]
    pub http_codes: BTreeMap<String, MaybeRef<Response>>,
}

impl Responses {
    fn validate(&self) -> eyre::Result<()> {
        if self.default.is_none() && self.http_codes.is_empty() {
            eyre::bail!("must have at least one response");
        }
        if let Some(MaybeRef::Value { value }) = &self.default {
            value.validate().wrap_err("default response")?;
        }
        for (code, response) in &self.http_codes {
            let code_int = code.parse::<u16>().wrap_err("http code must be a number")?;
            eyre::ensure!(
                code_int >= 100 && code_int < 1000,
                "invalid http status code"
            );
            if let MaybeRef::Value { value } = response {
                value.validate().wrap_err_with(|| code.to_string())?;
            }
        }
        Ok(())
    }
}

#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Response {
    pub description: String,
    pub schema: Option<MaybeRef<Schema>>,
    pub headers: Option<BTreeMap<String, Header>>,
    pub examples: Option<BTreeMap<String, serde_json::Value>>,
}

impl Response {
    fn validate(&self) -> eyre::Result<()> {
        if let Some(headers) = &self.headers {
            for (_, value) in headers {
                value.validate().wrap_err("response header")?;
            }
        }
        if let Some(MaybeRef::Value { value }) = &self.schema {
            value.validate().wrap_err("response")?;
        }
        Ok(())
    }
}

#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Header {
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub _type: ParameterType,
    pub format: Option<String>,
    pub items: Option<Items>,
    pub collection_format: Option<CollectionFormat>,
    pub default: Option<serde_json::Value>,
    pub maximum: Option<f64>,
    pub exclusive_maximum: Option<bool>,
    pub minimum: Option<f64>,
    pub exclusive_minimum: Option<bool>,
    pub max_length: Option<u64>,
    pub min_length: Option<u64>,
    pub pattern: Option<String>, // should be regex
    pub max_items: Option<u64>,
    pub min_items: Option<u64>,
    pub unique_items: Option<bool>,
    #[serde(rename = "enum")]
    pub _enum: Option<Vec<serde_json::Value>>,
    pub multiple_of: Option<u64>,
}

impl Header {
    fn validate(&self) -> eyre::Result<()> {
        if self._type == ParameterType::Array {
            eyre::ensure!(
                self.items.is_some(),
                "array paramters must define their item types"
            );
        }
        if let Some(default) = &self.default {
            match (&self._type, default) {
                (ParameterType::String, serde_json::Value::String(_))
                | (ParameterType::Number, serde_json::Value::Number(_))
                | (ParameterType::Integer, serde_json::Value::Number(_))
                | (ParameterType::Boolean, serde_json::Value::Bool(_))
                | (ParameterType::Array, serde_json::Value::Array(_)) => (),
                (ParameterType::File, _) => eyre::bail!("file params cannot have default value"),
                _ => eyre::bail!("param's default must match its type"),
            };
        }
        if let Some(_enum) = &self._enum {
            for variant in _enum {
                eyre::ensure!(
                    self._type.matches_value(variant),
                    "header enum variant must match its type"
                );
            }
        }
        if self.exclusive_maximum.is_some() {
            eyre::ensure!(
                self.maximum.is_some(),
                "presence of `exclusiveMaximum` requires `maximum` be there too"
            );
        }
        if self.exclusive_minimum.is_some() {
            eyre::ensure!(
                self.minimum.is_some(),
                "presence of `exclusiveMinimum` requires `minimum` be there too"
            );
        }
        if let Some(multiple_of) = self.multiple_of {
            eyre::ensure!(multiple_of > 0, "multipleOf must be greater than 0");
        }
        Ok(())
    }
}

#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Tag {
    pub name: String,
    pub description: Option<String>,
    pub external_docs: Option<ExternalDocs>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Schema {
    pub format: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub default: Option<serde_json::Value>,
    pub multiple_of: Option<u64>,
    pub maximum: Option<f64>,
    pub exclusive_maximum: Option<bool>,
    pub minimum: Option<f64>,
    pub exclusive_minimum: Option<bool>,
    pub max_length: Option<u64>,
    pub min_length: Option<u64>,
    pub pattern: Option<String>, // should be regex
    pub max_items: Option<u64>,
    pub min_items: Option<u64>,
    pub unique_items: Option<bool>,
    pub max_properties: Option<u64>,
    pub min_properties: Option<u64>,
    pub required: Option<Vec<String>>,
    #[serde(rename = "enum")]
    pub _enum: Option<Vec<serde_json::Value>>,
    #[serde(rename = "type")]
    pub _type: Option<SchemaType>,
    pub properties: Option<BTreeMap<String, MaybeRef<Schema>>>,
    pub additional_properties: Option<Box<MaybeRef<Schema>>>,
    pub items: Option<Box<MaybeRef<Schema>>>,

    pub discriminator: Option<String>,
    pub read_only: Option<bool>,
    pub xml: Option<Xml>,
    pub external_docs: Option<ExternalDocs>,
    pub example: Option<serde_json::Value>,
}

impl Schema {
    fn validate(&self) -> eyre::Result<()> {
        if let Some(_type) = &self._type {
            match _type {
                SchemaType::One(_type) => {
                    if _type == &Primitive::Array {
                        eyre::ensure!(
                            self.items.is_some(),
                            "array paramters must define their item types"
                        );
                    }
                    if let Some(default) = &self.default {
                        eyre::ensure!(
                            _type.matches_value(default),
                            "param's default must match its type"
                        );
                    }
                    if let Some(_enum) = &self._enum {
                        for variant in _enum {
                            eyre::ensure!(
                                _type.matches_value(variant),
                                "schema enum variant must match its type"
                            );
                        }
                    }
                }
                SchemaType::List(_) => {
                    eyre::bail!("sum types not supported");
                }
            }
        } else {
            eyre::ensure!(
                self.default.is_none(),
                "cannot have default when no type is specified"
            );
        }
        if let Some(items) = &self.items {
            if let MaybeRef::Value { value } = &**items {
                value.validate()?;
            }
        }
        if let Some(required) = &self.required {
            let properties = self.properties.as_ref().ok_or_else(|| {
                eyre::eyre!("required properties listed but no properties present")
            })?;
            for i in required {
                eyre::ensure!(
                    properties.contains_key(i),
                    "property \"{i}\" required, but is not defined"
                );
            }
        }
        if let Some(properties) = &self.properties {
            for (_, schema) in properties {
                if let MaybeRef::Value { value } = schema {
                    value.validate().wrap_err("schema properties")?;
                }
            }
        }
        if let Some(additional_properties) = &self.additional_properties {
            if let MaybeRef::Value { value } = &**additional_properties {
                value.validate().wrap_err("schema additional properties")?;
            }
        }
        if self.exclusive_maximum.is_some() {
            eyre::ensure!(
                self.maximum.is_some(),
                "presence of `exclusiveMaximum` requires `maximum` be there too"
            );
        }
        if self.exclusive_minimum.is_some() {
            eyre::ensure!(
                self.minimum.is_some(),
                "presence of `exclusiveMinimum` requires `minimum` be there too"
            );
        }
        if let Some(multiple_of) = self.multiple_of {
            eyre::ensure!(multiple_of > 0, "multipleOf must be greater than 0");
        }
        Ok(())
    }
}

#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum SchemaType {
    One(Primitive),
    List(Vec<Primitive>),
}

#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(rename_all(deserialize = "camelCase"))]
pub enum Primitive {
    Array,
    Boolean,
    Integer,
    Number,
    Null,
    Object,
    String,
}

impl Primitive {
    fn matches_value(&self, value: &serde_json::Value) -> bool {
        match (self, value) {
            (Primitive::String, serde_json::Value::String(_))
            | (Primitive::Number, serde_json::Value::Number(_))
            | (Primitive::Integer, serde_json::Value::Number(_))
            | (Primitive::Boolean, serde_json::Value::Bool(_))
            | (Primitive::Array, serde_json::Value::Array(_)) => true,
            _ => false,
        }
    }
}

#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Xml {
    pub name: Option<String>,
    pub namespace: Option<Url>,
    pub prefix: Option<String>,
    pub attribute: Option<bool>,
    pub wrapped: Option<bool>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct SecurityScheme {
    #[serde(flatten)]
    pub _type: SecurityType,
    pub description: Option<String>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(rename_all(deserialize = "camelCase"), tag = "type")]
pub enum SecurityType {
    Basic,
    ApiKey {
        name: String,
        #[serde(rename = "in")]
        _in: KeyIn,
    },
    OAuth2 {
        #[serde(flatten)]
        flow: OAuth2Flow,
        scopes: BTreeMap<String, String>,
    },
}

#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(rename_all(deserialize = "camelCase"))]
pub enum KeyIn {
    Query,
    Header,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(rename_all(deserialize = "camelCase"), tag = "flow")]
pub enum OAuth2Flow {
    Implicit {
        authorization_url: Url,
    },
    Password {
        token_url: Url,
    },
    Application {
        token_url: Url,
    },
    AccessCode {
        authorization_url: Url,
        token_url: Url,
    },
}

#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum MaybeRef<T> {
    Ref {
        #[serde(rename = "$ref")]
        _ref: String,
    },
    Value {
        #[serde(flatten)]
        value: T,
    },
}
