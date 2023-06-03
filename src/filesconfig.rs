use std::fs::File;

pub async fn config() -> bool {
    println!("🔧 Checking files configuration");
    // config.json file
    let config_file = File::open("config.json");
    if config_file.is_err() {
        println!("📁 config.json file not found");
        println!("📁 creating config.json file");
        let config_file = File::create("config.json");
        if config_file.is_err() {
            println!("❌ failed to create config.json file");
            return false;
        }
        let config_file = config_file.unwrap();
        let config: std::collections::HashMap<String, String> = std::collections::HashMap::new();
        serde_json::to_writer_pretty(config_file, &config).unwrap();
    }
    return true;
}
