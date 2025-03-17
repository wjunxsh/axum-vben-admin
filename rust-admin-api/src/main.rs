use std::sync::Arc;
use tracing::info;
use tracing_appender::rolling::{ RollingFileAppender, Rotation};
use std::fs::create_dir_all;
use dotenv::dotenv;
use cargo_demo::{app_state::AppState, router};
#[tokio::main]
async fn main() {
    dotenv().ok();
     // 创建日志目录
     create_dir_all("logs").unwrap();
     let file_appender = RollingFileAppender::new(Rotation::DAILY, "logs", "server.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    //let stdout = std::io::stdout.with_max_level(tracing::Level::INFO);
    tracing_subscriber::fmt()
        .with_writer(non_blocking)//只输出到文件
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::NEW | tracing_subscriber::fmt::format::FmtSpan::CLOSE) // 捕获 span 开启与关闭
        .init();
     info!("🚀 Server is starting...");
    let app_state = AppState::new().await;
    let admin_router = router::init_route(Arc::new(app_state.clone()));
    //let app: Router = Router::new().route("/", get(index)).with_state(Arc::new(app_state.clone()));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, admin_router).await.unwrap();
}