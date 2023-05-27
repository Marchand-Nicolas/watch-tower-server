use std::sync::Arc;

use mongodb::bson::doc;

use crate::AppState;

pub async fn has_permission(user_id: String, permission: String, app_state: Arc<AppState>) -> bool {
    let db = &app_state.db;
    let object_id = mongodb::bson::oid::ObjectId::parse_str(&user_id).unwrap();
    let user: Option<mongodb::bson::Document> = db
        .collection("users")
        .find_one(doc! { "_id": object_id }, None)
        .await
        .unwrap();
    let user = user.unwrap();
    let user_permissions = user.get_array("permissions").unwrap();
    let mut has_perm = false;
    for user_permission in user_permissions {
        if user_permission.as_str().unwrap() == permission {
            has_perm = true;
            break;
        }
    }
    return has_perm;
}
