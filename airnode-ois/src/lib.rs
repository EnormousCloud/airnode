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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn it_parses() {
        let input = r#"
        {
            "oisFormat": "1.0.0",
            "title": "myOisTitle",
            "version": "1.2.3",
            "apiSpecifications": {
              "servers": [
                {
                  "url": "https://myapi.com/api/v1"
                }
              ],
              "components": {
                "securitySchemes": {
                  "mySecurityScheme1": {
                    "type": "apiKey",
                    "name": "X-MY-API-KEY",
                    "in": "query"
                  }
                }
              },
              "security": {
                "mySecurityScheme1": []
              },
              "paths": {
                "/myPath": {
                  "get": {
                    "parameters": [
                      {
                        "name": "myParameter",
                        "in": "query"
                      }
                    ]
                  }
                }
              }
            },
            "endpoints": [
              {
                "name": "convertToUsd",
                "operation": {
                  "path": "/myPath",
                  "method": "get"
                },
                "fixedOperationParameters": [
                  {
                    "operationParameter": {
                      "name": "to",
                      "in": "query"
                    },
                    "value": "USD"
                  }
                ],
                "reservedParameters": [
                  {
                    "name": "_type",
                    "fixed": "int256"
                  },
                  {
                    "name": "_path",
                    "default": "data.0.price"
                  },
                  {
                    "name": "_times"
                  }
                ],
                "parameters": [
                  {
                    "name": "f",
                    "default": "EUR",
                    "operationParameter": {
                      "name": "from",
                      "in": "query"
                    }
                  }
                ]
              }
            ]
          }"#;
        let out: OIS = serde_json::from_str(input).unwrap();
        println!("{:#?}", out);
    }
}
