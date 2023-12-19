use crate::models::agent_basic::basic_agent::BasicAgent;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)] 
pub struct RouteObject {
    pub is_route_dynamic: String,
    pub methodt: String,
    pub request_body: serde_json::Value,
    pub response: serde_json::Value,
    pub route : String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)] 
pub struct ProjectScope {
    pub is_crud_required: bool,
    pub is_user_login_and_logout: bool,
    pub is_external_urls_required: bool,
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)] 
pub struct FacySheet {
    pub project_description: String,
    pub project_scope: Option<ProjectScope>,
    pub external_urls: Option<Vec<String>>,
    pub backend_code: Option<String>,
    pub api_endpoint_schema: Option<Vec<RouteObject>>
} 

