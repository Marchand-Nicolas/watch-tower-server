use mongodb::bson::doc;

use crate::config::Config;

use crate::utils::hash_password::hash_password;

pub async fn config(db: mongodb::Database) -> bool {
    println!("🔧 Creating root user");
    let config = Config::init();
    let root_user_password = config.root_user_password;
    // hash password
    let password_hash = hash_password(root_user_password.clone());
    let user =
        doc! { "username": "root", "password": password_hash, "permissions": ["administrator"] };
    db.collection("users").insert_one(user, None).await.unwrap();
    println!("👤 Created root user");
    return true;
}
