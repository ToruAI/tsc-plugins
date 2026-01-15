use serde_json::json;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "--metadata" {
        let metadata = json!({
            "id": "systemd-timers",
            "name": "Scheduled Tasks",
            "version": "0.1.0",
            "author": "ToruAI",
            "icon": "⏰",
            "route": "/systemd-timers"
        });
        println!("{}", serde_json::to_string_pretty(&metadata).unwrap());
        return;
    }

    println!("systemd-timers plugin - use --metadata to view plugin information");
}
