use std::collections::{BTreeMap, BTreeSet};

use eyre::WrapErr;
use url::Url;

trait JsonDeref: std::any::Any {
    fn deref_any<'a>(&'a self, path: &str) -> eyre::Result<&'a dyn std::any::Any>;
}

impl JsonDeref for bool {
    fn deref_any<'a>(&'a self, path: &str) -> eyre::Result<&'a dyn std::any::Any> {
        if path.is_empty() {
            Ok(self)
        } else {
            Err(eyre::eyre!("not found"))
        }
    }
}

impl JsonDeref for u64 {
    fn deref_any<'a>(&'a self, path: &str) -> eyre::Result<&'a dyn std::any::Any> {
        if path.is_empty() {
            Ok(self)
        } else {
            Err(eyre::eyre!("not found"))
        }
    }
}

impl JsonDeref for f64 {
    fn deref_any<'a>(&'a self, path: &str) -> eyre::Result<&'a dyn std::any::Any> {
        if path.is_empty() {
            Ok(self)
        } else {
            Err(eyre::eyre!("not found"))
        }
    }
}

impl JsonDeref for String {
    fn deref_any<'a>(&'a self, path: &str) -> eyre::Result<&'a dyn std::any::Any> {
        if path.is_empty() {
            Ok(self)
        } else {
            Err(eyre::eyre!("not found"))
        }
    }
}

impl JsonDeref for Url {
    fn deref_any<'a>(&'a self, path: &str) -> eyre::Result<&'a dyn std::any::Any> {
        if path.is_empty() {
            Ok(self)
        } else {
            Err(eyre::eyre!("not found"))
        }
    }
}

impl<T: JsonDeref> JsonDeref for Option<T> {
    fn deref_any<'a>(&'a self, path: &str) -> eyre::Result<&'a dyn std::any::Any> {
        match self {
            Some(x) => x.deref_any(path),
            None => Err(eyre::eyre!("not found")),
        }
    }
}

impl<T: JsonDeref> JsonDeref for Box<T> {
    fn deref_any<'a>(&'a self, path: &str) -> eyre::Result<&'a dyn std::any::Any> {
        T::deref_any(&**self, path)
    }
}

impl<T: JsonDeref> JsonDeref for Vec<T> {
    fn deref_any(&self, path: &str) -> eyre::Result<&dyn std::any::Any> {
        if path.is_empty() {
            return Ok(self);
        }
        let (head, tail) = path.split_once("/").unwrap_or((path, ""));
        let idx = head.parse::<usize>().wrap_err("not found")?;
        let value = self.get(idx).ok_or_else(|| eyre::eyre!("not found"))?;
        value.deref_any(tail)
    }
}

impl<T: JsonDeref> JsonDeref for BTreeMap<String, T> {
    fn deref_any(&self, path: &str) -> eyre::Result<&dyn std::any::Any> {
        if path.is_empty() {
            return Ok(self);
        }
        let (head, tail) = path.split_once("/").unwrap_or((path, ""));
        let value = self.get(head).ok_or_else(|| eyre::eyre!("not found"))?;
        value.deref_any(tail)
    }
}

impl JsonDeref for serde_json::Value {
    fn deref_any<'a>(&'a self, path: &str) -> eyre::Result<&'a dyn std::any::Any> {
        match self {
            serde_json::Value::Null => eyre::bail!("not found"),
            serde_json::Value::Bool(b) => b.deref_any(path),
            serde_json::Value::Number(x) => x.deref_any(path),
            serde_json::Value::String(s) => s.deref_any(path),
            serde_json::Value::Array(list) => list.deref_any(path),
            serde_json::Value::Object(map) => map.deref_any(path),
        }
    }
}

impl JsonDeref for serde_json::Map<String, serde_json::Value> {
    fn deref_any(&self, path: &str) -> eyre::Result<&dyn std::any::Any> {
        if path.is_empty() {
            return Ok(self);
        }
        let (head, tail) = path.split_once("/").unwrap_or((path, ""));
        let value = self.get(head).ok_or_else(|| eyre::eyre!("not found"))?;
        value.deref_any(tail)
    }
}

