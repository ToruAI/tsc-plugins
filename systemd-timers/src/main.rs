use serde_json::json;
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use systemd_timers::command::SystemCommandExecutor;
use toru_plugin_api::{
    HttpRequest, HttpResponse, KvOp, Message, MessagePayload, PluginContext,
    PluginError, PluginKvStore, PluginMetadata, PluginProtocol, ToruPlugin,
};

struct SystemdTimersPlugin {
    ctx: Option<PluginContext>,
    executor: Arc<SystemCommandExecutor>,
}

impl SystemdTimersPlugin {
    fn new() -> Self {
        Self {
            ctx: None,
            executor: Arc::new(SystemCommandExecutor),
        }
    }

    fn metadata() -> PluginMetadata {
        PluginMetadata {
            id: "systemd-timers".to_string(),
            name: "Scheduled Tasks".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            author: Some("ToruAI".to_string()),
            icon: "⏰".to_string(),
            route: "/systemd-timers".to_string(),
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
impl ToruPlugin for SystemdTimersPlugin {
    fn metadata() -> PluginMetadata {
        Self::metadata()
    }

    async fn init(&mut self, ctx: PluginContext) -> Result<(), PluginError> {
        eprintln!(
            "[SystemdTimersPlugin] Initializing with instance_id: {}",
            ctx.instance_id
        );
        self.ctx = Some(ctx);
        Ok(())
    }

    async fn handle_http(&self, req: HttpRequest) -> Result<HttpResponse, PluginError> {
        eprintln!(
            "[SystemdTimersPlugin] HTTP request: {} {}",
            req.method, req.path
        );

        let path_only = systemd_timers::handlers::path_without_query(&req.path);
        let query_params = systemd_timers::handlers::parse_query_params(&req.path);

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
                    "plugin": "systemd-timers",
                    "version": env!("CARGO_PKG_VERSION"),
                    "description": "Monitor and control systemd timers for scheduled tasks"
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

            // GET /timers - watched timers with status
            ("GET", "/timers") => {
                let kv = self.kv_store()?;
                systemd_timers::handlers::handle_get_timers(self.executor.clone(), kv)
                    .await
                    .map_err(|e| PluginError::Internal(e.to_string()))
            }

            // GET /timers/available - all systemd timers
            ("GET", "/timers/available") => {
                systemd_timers::handlers::handle_get_available_timers(self.executor.clone())
                    .await
                    .map_err(|e| PluginError::Internal(e.to_string()))
            }

            // GET /timers/settings - get settings
            ("GET", "/timers/settings") => {
                let kv = self.kv_store()?;
                systemd_timers::handlers::handle_get_settings(kv)
                    .await
                    .map_err(|e| PluginError::Internal(e.to_string()))
            }

            // POST /timers/settings - save settings
            ("POST", "/timers/settings") => {
                let kv = self.kv_store()?;
                let body = req.body.as_deref().unwrap_or("{}");
                systemd_timers::handlers::handle_save_settings(kv, body)
                    .await
                    .map_err(|e| PluginError::Internal(e.to_string()))
            }

            // POST /timers/:name/run - run timer
            ("POST", path) if path.starts_with("/timers/") && path.ends_with("/run") => {
                let timer_name = path
                    .trim_start_matches("/timers/")
                    .trim_end_matches("/run");
                systemd_timers::handlers::handle_run_timer(self.executor.clone(), timer_name)
                    .await
                    .map_err(|e| PluginError::Internal(e.to_string()))
            }

            // POST /timers/:name/test - test timer
            ("POST", path) if path.starts_with("/timers/") && path.ends_with("/test") => {
                let timer_name = path
                    .trim_start_matches("/timers/")
                    .trim_end_matches("/test");
                systemd_timers::handlers::handle_test_timer(self.executor.clone(), timer_name)
                    .await
                    .map_err(|e| PluginError::Internal(e.to_string()))
            }

            // POST /timers/:name/enable - enable timer
            ("POST", path) if path.starts_with("/timers/") && path.ends_with("/enable") => {
                let timer_name = path
                    .trim_start_matches("/timers/")
                    .trim_end_matches("/enable");
                systemd_timers::handlers::handle_enable_timer(self.executor.clone(), timer_name)
                    .await
                    .map_err(|e| PluginError::Internal(e.to_string()))
            }

            // POST /timers/:name/disable - disable timer
            ("POST", path) if path.starts_with("/timers/") && path.ends_with("/disable") => {
                let timer_name = path
                    .trim_start_matches("/timers/")
                    .trim_end_matches("/disable");
                systemd_timers::handlers::handle_disable_timer(self.executor.clone(), timer_name)
                    .await
                    .map_err(|e| PluginError::Internal(e.to_string()))
            }

            // GET /timers/:name/history/:id - execution details
            ("GET", path) if path.starts_with("/timers/") && path.contains("/history/") => {
                let parts: Vec<&str> = path.trim_start_matches("/timers/").split("/history/").collect();
                if parts.len() == 2 {
                    let timer_name = parts[0];
                    let invocation_id = parts[1];
                    systemd_timers::handlers::handle_get_history_details(
                        self.executor.clone(),
                        timer_name,
                        invocation_id,
                    )
                    .await
                    .map_err(|e| PluginError::Internal(e.to_string()))
                } else {
                    systemd_timers::handlers::error_response(400, "Invalid path format")
                        .map_err(|e| PluginError::Internal(e.to_string()))
                }
            }

            // GET /timers/:name/history - execution history
            ("GET", path) if path.starts_with("/timers/") && path.ends_with("/history") => {
                let timer_name = path
                    .trim_start_matches("/timers/")
                    .trim_end_matches("/history");
                systemd_timers::handlers::handle_get_history(
                    self.executor.clone(),
                    timer_name,
                    &query_params,
                )
                .await
                .map_err(|e| PluginError::Internal(e.to_string()))
            }

            // 404 Not Found
            _ => systemd_timers::handlers::error_response(404, "Not found")
                .map_err(|e| PluginError::Internal(e.to_string())),
        }
    }

    async fn handle_kv(&mut self, op: KvOp) -> Result<Option<String>, PluginError> {
        eprintln!("[SystemdTimersPlugin] KV operation: {:?}", op);

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
        let metadata = SystemdTimersPlugin::metadata();
        println!("{}", serde_json::to_string_pretty(&metadata).unwrap());
        return;
    }

    eprintln!("[SystemdTimersPlugin] Starting...");

    // Get socket path from environment or use default
    let plugin_id = SystemdTimersPlugin::metadata().id;
    let socket_path = env::var("TORU_PLUGIN_SOCKET")
        .unwrap_or_else(|_| format!("/tmp/toru-plugins/{}.sock", plugin_id));

    eprintln!("[SystemdTimersPlugin] Socket path: {}", socket_path);

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

    eprintln!("[SystemdTimersPlugin] Listening on socket...");

    let mut plugin = SystemdTimersPlugin::new();
    let mut protocol = PluginProtocol::new();

    // Accept connections
    loop {
        match listener.accept().await {
            Ok((mut stream, _)) => {
                eprintln!("[SystemdTimersPlugin] Connection accepted");

                // Handle messages
                loop {
                    match protocol.read_message(&mut stream).await {
                        Ok(message) => {
                            eprintln!(
                                "[SystemdTimersPlugin] Received message: {:?}",
                                message.message_type
                            );

                            // Handle message
                            match &message.payload {
                                MessagePayload::Lifecycle { action, payload } => {
                                    if action == "init" {
                                        if let Some(init_payload) = payload {
                                            let plugin_id = SystemdTimersPlugin::metadata().id;
                                            let ctx = PluginContext {
                                                instance_id: init_payload.instance_id.clone(),
                                                config: toru_plugin_api::PluginConfig::default(),
                                                kv: Box::new(FileKvStore::new(&plugin_id)),
                                            };
                                            if let Err(e) = plugin.init(ctx).await {
                                                eprintln!(
                                                    "[SystemdTimersPlugin] Init error: {}",
                                                    e
                                                );
                                            }
                                        }
                                    } else if action == "shutdown" {
                                        eprintln!("[SystemdTimersPlugin] Shutdown received");
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
                                                "[SystemdTimersPlugin] Failed to write HTTP response: {}",
                                                e
                                            );
                                        }
                                    }
                                    Err(e) => {
                                        eprintln!(
                                            "[SystemdTimersPlugin] Error handling HTTP: {}",
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
                                                        "[SystemdTimersPlugin] Failed to write KV response: {}",
                                                        e
                                                    );
                                                }
                                            }
                                            Err(e) => {
                                                eprintln!(
                                                    "[SystemdTimersPlugin] Error handling KV: {}",
                                                    e
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            // EOF is expected when TSC closes connection after sending a message
                            if !matches!(&e, PluginError::Io(io_err) if io_err.kind() == std::io::ErrorKind::UnexpectedEof) {
                                eprintln!(
                                    "[SystemdTimersPlugin] Failed to read message: {}",
                                    e
                                );
                            }
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!(
                    "[SystemdTimersPlugin] Failed to accept connection: {}",
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

// File-based KV store implementation for persistent settings
use std::sync::Mutex;

struct FileKvStore {
    file_path: std::path::PathBuf,
    cache: Mutex<HashMap<String, String>>,
}

impl FileKvStore {
    fn new(plugin_id: &str) -> Self {
        let data_dir = std::path::PathBuf::from("/var/lib/toru-plugins");
        std::fs::create_dir_all(&data_dir).ok();
        let file_path = data_dir.join(format!("{}.json", plugin_id));

        // Load existing data
        let cache = if file_path.exists() {
            std::fs::read_to_string(&file_path)
                .ok()
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_default()
        } else {
            HashMap::new()
        };

        Self {
            file_path,
            cache: Mutex::new(cache),
        }
    }

    fn save(&self) -> std::io::Result<()> {
        let cache = self.cache.lock().unwrap();
        let json = serde_json::to_string_pretty(&*cache)?;
        std::fs::write(&self.file_path, json)
    }
}

#[async_trait::async_trait]
impl PluginKvStore for FileKvStore {
    async fn get(&self, key: &str) -> toru_plugin_api::PluginResult<Option<String>> {
        let cache = self.cache.lock().unwrap();
        Ok(cache.get(key).cloned())
    }

    async fn set(&self, key: &str, value: &str) -> toru_plugin_api::PluginResult<()> {
        {
            let mut cache = self.cache.lock().unwrap();
            cache.insert(key.to_string(), value.to_string());
        }
        self.save().map_err(|e| toru_plugin_api::PluginError::Internal(e.to_string()))?;
        Ok(())
    }

    async fn delete(&self, key: &str) -> toru_plugin_api::PluginResult<()> {
        {
            let mut cache = self.cache.lock().unwrap();
            cache.remove(key);
        }
        self.save().map_err(|e| toru_plugin_api::PluginError::Internal(e.to_string()))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_format() {
        let metadata = SystemdTimersPlugin::metadata();

        assert_eq!(metadata.id, "systemd-timers");
        assert_eq!(metadata.route, "/systemd-timers");
        assert_eq!(metadata.version, env!("CARGO_PKG_VERSION"));
        assert_eq!(metadata.name, "Scheduled Tasks");
        assert_eq!(metadata.icon, "⏰");
    }

    #[test]
    fn test_metadata_json() {
        let metadata = SystemdTimersPlugin::metadata();
        let json_str = serde_json::to_string(&metadata).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(parsed["id"], "systemd-timers");
        assert_eq!(parsed["route"], "/systemd-timers");
    }
}
