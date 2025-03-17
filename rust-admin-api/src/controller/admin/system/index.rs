use axum::{
    http::StatusCode,
    extract::Json,
};
use res::Response;


pub struct Index;
impl Index {
    pub async fn index() ->  Result<Json<Response<String>>, (StatusCode, String)>  {
        Ok(Json(Response::new(0, "Hello world!".to_string())))
        // String::from("Hello, World!")
    }
}