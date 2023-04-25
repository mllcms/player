use std::{
    fmt::Display,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    process::exit,
};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

pub static CONFIG: Lazy<Config> = Lazy::new(|| Config::new());

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: String,
    pub filter: Vec<String>,
    pub r#static: String,
}

impl Config {
    fn new() -> Self {
        let mut config = r#"# 地址
host = "127.0.0.1"
# 端口
port = "8080"
# 过滤视频文件扩展名
filter = ["mp4", "avi"]
# 静态资源地址
static = "static"
"#
        .to_string();

        let path = Path::new("config.toml");
        if path.exists() {
            config = fs::read_to_string(path).expect("读取 config 文件失败");
        } else {
            let mut file = File::create(path).expect("创建 config 文件失败");
            file.write(config.as_bytes()).unwrap();
        }

        toml::from_str(&config).expect("配置文件 config 内容有误")
    }
}

pub fn exit_fmt<S: Display>(s: S) -> ! {
    eprint!("[ERROR] {s}");
    exit(0)
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

impl Config {
    pub fn sub_path(&self, path: &str) -> PathBuf {
        PathBuf::from(format!("{}/{}", self.r#static, path))
    }
    pub fn search_video(&self, path: &PathBuf, net_addr: &str) -> Result<Vec<VideoItem>, String> {
        let dirs = match fs::read_dir(path) {
            Ok(res) => res.into_iter().filter_map(Result::ok),
            Err(err) => {
                return Err(format!(
                    "读取文件目录[{}]失败: {err}",
                    path.to_string_lossy()
                ))
            }
        };

        let mut data = Vec::new();
        for item in dirs {
            let path = item.path();
            if path.is_dir() {
                // 递归深层扫描
                data.extend(self.search_video(&path, net_addr)?);
                continue;
            }

            // 匹配文件后缀
            match path.extension() {
                Some(ext) if self.filter.iter().any(|s| ext.eq_ignore_ascii_case(s)) => {
                    // 去掉 base 路径 web 使用的是相对路径
                    let path = if let Ok(path) = path.strip_prefix(&self.r#static) {
                        path.to_path_buf()
                    } else {
                        path
                    };

                    let name = path.file_stem().unwrap().to_string_lossy().to_string();
                    let path = format!("{net_addr}/{}", path.to_string_lossy().replace("\\", "/"));
                    data.push(VideoItem::new(path, name));
                }
                _ => {}
            }
        }

        Ok(data)
    }
}
