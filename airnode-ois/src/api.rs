use crate::security;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
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

/*

// ===========================================
// API Specification
// ===========================================
export interface Server {
  url: string;
}

export interface SecurityRequirement {
  [key: string]: string[];
}

export interface Operation {
  parameters: OperationParameter[];
}

export interface Path {
  [key: string]: Operation;
}

export type SecuritySchemeName = 'bearer' | 'basic';
export type SecuritySchemeType = 'apiKey' | 'http'; // | 'oauth2' | 'openIdConnect';
export type SecuritySchemeTarget = 'query' | 'header' | 'cookie';

export interface ApiSecurityScheme {
  in?: SecuritySchemeTarget;
  name?: string;
  scheme?: SecuritySchemeName;
  type: SecuritySchemeType;
}

export interface ApiComponents {
  securitySchemes: {
    [key: string]: ApiSecurityScheme;
  };
}

export interface ApiSpecification {
  components: ApiComponents;
  paths: { [key: string]: Path };
  security: SecurityRequirement;
  servers: Server[];
}

*/
