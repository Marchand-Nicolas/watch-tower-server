use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2,
};
use mongodb::bson::{doc, Document};

use crate::config::Config;

pub async fn config(db: mongodb::Database) -> bool {
    let users_collections = db.collection("users");
    let user: Option<Document> = users_collections
        .find_one(None, None)
        .await
        .expect("Failed to get user");

    let config = Config::init();
    let root_user_password = config.root_user_password;
    // hash password
    let salt = SaltString::encode_b64(&config.password_salt.as_bytes()).unwrap();
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(root_user_password.as_bytes(), &salt)
        .unwrap()
        .to_string();
    if user.is_none() {
        println!("🔧 Creating root user");
        let user = doc! { "username": "root", "password": password_hash, "permissions": ["administrator"] };
        users_collections.insert_one(user, None).await.unwrap();
        println!("👤 Created root user");
    } else {
        let auto_update_root_user = config.auto_update_root_user;
        if auto_update_root_user {
            users_collections
                .update_one(
                    doc! { "username": "root" },
                    doc! { "$set": { "password": password_hash } },
                    None,
                )
                .await
                .unwrap();
            println!("🔧 Updated root user password");
        }
    }
    return true;
}
