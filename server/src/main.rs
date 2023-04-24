use std::net::SocketAddr;

use axum::{
    middleware::from_fn,
    routing::{get, post},
    Json, Router,
};
use player_web::{exit_fmt, CONFIG};
use tower_http::services::{ServeDir, ServeFile};
mod middleware;

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

async fn list() -> Json<Vec<VideoInfo>> {
    let dirs = match fs::read_dir(&CONFIG.r#static) {
        Ok(res) => res.into_iter().filter_map(Result::ok),
        Err(err) => {
            eprintln!("读取文件目录{}失败: {err}", CONFIG.r#static);
            return Json::from(vec![]);
        }
    };

    let mut res = Vec::new();
    for item in dirs {
        let mut path = item.path();
        if path.is_file() {
            continue;
        }

        let name = path.file_stem().unwrap().to_string_lossy().to_string();

        path.push("icon.png");
        let icon = path.exists().then_some(format!("/video/{name}/icon.png"));

        res.push(VideoInfo::new(name, icon));
    }

    Json::from(res)
}

// #[derive(Debug, Clone, Deserialize)]
// struct MyError {
//     code: u16,
//     msg: String,
// }

// impl MyError {
//     fn new(code: u16, msg: String) -> Json<Self> {
//         Json(Self { code, msg })
//     }
// }

#[derive(Deserialize)]
struct VideoName {
    name: String,
}
async fn items(Json(video_name): Json<VideoName>) -> Json<Vec<VideoItem>> {
    let path = PathBuf::from(format!("{}/{}", CONFIG.r#static, video_name.name));
    let res = search_video(
        &path,
        &path,
        &CONFIG.filter,
        &format!("/video/{}", video_name.name),
    );
    Json(res)
}

use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

use crate::middleware::logger::logger;

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

/// 视频 item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoItem {
    path: String,
    name: String,
}

impl VideoItem {
    pub fn new(path: String, name: String) -> Self {
        Self { path, name }
    }
}

pub fn search_video(p: &PathBuf, base: &PathBuf, filter: &[String], addr: &str) -> Vec<VideoItem> {
    let dirs = match fs::read_dir(&p) {
        Ok(res) => res.into_iter().filter_map(Result::ok),
        Err(err) => {
            eprintln!("读取文件目录[{}]失败: {err}", p.to_string_lossy());
            return vec![];
        }
    };

    let mut res = Vec::new();
    for item in dirs {
        let path = item.path();
        if path.is_dir() {
            // 递归深层扫描
            res.extend(search_video(&path, &base, filter, addr));
            continue;
        }

        // 匹配文件后缀
        match path.extension() {
            Some(ext) if filter.iter().any(|s| ext.eq_ignore_ascii_case(s)) => {
                // 去掉 base 路径 web 使用的是相对路径
                let path = if let Ok(path) = path.strip_prefix(&base) {
                    path.to_path_buf()
                } else {
                    path
                };

                let name = path.file_stem().unwrap().to_string_lossy().to_string();
                let path = format!("{addr}/{}", path.to_string_lossy().replace("\\", "/"));
                res.push(VideoItem::new(path, name));
            }
            _ => {}
        }
    }

    res
}