impl JsonDeref for serde_json::Number {
    fn deref_any<'a>(&'a self, path: &str) -> eyre::Result<&'a dyn std::any::Any> {
        if path.is_empty() {
            Ok(self)
        } else {
            eyre::bail!("not found")
        }
    }
}

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

impl JsonDeref for OpenApiV2 {
    fn deref_any(&self, path: &str) -> eyre::Result<&dyn std::any::Any> {
        let path = path
            .strip_prefix("#/")
            .ok_or_else(|| eyre::eyre!("invalid ref prefix"))?;
        if path.is_empty() {
            return Ok(self);
        }
        let (head, tail) = path.split_once("/").unwrap_or((path, ""));
        match head {
            "swagger" => self.swagger.deref_any(tail),
            "info" => self.info.deref_any(tail),
            "host" => self.host.deref_any(tail),
            "base_path" => self.base_path.deref_any(tail),
            "schemes" => self.schemes.deref_any(tail),
            "consumes" => self.consumes.deref_any(tail),
            "produces" => self.produces.deref_any(tail),
            "paths" => self.paths.deref_any(tail),
            "definitions" => self.definitions.deref_any(tail),
            "parameters" => self.parameters.deref_any(tail),
            "responses" => self.responses.deref_any(tail),
            "security_definitions" => self.security_definitions.deref_any(tail),
            "security" => self.security.deref_any(tail),
            "tags" => self.tags.deref_any(tail),
            "external_docs" => self.external_docs.deref_any(tail),
            _ => eyre::bail!("not found: {head}"),
        }
    }
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

    pub fn deref<T: std::any::Any>(&self, path: &str) -> eyre::Result<&T> {
        self.deref_any(path).and_then(|a| {
            a.downcast_ref::<T>()
                .ok_or_else(|| eyre::eyre!("incorrect type found at reference"))
        })
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

impl JsonDeref for SpecInfo {
    fn deref_any(&self, path: &str) -> eyre::Result<&dyn std::any::Any> {
        if path.is_empty() {
            return Ok(self);
        }
        let (head, tail) = path.split_once("/").unwrap_or((path, ""));
        match head {
            "title" => self.title.deref_any(tail),
            "description" => self.description.deref_any(tail),
            "terms_of_service" => self.terms_of_service.deref_any(tail),
            "contact" => self.contact.deref_any(tail),
            "license" => self.license.deref_any(tail),
            "version" => self.version.deref_any(tail),
            _ => eyre::bail!("not found: {head}"),
        }
    }
}

#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Contact {
    pub name: Option<String>,
    pub url: Option<String>,
    pub email: Option<String>,
}

impl JsonDeref for Contact {
    fn deref_any(&self, path: &str) -> eyre::Result<&dyn std::any::Any> {
        if path.is_empty() {
            return Ok(self);
        }
        let (head, tail) = path.split_once("/").unwrap_or((path, ""));
        match head {
            "name" => self.name.deref_any(tail),
            "url" => self.url.deref_any(tail),
            "email" => self.email.deref_any(tail),
            _ => eyre::bail!("not found: {head}"),
        }
    }
}

#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct License {
    pub name: String,
    pub url: Option<Url>,
}

