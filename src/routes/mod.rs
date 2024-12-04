pub mod todo;

use ntex::{http, web};

use crate::errors;

pub async fn default() -> Result<String, errors::HttpError> {
    Err(errors::HttpError::new(
        "Not Found",
        http::StatusCode::NOT_FOUND,
    ))
}

#[web::get("/")]
pub async fn index() -> Result<impl web::Responder, web::Error> {
    Ok(web::HttpResponse::Ok().json(&errors::MessageRes {
        msg: String::from("Hello World"),
    }))
}
