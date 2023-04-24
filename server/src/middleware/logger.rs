use std::{
    fs::{self, File},
    io::Write,
    net::SocketAddr,
    path::Path,
};

use axum::{
    extract::ConnectInfo,
    http::{header::LOCATION, Request, StatusCode},
    middleware::Next,
    response::Response,
};
use chrono::{DateTime, Local};
use colored::Colorize;
use once_cell::unsync::Lazy;
use percent_encoding::percent_decode;

static mut LOG_FILE: Lazy<LogFile> = Lazy::new(|| LogFile::new("logs/%Y-%m-%d.log"));
struct LogFile {
    format: String,
    file: File,
    time: DateTime<Local>,
}

impl LogFile {
    fn new(format: &str) -> Self {
        let time = Local::now();
        let path = time.format(format).to_string();
        let path = Path::new(&path);

        if let Some(p) = path.parent() {
            fs::create_dir_all(p).expect("创建日志文件父级目录失败")
        }

        Self {
            format: format.to_owned(),
            file: File::options()
                .create(true)
                .append(true)
                .write(true)
                .open(path)
                .expect("日志文件创建失败"),
            time,
        }
    }
    pub fn update(&mut self) {
        let time = Local::now();
        let path = time.format(&self.format).to_string();
        let path = Path::new(&path);
        let file = File::options()
            .create(true)
            .append(true)
            .write(true)
            .open(path)
            .expect("日志文件创建失败");
        self.time = time;
        self.file = file;
    }

    pub fn write(time: &DateTime<Local>, s: &[u8]) {
        let mut log_file = unsafe {
            if time.date_naive() != LOG_FILE.time.date_naive() {
                LOG_FILE.update();
            }
            &LOG_FILE.file
        };

        if let Err(err) = log_file.write(s) {
            eprintln!("日志写入文件失败 -> {err}")
        }
    }
}

// 日志中间件
pub async fn logger<B>(
    ConnectInfo(ip): ConnectInfo<SocketAddr>,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    // 请求方式
    let method = req.method().to_string();
    // 请求路径
    let mut path = percent_decode(req.uri().path().as_bytes())
        .decode_utf8_lossy()
        .to_string();
    // 开始时间
    let time = Local::now();

    // 执行结果
    let res = next.run(req).await;
    // 状态码
    let status = res.status().as_u16();

    // 是否重定向
    if let Some(p) = res.headers().get(LOCATION) {
        let p = p.to_str().unwrap().as_bytes();
        path = format!("{path} -> {}", percent_decode(p).decode_utf8_lossy())
    }

    LogFile::write(
        &time,
        format!(
            "{} {} |{}| {:>6} | {:>15} | {:<5} {}\n",
            "[AXUM]",
            time.format("%Y-%m-%d %H:%M:%S"),
            status,
            format!("{}ms", (Local::now() - time).num_milliseconds()),
            ip.ip(),
            method,
            path
        )
        .as_bytes(),
    );

    let status = match status / 100 {
        2 => format!(" {status} ").on_green(),
        3 => format!(" {status} ").on_blue(),
        4 | 5 => format!(" {status} ").on_red(),
        _ => format!(" {status} ").on_yellow(),
    };

    println!(
        "{} {} |{}| {:>6} | {:>15} | {:<5} {}",
        "[AXUM]".bold().yellow(),
        time.format("%Y-%m-%d %H:%M:%S"),
        status,
        format!("{}ms", (Local::now() - time).num_milliseconds()),
        ip.ip(),
        method,
        path
    );
    Ok(res)
}