impl JsonDeref for License {
    fn deref_any(&self, path: &str) -> eyre::Result<&dyn std::any::Any> {
        if path.is_empty() {
            return Ok(self);
        }
        let (head, tail) = path.split_once("/").unwrap_or((path, ""));
        match head {
            "name" => self.name.deref_any(tail),
            "url" => self.url.deref_any(tail),
            _ => eyre::bail!("not found: {head}"),
        }
    }
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

impl JsonDeref for PathItem {
    fn deref_any(&self, path: &str) -> eyre::Result<&dyn std::any::Any> {
        if path.is_empty() {
            return Ok(self);
        }
        let (head, tail) = path.split_once("/").unwrap_or((path, ""));
        match head {
            "$ref" => self._ref.deref_any(tail),
            "get" => self.get.deref_any(tail),
            "put" => self.put.deref_any(tail),
            "post" => self.post.deref_any(tail),
            "delete" => self.delete.deref_any(tail),
            "options" => self.options.deref_any(tail),
            "head" => self.head.deref_any(tail),
            "patch" => self.patch.deref_any(tail),
            "parameters" => self.parameters.deref_any(tail),
            _ => eyre::bail!("not found: {head}"),
        }
    }
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

impl JsonDeref for Operation {
    fn deref_any(&self, path: &str) -> eyre::Result<&dyn std::any::Any> {
        if path.is_empty() {
            return Ok(self);
        }
        let (head, tail) = path.split_once("/").unwrap_or((path, ""));
        match head {
            "tags" => self.tags.deref_any(tail),
            "summary" => self.summary.deref_any(tail),
            "description" => self.description.deref_any(tail),
            "external_docs" => self.external_docs.deref_any(tail),
            "operation_id" => self.operation_id.deref_any(tail),
            "consumes" => self.consumes.deref_any(tail),
            "produces" => self.produces.deref_any(tail),
            "parameters" => self.parameters.deref_any(tail),
            "responses" => self.responses.deref_any(tail),
            "schemes" => self.schemes.deref_any(tail),
            "deprecated" => self.deprecated.deref_any(tail),
            "security" => self.security.deref_any(tail),
            _ => eyre::bail!("not found: {head}"),
        }
    }
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

impl JsonDeref for ExternalDocs {
    fn deref_any(&self, path: &str) -> eyre::Result<&dyn std::any::Any> {
        if path.is_empty() {
            return Ok(self);
        }
        let (head, tail) = path.split_once("/").unwrap_or((path, ""));
        match head {
            "description" => self.description.deref_any(tail),
            "url" => self.url.deref_any(tail),
            _ => eyre::bail!("not found: {head}"),
        }
    }
}

#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Parameter {
    pub name: String,
    pub description: Option<String>,
    #[serde(flatten)]
    pub _in: ParameterIn,
}

impl JsonDeref for Parameter {
    fn deref_any(&self, path: &str) -> eyre::Result<&dyn std::any::Any> {
        if path.is_empty() {
            return Ok(self);
        }
        let (head, tail) = path.split_once("/").unwrap_or((path, ""));
        match head {
            "name" => self.name.deref_any(tail),
            "description" => self.description.deref_any(tail),
            "in" => {
                if tail.is_empty() {
                    Ok(match &self._in {
                        ParameterIn::Body { schema: _ } => &"body" as _,
                        ParameterIn::Path { param: _ } => &"path" as _,
                        ParameterIn::Query { param: _ } => &"query" as _,
                        ParameterIn::Header { param: _ } => &"header" as _,
                        ParameterIn::FormData { param: _ } => &"formData" as _,
                    })
                } else {
                    eyre::bail!("not found")
                }
            }
            _ => self._in.deref_any(path),
        }
    }
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

impl JsonDeref for ParameterIn {
    fn deref_any(&self, path: &str) -> eyre::Result<&dyn std::any::Any> {
        let (head, tail) = path.split_once("/").unwrap_or((path, ""));
        match self {
            ParameterIn::Body { schema } => match head {
                "schema" => schema.deref_any(tail),
                _ => eyre::bail!("not found: {head}"),
            },
            ParameterIn::Path { param }
            | ParameterIn::Query { param }
            | ParameterIn::Header { param }
            | ParameterIn::FormData { param } => param.deref_any(path),
        }
    }
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

impl JsonDeref for NonBodyParameter {
    fn deref_any(&self, path: &str) -> eyre::Result<&dyn std::any::Any> {
        if path.is_empty() {
            return Ok(self);
        }
        let (head, tail) = path.split_once("/").unwrap_or((path, ""));
        match head {
            "required" => self.required.deref_any(tail),
            "type" => self._type.deref_any(tail),
            "format" => self.format.deref_any(tail),
            "allow_empty_value" => self.allow_empty_value.deref_any(tail),
            "items" => self.items.deref_any(tail),
            "collection_format" => self.collection_format.deref_any(tail),
            "default" => self.default.deref_any(tail),
            "maximum" => self.maximum.deref_any(tail),
            "exclusive_maximum" => self.exclusive_maximum.deref_any(tail),
            "minimum" => self.minimum.deref_any(tail),
            "exclusive_minimum" => self.exclusive_minimum.deref_any(tail),
            "max_length" => self.max_length.deref_any(tail),
            "min_length" => self.min_length.deref_any(tail),
            "pattern" => self.pattern.deref_any(tail), // should be regex
            "max_items" => self.max_items.deref_any(tail),
            "min_items" => self.min_items.deref_any(tail),
            "unique_items" => self.unique_items.deref_any(tail),
            "enum" => self._enum.deref_any(tail),
            "multiple_of" => self.multiple_of.deref_any(tail),
            _ => eyre::bail!("not found: {head}"),
        }
    }
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

impl JsonDeref for ParameterType {
    fn deref_any<'a>(&'a self, path: &str) -> eyre::Result<&'a dyn std::any::Any> {
        if path.is_empty() {
            Ok(self)
        } else {
            Err(eyre::eyre!("not found"))
        }
    }
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

impl JsonDeref for CollectionFormat {
    fn deref_any<'a>(&'a self, path: &str) -> eyre::Result<&'a dyn std::any::Any> {
        if path.is_empty() {
            Ok(self)
        } else {
            Err(eyre::eyre!("not found"))
        }
    }
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

impl JsonDeref for Items {
    fn deref_any(&self, path: &str) -> eyre::Result<&dyn std::any::Any> {
        if path.is_empty() {
            return Ok(self);
        }
        let (head, tail) = path.split_once("/").unwrap_or((path, ""));
        match head {
            "type" => self._type.deref_any(tail),
            "format" => self.format.deref_any(tail),
            "items" => self.items.deref_any(tail),
            "collection_format" => self.collection_format.deref_any(tail),
            "default" => self.default.deref_any(tail),
            "maximum" => self.maximum.deref_any(tail),
            "exclusive_maximum" => self.exclusive_maximum.deref_any(tail),
            "minimum" => self.minimum.deref_any(tail),
            "exclusive_minimum" => self.exclusive_minimum.deref_any(tail),
            "max_length" => self.max_length.deref_any(tail),
            "min_length" => self.min_length.deref_any(tail),
            "pattern" => self.pattern.deref_any(tail), // should be regex
            "max_items" => self.max_items.deref_any(tail),
            "min_items" => self.min_items.deref_any(tail),
            "unique_items" => self.unique_items.deref_any(tail),
            "enum" => self._enum.deref_any(tail),
            "multiple_of" => self.multiple_of.deref_any(tail),
            _ => eyre::bail!("not found: {head}"),
        }
    }
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

impl JsonDeref for Responses {
    fn deref_any(&self, path: &str) -> eyre::Result<&dyn std::any::Any> {
        if path.is_empty() {
            return Ok(self);
        }
        let (head, tail) = path.split_once("/").unwrap_or((path, ""));
        match head {
            "default" => self.default.deref_any(tail),
            code => self
                .http_codes
                .get(code)
                .map(|r| r as _)
                .ok_or_else(|| eyre::eyre!("not found")),
        }
    }
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

impl JsonDeref for Response {
    fn deref_any(&self, path: &str) -> eyre::Result<&dyn std::any::Any> {
        if path.is_empty() {
            return Ok(self);
        }
        let (head, tail) = path.split_once("/").unwrap_or((path, ""));
        match head {
            "description" => self.description.deref_any(tail),
            "schema" => self.schema.deref_any(tail),
            "headers" => self.headers.deref_any(tail),
            "examples" => self.examples.deref_any(tail),
            _ => eyre::bail!("not found: {head}"),
        }
    }
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

impl JsonDeref for Header {
    fn deref_any(&self, path: &str) -> eyre::Result<&dyn std::any::Any> {
        if path.is_empty() {
            return Ok(self);
        }
        let (head, tail) = path.split_once("/").unwrap_or((path, ""));
        match head {
            "description" => self.description.deref_any(tail),
            "type" => self._type.deref_any(tail),
            "format" => self.format.deref_any(tail),
            "items" => self.items.deref_any(tail),
            "collection_format" => self.collection_format.deref_any(tail),
            "default" => self.default.deref_any(tail),
            "maximum" => self.maximum.deref_any(tail),
            "exclusive_maximum" => self.exclusive_maximum.deref_any(tail),
            "minimum" => self.minimum.deref_any(tail),
            "exclusive_minimum" => self.exclusive_minimum.deref_any(tail),
            "max_length" => self.max_length.deref_any(tail),
            "min_length" => self.min_length.deref_any(tail),
            "pattern" => self.pattern.deref_any(tail), // should be regex
            "max_items" => self.max_items.deref_any(tail),
            "min_items" => self.min_items.deref_any(tail),
            "unique_items" => self.unique_items.deref_any(tail),
            "enum" => self._enum.deref_any(tail),
            "multiple_of" => self.multiple_of.deref_any(tail),
            _ => eyre::bail!("not found: {head}"),
        }
    }
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

impl JsonDeref for Tag {
    fn deref_any(&self, path: &str) -> eyre::Result<&dyn std::any::Any> {
        if path.is_empty() {
            return Ok(self);
        }
        let (head, tail) = path.split_once("/").unwrap_or((path, ""));
        match head {
            "name" => self.name.deref_any(tail),
            "description" => self.description.deref_any(tail),
            "external_docs" => self.external_docs.deref_any(tail),
            _ => eyre::bail!("not found: {head}"),
        }
    }
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

