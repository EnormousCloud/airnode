use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "method")]
pub enum EndpointOperation {
    Get { path: String },
    Post { path: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "in")]
pub enum OperationParameter {
    Path { name: String },
    Query { name: String },
    Header { name: String },
    Cookie { name: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parameter {
    default: Option<String>,
    description: Option<String>,
    example: Option<String>,
    name: String,
    operation_arameter: OperationParameter,
    required: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedParameter {
    operation_parameter: OperationParameter,
    value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReservedParameterName {
    #[serde(rename = "_path")]
    Path,
    #[serde(rename = "_times")]
    Times,
    #[serde(rename = "_type")]
    Type,
    #[serde(rename = "_relay_metadata")]
    RelayMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReservedParameter {
    default: Option<String>,
    fixed: Option<String>,
    name: ReservedParameterName,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Endpoint {
    pub description: Option<String>,
    pub external_docs: Option<String>,
    pub fixed_operation_parameters: Vec<FixedParameter>,
    pub name: String,
    pub operation: EndpointOperation,
    pub parameters: Vec<Parameter>,
    pub reserved_parameters: Vec<ReservedParameter>,
    pub summary: Option<String>,
}
