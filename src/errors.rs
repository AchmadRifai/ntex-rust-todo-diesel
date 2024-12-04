use std::backtrace::Backtrace;

use ntex::http;
use ntex::web;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct HttpError {
    pub msg: String,
    pub path: String,
    #[serde(skip)]
    pub status: http::StatusCode,
}

impl HttpError {
    pub fn new(msg: &str, status: http::StatusCode) -> Self {
        Self {
            msg: String::from(msg),
            path: String::new(),
            status,
        }
    }
}

impl std::fmt::Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.status, self.msg)
    }
}

impl std::error::Error for HttpError {}

impl web::WebResponseError for HttpError {
    fn error_response(&self, req: &web::HttpRequest) -> web::HttpResponse {
        log::error!("[{}] msg is {}", req.path(), self.msg);
        if self.status.is_server_error() {
            log::error!("{:#?}", Backtrace::force_capture());
        }
        let mut error2 = HttpError::new(&self.msg, self.status);
        error2.path = String::from(req.path());
        web::HttpResponse::build(self.status).json(&error2)
    }
}

#[derive(Serialize)]
pub struct MessageRes {
    pub msg: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct BaseRes<T> {
    pub msg: String,
    pub data: T,
}

#[derive(Clone, Debug, Serialize)]
pub struct ArrayRes<T> {
    pub msg: String,
    pub datas: Vec<T>,
}
