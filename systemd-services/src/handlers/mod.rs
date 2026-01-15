// HTTP request handlers for systemd-services plugin

mod services;

#[cfg(test)]
mod tests;

pub use services::{
    handle_get_services,
    handle_get_available_services,
    handle_service_action,
    handle_get_logs,
};

use crate::error::Result;
use serde::Serialize;
use std::collections::HashMap;
use toru_plugin_api::HttpResponse;

/// Creates a JSON response with given status and data
pub fn json_response<T: Serialize>(status: u16, data: T) -> Result<HttpResponse> {
    let body = serde_json::to_string(&data)?;

    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "application/json".to_string());

    Ok(HttpResponse {
        status,
        headers,
        body: Some(body),
    })
}

/// Creates an error response
pub fn error_response(status: u16, error: &str) -> Result<HttpResponse> {
    let error_obj = serde_json::json!({
        "success": false,
        "error": error
    });

    json_response(status, error_obj)
}

/// Creates a success response
pub fn success_response(message: &str) -> Result<HttpResponse> {
    let success_obj = serde_json::json!({
        "success": true,
        "message": message
    });

    json_response(200, success_obj)
}

/// Parses query parameters from a path
pub fn parse_query_params(path: &str) -> HashMap<String, String> {
    let mut params = HashMap::new();

    if let Some(query_start) = path.find('?') {
        let query = &path[query_start + 1..];
        for pair in query.split('&') {
            if let Some(eq_pos) = pair.find('=') {
                let key = &pair[..eq_pos];
                let value = &pair[eq_pos + 1..];
                params.insert(key.to_string(), value.to_string());
            }
        }
    }

    params
}

/// Extracts path without query parameters
pub fn path_without_query(path: &str) -> &str {
    path.split('?').next().unwrap_or(path)
}
