use serde_json::json;
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use systemd_services::systemctl::SystemCommandExecutor;
use toru_plugin_api::{
    HttpRequest, HttpResponse, KvOp, Message, MessagePayload, PluginContext, PluginError,
    PluginKvStore, PluginMetadata, PluginProtocol, ToruPlugin,
};

struct SystemdServicesPlugin {
    ctx: Option<PluginContext>,
    executor: Arc<SystemCommandExecutor>,
}

impl SystemdServicesPlugin {
    fn new() -> Self {
        Self {
            ctx: None,
            executor: Arc::new(SystemCommandExecutor::new()),
        }
    }

    fn metadata() -> PluginMetadata {
        PluginMetadata {
            id: "systemd-services".to_string(),
            name: "Systemd Services".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            author: Some("ToruAI".to_string()),
            icon: "⚙️".to_string(),
            route: "/systemd-services".to_string(),
        }
    }

    fn get_bundle_js() -> &'static str {
        include_str!("../frontend/dist/bundle.js")
    }

    fn kv_store(&self) -> Result<&dyn PluginKvStore, PluginError> {
        self.ctx
            .as_ref()
            .map(|ctx| ctx.kv.as_ref())
            .ok_or(PluginError::NotInitialized)
    }
}

#[async_trait::async_trait]
impl ToruPlugin for SystemdServicesPlugin {
    fn metadata() -> PluginMetadata {
        Self::metadata()
    }

    async fn init(&mut self, ctx: PluginContext) -> Result<(), PluginError> {
        eprintln!(
            "[SystemdServicesPlugin] Initializing with instance_id: {}",
            ctx.instance_id
        );
        self.ctx = Some(ctx);
        Ok(())
    }

    async fn handle_http(&self, req: HttpRequest) -> Result<HttpResponse, PluginError> {
        eprintln!(
            "[SystemdServicesPlugin] HTTP request: {} {}",
            req.method, req.path
        );

        // Extract path without query params
        let path_only = systemd_services::handlers::path_without_query(&req.path);
        let query_params = systemd_services::handlers::parse_query_params(&req.path);

        // Route the request
        match (req.method.as_str(), path_only) {
            // Serve frontend bundle
            ("GET", "/bundle.js") => Ok(HttpResponse {
                status: 200,
                headers: {
                    let mut h = HashMap::new();
                    h.insert(
                        "Content-Type".to_string(),
                        "application/javascript".to_string(),
                    );
                    h
                },
                body: Some(Self::get_bundle_js().to_string()),
            }),

            // Plugin info
            ("GET", "/" | "") => {
                let info = json!({
                    "plugin": "systemd-services",
                    "version": env!("CARGO_PKG_VERSION"),
                    "description": "Monitor and control systemd services"
                });

                Ok(HttpResponse {
                    status: 200,
                    headers: {
                        let mut h = HashMap::new();
                        h.insert("Content-Type".to_string(), "application/json".to_string());
                        h
                    },
                    body: Some(serde_json::to_string(&info)?),
                })
            }

            // GET /services - watched services with status
            ("GET", "/services") => {
                let kv = self.kv_store()?;
                systemd_services::handlers::handle_get_services(self.executor.clone(), kv)
                    .await
                    .map_err(|e| PluginError::Internal(e.to_string()))
            }

            // GET /services/available - all systemd services
            ("GET", "/services/available") => {
                systemd_services::handlers::handle_get_available_services(self.executor.clone())
                    .await
                    .map_err(|e| PluginError::Internal(e.to_string()))
            }

            // POST /services/:name/start|stop|restart
            ("POST", path) if path.starts_with("/services/") => {
                let parts: Vec<&str> = path.trim_start_matches("/services/").split('/').collect();

                if parts.len() != 2 {
                    return systemd_services::handlers::error_response(400, "Invalid path format")
                        .map_err(|e| PluginError::Internal(e.to_string()));
                }

                let service_name = parts[0];
                let action = parts[1];

                systemd_services::handlers::handle_service_action(
                    self.executor.clone(),
                    service_name,
                    action,
                )
                .await
                .map_err(|e| PluginError::Internal(e.to_string()))
            }

            // GET /services/:name/logs
            ("GET", path) if path.starts_with("/services/") && path.ends_with("/logs") => {
                let service_name = path
                    .trim_start_matches("/services/")
                    .trim_end_matches("/logs");

                systemd_services::handlers::handle_get_logs(
                    self.executor.clone(),
                    service_name,
                    &query_params,
                )
                .await
                .map_err(|e| PluginError::Internal(e.to_string()))
            }

            // 404 Not Found
            _ => systemd_services::handlers::error_response(404, "Not found")
                .map_err(|e| PluginError::Internal(e.to_string())),
        }
    }

