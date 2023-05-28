pub async fn config(client: mongodb::Client) -> bool {
    let databases = client.list_database_names(None, None).await.unwrap();
    println!("🔧 Configuring database");
    println!("{:?}", db);
    return false;
    return true;
}
