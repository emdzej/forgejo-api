use std::collections::BTreeMap;

use url::Url;

#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct OpenApiV2 {
    pub swagger: String,
    pub info: SpecInfo,
    pub host: Option<Url>,
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
    #[serde(rename = "in")]
    pub _in: String,
    pub description: Option<String>,
    pub required: Option<bool>,
    pub schema: Option<MaybeRef<Schema>>,
    #[serde(rename = "type")]
    pub _type: Option<ParameterType>,
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

#[derive(serde::Deserialize, Debug, PartialEq)]
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

#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Responses {
    pub default: Option<MaybeRef<Response>>,
    #[serde(flatten)]
    pub http_codes: BTreeMap<String, MaybeRef<Response>>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Response {
    pub description: String,
    pub schema: Option<Schema>,
    pub headers: Option<BTreeMap<String, Header>>,
    pub examples: Option<BTreeMap<String, serde_json::Value>>,
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

    pub discriminator: Option<String>,
    pub read_only: Option<bool>,
    pub xml: Option<Xml>,
    pub external_docs: Option<ExternalDocs>,
    pub example: Option<serde_json::Value>,
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
    Value {
        #[serde(flatten)]
        value: T,
    },
    Ref {
        #[serde(rename = "$ref")]
        _ref: String,
    },
}
