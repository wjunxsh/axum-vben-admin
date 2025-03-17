use std::time::Duration;

use dotenv::dotenv;
pub use sea_orm::*;
pub async fn create_db_pool() -> DatabaseConnection {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut opt = ConnectOptions::new(database_url);
   opt.max_connections(100)
           .min_connections(5)
           .connect_timeout(Duration::from_secs(8))
           .acquire_timeout(Duration::from_secs(8))
           .idle_timeout(Duration::from_secs(8))
           .max_lifetime(Duration::from_secs(8))
   ;
   let db = Database::connect(opt).await.expect("Failed to connect to database");
   db
}