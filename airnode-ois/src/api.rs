use crate::security;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "in")]
pub enum OperationParameter {
    Query { name: String },
    Header { name: String },
    Cookie { name: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Server {
    url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiSecurityScheme {
    #[serde(rename = "in")]
    target: security::scheme::Target,
    name: Option<String>,
    scheme: Option<security::scheme::Name>,
    #[serde(rename = "type")]
    t: security::scheme::Type,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Components {
    security_schemes: BTreeMap<String, ApiSecurityScheme>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Operation {
    parameters: Vec<OperationParameter>,
}

pub type Path = BTreeMap<String, Operation>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Specification {
    components: Components,
    paths: BTreeMap<String, Path>,
    security: security::Requirement,
    servers: Vec<Server>,
}
