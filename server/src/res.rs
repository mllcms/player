use axum::{
    body::{boxed, Full},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Res<T> {
    code: u16,
    msg: String,
    data: Option<T>,
}

impl<T> IntoResponse for Res<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let body = Full::from(serde_json::to_vec(&self).unwrap());

        Response::builder()
            .status(StatusCode::OK)
            .header("Content-type", "application/json")
            .body(boxed(body))
            .unwrap()
    }
}

impl<T> Res<T>
where
    T: Serialize,
{
    #[allow(dead_code)]
    pub fn new<C, M>(code: C, msg: M, data: Option<T>) -> Self
    where
        C: Into<u16>,
        M: Into<String>,
    {
        Self {
            code: code.into(),
            msg: msg.into(),
            data,
        }
    }

    ///  成功的响应
    #[allow(dead_code)]
    pub fn ok(data: T) -> Self {
        Self {
            code: StatusCode::OK.as_u16(),
            msg: "ok".into(),
            data: Some(data),
        }
    }

    ///  成功的响应
    #[allow(dead_code)]
    pub fn success<M>(msg: M, data: T) -> Self
    where
        M: Into<String>,
    {
        Self {
            code: StatusCode::OK.as_u16(),
            msg: msg.into(),
            data: Some(data),
        }
    }

    ///  失败的响应
    #[allow(dead_code)]
    pub fn error<M>(msg: M) -> Self
    where
        M: Into<String>,
    {
        Self {
            code: StatusCode::BAD_REQUEST.as_u16(),
            msg: msg.into(),
            data: None,
        }
    }

    ///  身份认证失败
    #[allow(dead_code)]
    pub fn auth() -> Self {
        Self {
            code: StatusCode::UNAUTHORIZED.as_u16(),
            msg: "身份认证失败".to_owned(),
            data: None,
        }
    }

    ///  权限不足
    #[allow(dead_code)]
    pub fn privilege() -> Self {
        Self {
            code: StatusCode::UNAUTHORIZED.as_u16(),
            msg: "权限不足".to_owned(),
            data: None,
        }
    }
}
