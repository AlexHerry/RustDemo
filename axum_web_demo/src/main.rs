use axum::{routing::get, Router};
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// 简单的处理函数，返回"Hello, World!"
async fn hello_world() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    // 设置日志记录
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "axum_web_demo=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 创建路由
    let app = Router::new()
        // 添加GET路由，路径为"/"，调用hello_world处理函数
        .route("/", get(hello_world));

    // 设置服务器地址
    let addr = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::debug!("listening on {:?}", addr);

    // 启动服务器
    axum::serve(addr, app).await.unwrap();
}
