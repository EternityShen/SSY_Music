use std::sync::{Arc, Mutex};

use axum::{Router, routing::get};
use server::api;

#[tokio::main]
async fn main() {
    // 创建logger_handle并清理历史日志(纯废话)
    let mut logger = logger::Logger::new("./server.log");
    logger.clear();
    let logger_handle = Arc::new(Mutex::new(logger));
    if let Ok(mut log) = logger_handle.lock() {
        log.clear();
    }

    // 创建数据库(纯废话)
    let data = Arc::new(api::data::Data::load_from_toml(
        "./Test/music_db.toml",
        "/home/eternity/Music/歌词/",
        logger_handle,
    ));

    // 创建服务，并添加路由(纯废话)
    let server = Router::new()
        // http .........  /api/songs // /api/songs?keyword=xxx
        .route("/api/songs", get(api::request::get_songs))
        // http .........  /api/songs/{id}/stream
        .route("/api/songs/{id}/stream", get(api::request::stream_song))
        // http .........  /api/songs/{id}/lyrics
        .route("/api/songs/{id}/lyrics", get(api::request::get_lyrics))
        // http .........  /api/songs/{id}/image
        .route("/api/songs/{id}/image", get(api::request::get_image))
        // http ......... /api/songs/{id}/data
        .route("/api/songs/{id}/data", get(api::request::get_song))
        .with_state(data);

    // 创建端口监听(纯废话)
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    // 打印废话
    println!("ShenEternity-Player 服务端启动: 127.0.0.1:3000");

    // 启动服务
    axum::serve(listener, server).await.unwrap();
}
