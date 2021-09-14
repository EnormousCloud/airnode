pub mod api;
pub mod endpoint;
pub mod security;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OIS {
    ois_format: String,
    title: String,
    version: String,
    api_specifications: api::Specification,
    endpoints: Vec<endpoint::Endpoint>,
}
