use crate::config::Config;

pub async fn config(client: mongodb::Client) -> bool {
    println!("🔧 Checking database configuration");
    let config = Config::init();
    let databases = client.list_database_names(None, None).await.unwrap();
    let database_name = config.database_name;
    let mut found = false;
    println!("📙 Databases:");
    for name in databases {
        println!("- {}", name);
        if name == database_name {
            found = true
        }
    }
    let db = client.database(&database_name);
    // Create the database if it doesn't already exist:
    if found == false {
        println!("❌ Database not found: {}", database_name);
        println!("📝 Creating database: {}", database_name);
        db.create_collection("users", None)
            .await
            .expect("Failed to create collection: users");
        db.create_collection("logs", None)
            .await
            .expect("Failed to create collection: posts");
        db.create_collection("expired_tokens", None)
            .await
            .expect("Failed to create collection: services");
        db.create_collection("expired_tokens", None)
            .await
            .expect("Failed to create collection: services");
    }
    // Print the collections in our database:
    println!("📌 Collections:");
    for collection_name in db.list_collection_names(None).await.unwrap() {
        println!("- {}", collection_name);
    }
    return true;
}
