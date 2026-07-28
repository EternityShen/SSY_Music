use axum::{
    Json,
    extract::{Path, Query, State},
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use std::sync::Arc;
use tower::ServiceExt;
use tower_http::services::ServeFile;

use crate::api::data::{Data, Song};

/// 接收 URL Query 参数的结构体
#[derive(Debug, Deserialize)]
pub struct SearchParams {
    pub keyword: Option<String>,
}

/// 获取歌曲列表
///     -> Json<Vec<Song>>
///    GET /api/songs 或 /api/songs?keyword=xxx
pub async fn get_songs(
    Query(params): Query<SearchParams>,
    State(data): State<Arc<Data>>,
) -> Json<Vec<Song>> {
    // 拿到原始包含本地绝对路径的列表
    let mut list = data.search_songs(params.keyword);

    // 把本地文件路径变成客户端可以直接访问的网络相对路径
    for song in &mut list {
        song.path = format!("/api/songs/{}/stream", song.id);
        song.image = format!("/api/songs/{}/image", song.id);
    }

    Json(list)
}

/// 获取歌词
///     -> Result<impl IntoResponse, StatusCode>
///    GET /api/songs/{id}/lyrics
pub async fn get_lyrics(
    Path(id): Path<u64>,
    State(data): State<Arc<Data>>,
) -> Result<impl IntoResponse, StatusCode> {
    let content = data.get_lyrics(id)?;

    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        content,
    ))
}

/// 获取音频流
///     -> Result<Response, StatusCode>
///    GET /api/songs/{id}/stream
pub async fn stream_song(
    Path(id): Path<u64>,
    State(data): State<Arc<Data>>,
    req: axum::http::Request<axum::body::Body>,
) -> Result<Response, StatusCode> {
    let path = data.find_song_path(id)?;

    // 交给 ServeFile 来解决网络切片分发逻辑 (自己写纯找虐)
    let service = ServeFile::new(path);
    let res = service
        .oneshot(req)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(res.into_response())
}

/// 获取音乐数据
///     -> Result<Json<Song>, StatusCode>
///    GET /api/songs/{id}/data
pub async fn get_song(
    Path(id): Path<u64>,
    State(data): State<Arc<Data>>,
) -> Result<Json<Song>, StatusCode> {
    let option = data.get_song(id);

    match option {
        Some(song) => Ok(Json(song)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// 获取专辑图片
///     -> Result<Response, StatusCode>
///    GET /api/songs/{id}/image
pub async fn get_image(
    Path(id): Path<u64>,
    State(data): State<Arc<Data>>,
    req: axum::http::Request<axum::body::Body>,
) -> Result<Response, StatusCode> {
    let path = data.find_image_path(id)?;

    // 交给 ServeFile 来解决网络切片分发逻辑 (找虐是不可能找虐的)
    let service = ServeFile::new(path);
    let res = service
        .oneshot(req)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(res.into_response())
}
