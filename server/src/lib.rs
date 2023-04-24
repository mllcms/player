use std::{
    fmt::Display,
    fs::{self, File},
    io::Write,
    path::Path,
    process::exit,
};

use once_cell::sync::Lazy;
use serde::Deserialize;

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
