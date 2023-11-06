use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudFormationMacroRequest {
    pub account_id: String,
    pub fragment: HashMap<String, serde_json::Value>,
    pub transform_id: String,
    pub request_id: String,
    pub region: String,
    pub params: HashMap<String, serde_json::Value>,
    pub template_parameter_values: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub request_id: String,
    pub status: String,
    pub fragment: serde_json::Value,
}
