use std::net::SocketAddr;

use axum::{
    middleware::from_fn,
    routing::{get, post},
    Json, Router,
};
use player_web::{exit_fmt, VideoItem, CONFIG};
use res::Res;
use serde::{Deserialize, Serialize};
use std::fs;
use tower_http::services::{ServeDir, ServeFile};

use crate::middleware::logger::logger;

mod middleware;
mod res;

#[tokio::main]
async fn main() {
    let video_server = ServeDir::new(&CONFIG.r#static);
    let client_server = ServeDir::new("dist");

    let app = Router::new()
        .route("/list", get(list))
        .route("/items", post(items))
        .nest_service("/", client_server.append_index_html_on_directories(true))
        .nest_service("/video", video_server)
        .fallback_service(ServeFile::new("dist/index.html"))
        .layer(from_fn(logger));

    let addr: SocketAddr = format!("{}:{}", CONFIG.host, CONFIG.port).parse().unwrap();

    println!("http://{addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap_or_else(|e| exit_fmt(e));
}

async fn list() -> Res<Vec<VideoInfo>> {
    let dirs = match fs::read_dir(&CONFIG.r#static) {
        Ok(res) => res.into_iter().filter_map(Result::ok),
        Err(err) => {
            eprintln!("读取文件目录{}失败: {err}", CONFIG.r#static);
            return Res::error("路径参数有误");
        }
    };

    let mut data = Vec::new();
    for item in dirs {
        let mut path = item.path();
        if path.is_file() {
            continue;
        }

        let name = path.file_stem().unwrap().to_string_lossy().to_string();

        path.push("icon.png");
        let icon = path.exists().then_some(format!("/video/{name}/icon.png"));

        data.push(VideoInfo::new(name, icon));
    }

    Res::ok(data)
}

#[derive(Deserialize)]
struct VideoName {
    name: String,
}
async fn items(Json(video_name): Json<VideoName>) -> Res<Vec<VideoItem>> {
    let path = CONFIG.sub_path(&video_name.name);

    match CONFIG.search_video(&path, "/video") {
        Ok(data) => Res::ok(data),
        Err(err) => {
            eprintln!("{err}");
            Res::error("参数有误或目录不存在")
        }
    }
}

/// 视频信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoInfo {
    name: String,
    icon: Option<String>,
}

impl VideoInfo {
    pub fn new(name: String, icon: Option<String>) -> Self {
        Self { name, icon }
    }
}