    #[serde(flatten)]
    pub extensions: BTreeMap<String, serde_json::Value>,
}

impl JsonDeref for Schema {
    fn deref_any(&self, path: &str) -> eyre::Result<&dyn std::any::Any> {
        if path.is_empty() {
            return Ok(self);
        }
        let (head, tail) = path.split_once("/").unwrap_or((path, ""));
        match head {
            "format" => self.format.deref_any(tail),
            "title" => self.title.deref_any(tail),
            "description" => self.description.deref_any(tail),
            "default" => self.default.deref_any(tail),
            "multiple_of" => self.multiple_of.deref_any(tail),
            "maximum" => self.maximum.deref_any(tail),
            "exclusive_maximum" => self.exclusive_maximum.deref_any(tail),
            "minimum" => self.minimum.deref_any(tail),
            "exclusive_minimum" => self.exclusive_minimum.deref_any(tail),
            "max_length" => self.max_length.deref_any(tail),
            "min_length" => self.min_length.deref_any(tail),
            "pattern" => self.pattern.deref_any(tail), // should be regex
            "max_items" => self.max_items.deref_any(tail),
            "min_items" => self.min_items.deref_any(tail),
            "unique_items" => self.unique_items.deref_any(tail),
            "max_properties" => self.max_properties.deref_any(tail),
            "min_properties" => self.min_properties.deref_any(tail),
            "required" => self.required.deref_any(tail),
            "enum" => self._enum.deref_any(tail),
            "type" => self._type.deref_any(tail),
            "properties" => self.properties.deref_any(tail),
            "additional_properties" => self.additional_properties.deref_any(tail),
            "items" => self.items.deref_any(tail),
            "discriminator" => self.discriminator.deref_any(tail),
            "read_only" => self.read_only.deref_any(tail),
            "xml" => self.xml.deref_any(tail),
            "external_docs" => self.external_docs.deref_any(tail),
            "example" => self.example.deref_any(tail),
            _ => eyre::bail!("not found: {head}"),
        }
    }
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

impl JsonDeref for SchemaType {
    fn deref_any(&self, path: &str) -> eyre::Result<&dyn std::any::Any> {
        match self {
            SchemaType::One(i) => i.deref_any(path),
            SchemaType::List(list) => list.deref_any(path),
        }
    }
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

impl JsonDeref for Primitive {
    fn deref_any(&self, path: &str) -> eyre::Result<&dyn std::any::Any> {
        if path.is_empty() {
            Ok(self)
        } else {
            Err(eyre::eyre!("not found"))
        }
    }
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

impl JsonDeref for Xml {
    fn deref_any(&self, path: &str) -> eyre::Result<&dyn std::any::Any> {
        if path.is_empty() {
            return Ok(self);
        }
        let (head, tail) = path.split_once("/").unwrap_or((path, ""));
        match head {
            "name" => self.name.deref_any(tail),
            "namespace" => self.namespace.deref_any(tail),
            "prefix" => self.prefix.deref_any(tail),
            "attribute" => self.attribute.deref_any(tail),
            "wrapped" => self.wrapped.deref_any(tail),
            _ => eyre::bail!("not found: {head}"),
        }
    }
}

#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct SecurityScheme {
    #[serde(flatten)]
    pub _type: SecurityType,
    pub description: Option<String>,
}

impl JsonDeref for SecurityScheme {
    fn deref_any(&self, path: &str) -> eyre::Result<&dyn std::any::Any> {
        if path.is_empty() {
            return Ok(self);
        }
        let (head, tail) = path.split_once("/").unwrap_or((path, ""));
        match head {
            "type" => self._type.deref_any(tail),
            "description" => self.description.deref_any(tail),
            _ => eyre::bail!("not found: {head}"),
        }
    }
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

impl JsonDeref for SecurityType {
    fn deref_any(&self, path: &str) -> eyre::Result<&dyn std::any::Any> {
        if path.is_empty() {
            return Ok(self);
        }
        let (head, tail) = path.split_once("/").unwrap_or((path, ""));
        match self {
            SecurityType::Basic => eyre::bail!("not found: {head}"),
            SecurityType::ApiKey { name, _in } => match head {
                "name" => name.deref_any(tail),
                "in" => _in.deref_any(tail),
                _ => eyre::bail!("not found: {head}"),
            },
            SecurityType::OAuth2 { flow, scopes } => match head {
                "flow" => flow.deref_any(tail),
                "scopes" => scopes.deref_any(tail),
                _ => eyre::bail!("not found: {head}"),
            },
        }
    }
}

#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(rename_all(deserialize = "camelCase"))]
pub enum KeyIn {
    Query,
    Header,
}

impl JsonDeref for KeyIn {
    fn deref_any(&self, path: &str) -> eyre::Result<&dyn std::any::Any> {
        if path.is_empty() {
            Ok(self)
        } else {
            eyre::bail!("not found")
        }
    }
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

impl JsonDeref for OAuth2Flow {
    fn deref_any(&self, path: &str) -> eyre::Result<&dyn std::any::Any> {
        if path.is_empty() {
            return Ok(self);
        }
        let (head, tail) = path.split_once("/").unwrap_or((path, ""));
        match self {
            OAuth2Flow::Implicit { authorization_url } => match head {
                "authorizationUrl" => authorization_url.deref_any(tail),
                _ => eyre::bail!("not found: {head}"),
            },
            OAuth2Flow::Password { token_url } => match head {
                "tokenUrl" => token_url.deref_any(tail),
                _ => eyre::bail!("not found: {head}"),
            },
            OAuth2Flow::Application { token_url } => match head {
                "tokenUrl" => token_url.deref_any(tail),
                _ => eyre::bail!("not found: {head}"),
            },
            OAuth2Flow::AccessCode {
                authorization_url,
                token_url,
            } => match head {
                "authorizationUrl" => authorization_url.deref_any(tail),
                "tokenUrl" => token_url.deref_any(tail),
                _ => eyre::bail!("not found: {head}"),
            },
        }
    }
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

impl<T: JsonDeref> JsonDeref for MaybeRef<T> {
    fn deref_any(&self, path: &str) -> eyre::Result<&dyn std::any::Any> {
        if path.is_empty() {
            return Ok(self);
        }
        let (head, tail) = path.split_once("/").unwrap_or((path, ""));
        match self {
            MaybeRef::Ref { _ref } => match head {
                "$ref" => _ref.deref_any(tail),
                _ => eyre::bail!("not found: {head}"),
            },
            MaybeRef::Value { value } => value.deref_any(path),
        }
    }
}

impl<T: std::any::Any> MaybeRef<T> {
    pub fn deref<'a>(&'a self, spec: &'a OpenApiV2) -> eyre::Result<&'a T> {
        match self {
            MaybeRef::Ref { _ref } => spec.deref(_ref),
            MaybeRef::Value { value } => Ok(value),
        }
    }
}
