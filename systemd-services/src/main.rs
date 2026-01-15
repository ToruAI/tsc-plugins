use serde_json::json;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    // Handle --metadata flag
    if args.len() > 1 && args[1] == "--metadata" {
        print_metadata();
        return;
    }

    // TODO: Start plugin server
    eprintln!("Plugin server not yet implemented");
    std::process::exit(1);
}

fn print_metadata() {
    let metadata = json!({
        "id": "systemd-services",
        "name": "Systemd Services",
        "version": env!("CARGO_PKG_VERSION"),
        "author": "ToruAI",
        "icon": "⚙️",
        "route": "/systemd-services"
    });

    println!("{}", serde_json::to_string_pretty(&metadata).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    #[test]
    fn test_metadata_format() {
        let metadata = json!({
            "id": "systemd-services",
            "name": "Systemd Services",
            "version": env!("CARGO_PKG_VERSION"),
            "author": "ToruAI",
            "icon": "⚙️",
            "route": "/systemd-services"
        });

        // Verify JSON is valid
        let json_str = serde_json::to_string(&metadata).unwrap();
        let parsed: Value = serde_json::from_str(&json_str).unwrap();

        // Verify required fields
        assert_eq!(parsed["id"], "systemd-services");
        assert_eq!(parsed["route"], "/systemd-services");
        assert_eq!(parsed["version"], env!("CARGO_PKG_VERSION"));
        assert_eq!(parsed["name"], "Systemd Services");
        assert_eq!(parsed["icon"], "⚙️");
    }
}