    async fn handle_kv(&mut self, op: KvOp) -> Result<Option<String>, PluginError> {
        eprintln!("[SystemdServicesPlugin] KV operation: {:?}", op);

        let kv = self.kv_store()?;

        match op {
            KvOp::Get { key } => kv.get(&key).await,
            KvOp::Set { key, value } => {
                kv.set(&key, &value).await?;
                Ok(None)
            }
            KvOp::Delete { key } => {
                kv.delete(&key).await?;
                Ok(None)
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    // Handle --metadata flag
    if args.len() > 1 && args[1] == "--metadata" {
        let metadata = SystemdServicesPlugin::metadata();
        println!("{}", serde_json::to_string_pretty(&metadata).unwrap());
        return;
    }

    eprintln!("[SystemdServicesPlugin] Starting...");

    // Get socket path from environment or use default
    let plugin_id = SystemdServicesPlugin::metadata().id;
    let socket_path = env::var("TORU_PLUGIN_SOCKET")
        .unwrap_or_else(|_| format!("/tmp/toru-plugins/{}.sock", plugin_id));

    eprintln!("[SystemdServicesPlugin] Socket path: {}", socket_path);

    // Ensure socket directory exists
    if let Some(parent) = std::path::Path::new(&socket_path).parent() {
        std::fs::create_dir_all(parent).expect("Failed to create socket directory");
    }

    // Remove socket file if it exists
    if std::path::Path::new(&socket_path).exists() {
        std::fs::remove_file(&socket_path).expect("Failed to remove existing socket");
    }

    // Bind to Unix socket
    let listener = tokio::net::UnixListener::bind(&socket_path).expect("Failed to bind to socket");

    eprintln!("[SystemdServicesPlugin] Listening on socket...");

    let mut plugin = SystemdServicesPlugin::new();
    let mut protocol = PluginProtocol::new();

    // Accept connections
    loop {
        match listener.accept().await {
            Ok((mut stream, _)) => {
                eprintln!("[SystemdServicesPlugin] Connection accepted");

                // Handle messages
                loop {
                    match protocol.read_message(&mut stream).await {
                        Ok(message) => {
                            eprintln!(
                                "[SystemdServicesPlugin] Received message: {:?}",
                                message.message_type
                            );

                            // Handle message
                            match &message.payload {
                                MessagePayload::Lifecycle { action, payload } => {
                                    if action == "init" {
                                        if let Some(init_payload) = payload {
                                            let ctx = PluginContext {
                                                instance_id: init_payload.instance_id.clone(),
                                                config: toru_plugin_api::PluginConfig::default(),
                                                kv: Box::new(DummyKvStore),
                                            };
                                            if let Err(e) = plugin.init(ctx).await {
                                                eprintln!(
                                                    "[SystemdServicesPlugin] Init error: {}",
                                                    e
                                                );
                                            }
                                        }
                                    } else if action == "shutdown" {
                                        eprintln!("[SystemdServicesPlugin] Shutdown received");
                                        std::process::exit(0);
                                    }
                                }
                                MessagePayload::Http {
                                    request_id,
                                    payload,
                                } => match plugin.handle_http(payload.clone()).await {
                                    Ok(http_response) => {
                                        let response_msg = create_http_response(
                                            request_id.clone(),
                                            http_response,
                                        );
                                        if let Err(e) =
                                            protocol.write_message(&mut stream, &response_msg).await
                                        {
                                            eprintln!(
                                                "[SystemdServicesPlugin] Failed to write HTTP response: {}",
                                                e
                                            );
                                        }
                                    }
                                    Err(e) => {
                                        eprintln!(
                                            "[SystemdServicesPlugin] Error handling HTTP: {}",
                                            e
                                        );
                                    }
                                },
                                MessagePayload::Kv {
                                    request_id,
                                    payload,
                                } => {
                                    if let toru_plugin_api::KvMessagePayload::Request(kv_op) =
                                        payload
                                    {
                                        match plugin.handle_kv(kv_op.clone()).await {
                                            Ok(value) => {
                                                let response_msg = Message::new_kv_response(
                                                    request_id.clone(),
                                                    value,
                                                );
                                                if let Err(e) =
                                                    protocol.write_message(&mut stream, &response_msg).await
                                                {
                                                    eprintln!(
                                                        "[SystemdServicesPlugin] Failed to write KV response: {}",
                                                        e
                                                    );
                                                }
                                            }
                                            Err(e) => {
                                                eprintln!(
                                                    "[SystemdServicesPlugin] Error handling KV: {}",
                                                    e
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!(
                                "[SystemdServicesPlugin] Failed to read message: {}",
                                e
                            );
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!(
                    "[SystemdServicesPlugin] Failed to accept connection: {}",
                    e
                );
            }
        }
    }
}

fn create_http_response(request_id: String, http_response: HttpResponse) -> Message {
    let response_body = json!({
        "status": http_response.status,
        "headers": http_response.headers,
        "body": http_response.body,
    });

    Message::new_http(
        request_id,
        HttpRequest {
            method: "RESPONSE".to_string(),
            path: "".to_string(),
            headers: HashMap::new(),
            body: Some(serde_json::to_string(&response_body).unwrap()),
        },
    )
}

// Dummy KV store implementation (TSC will provide real one)
struct DummyKvStore;

#[async_trait::async_trait]
impl PluginKvStore for DummyKvStore {
    async fn get(&self, _key: &str) -> toru_plugin_api::PluginResult<Option<String>> {
        Ok(None)
    }

    async fn set(&self, _key: &str, _value: &str) -> toru_plugin_api::PluginResult<()> {
        Ok(())
    }

    async fn delete(&self, _key: &str) -> toru_plugin_api::PluginResult<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_format() {
        let metadata = SystemdServicesPlugin::metadata();

        assert_eq!(metadata.id, "systemd-services");
        assert_eq!(metadata.route, "/systemd-services");
        assert_eq!(metadata.version, env!("CARGO_PKG_VERSION"));
        assert_eq!(metadata.name, "Systemd Services");
        assert_eq!(metadata.icon, "⚙️");
    }

    #[test]
    fn test_metadata_json() {
        let metadata = SystemdServicesPlugin::metadata();
        let json_str = serde_json::to_string(&metadata).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(parsed["id"], "systemd-services");
        assert_eq!(parsed["route"], "/systemd-services");
    }
}
