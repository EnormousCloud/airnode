use std::collections::BTreeMap;

pub mod scheme {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub enum Name {
        Bearer,
        Basic,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub enum Type {
        ApiKey,
        Http,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub enum Target {
        Query,
        Header,
        Cookie,
    }
}

pub type Requirement = BTreeMap<String, Vec<String>>;
