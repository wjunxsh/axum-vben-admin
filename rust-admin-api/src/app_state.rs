use r2d2;
use redis_r2d2::RedisConnectionManager;
use sea_orm::DatabaseConnection;
use s_orm::pool::pool::create_db_pool;
#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub redis_pool: r2d2::Pool<RedisConnectionManager>,
}


impl AppState {
    pub async fn new() -> Self {
        let manage = RedisConnectionManager::new("redis://192.168.16.136:6379/").unwrap();
        let redis_pool = r2d2::Pool::builder().max_size(15).build(manage).unwrap();
        //let app_state = create_db_pool().unwrap();】
        let db = create_db_pool().await;
        AppState {
            db: db.clone(),
            redis_pool: redis_pool.clone(),
        }
    }
}