use std::time::SystemTime;

use crate::{
    configs::db::DbPool,
    errors::{self, ArrayRes, BaseRes},
    models::{Todo, TodoDto, TodoReq},
    schema::todos,
    schema::todos::dsl::*,
};
use diesel::prelude::*;
use ntex::{http::StatusCode, web};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TodoQuery {
    pub desc: Option<String>,
}

#[web::delete("/todo/{id}")]
pub async fn del(
    path: web::types::Path<i32>,
    pool: web::types::State<DbPool>,
) -> Result<impl web::Responder, errors::HttpError> {
    let path_id = path.into_inner();
    let res_conn = pool.get();
    if res_conn.is_err() {
        let err = res_conn.err().unwrap();
        return Err(errors::HttpError::new(
            err.to_string().as_str(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }
    let opt_conn = res_conn.ok();
    if opt_conn.is_none() {
        return Err(errors::HttpError::new(
            "Connection not found",
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }
    let mut conn = opt_conn.unwrap();
    let res: Result<web::HttpResponse, diesel::result::Error> = conn
        .build_transaction()
        .serializable()
        .deferrable()
        .run(|conn| {
            let model = todos
                .filter(id.eq(path_id))
                .select(Todo::as_select())
                .first(conn)?;
            diesel::delete(todos.filter(id.eq(path_id))).execute(conn)?;
            let result = BaseRes {
                data: model,
                msg: "Success".to_string(),
            };
            Ok(web::HttpResponse::Ok().json(&result))
        });
    if res.is_err() {
        let err = res.err().unwrap();
        return Err(errors::HttpError::new(
            err.to_string().as_str(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }
    Ok(res.unwrap())
}

#[web::put("/todo/{id}")]
pub async fn put(
    path: web::types::Path<i32>,
    info: web::types::Json<TodoReq>,
    pool: web::types::State<DbPool>,
) -> Result<impl web::Responder, errors::HttpError> {
    let path_id = path.into_inner();
    let mut model = Todo {
        id: path_id,
        title: "".to_string(),
        body: "".to_string(),
        start_time: SystemTime::now(),
        created_at: SystemTime::now(),
    };
    let opt_err = info.put_model(&mut model);
    if opt_err.is_some() {
        return Err(opt_err.unwrap());
    }
    let res_conn = pool.get();
    if res_conn.is_err() {
        let err = res_conn.err().unwrap();
        return Err(errors::HttpError::new(
            err.to_string().as_str(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }
    let opt_conn = res_conn.ok();
    if opt_conn.is_none() {
        return Err(errors::HttpError::new(
            "Connection not found",
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }
    let mut conn = opt_conn.unwrap();
    let res: Result<web::HttpResponse, diesel::result::Error> = conn
        .build_transaction()
        .serializable()
        .deferrable()
        .run(|conn| {
            let mut model = todos
                .filter(id.eq(path_id))
                .select(Todo::as_select())
                .first(conn)?;
            info.put_model(&mut model);
            diesel::update(todos.filter(id.eq(path_id)))
                .set(Todo::from(model))
                .execute(conn)?;
            let model2 = todos
                .filter(id.eq(path_id))
                .select(Todo::as_select())
                .first(conn)?;
            let result = BaseRes {
                data: model2,
                msg: "Success".to_string(),
            };
            Ok(web::HttpResponse::Ok().json(&result))
        });
    if res.is_err() {
        let err = res.err().unwrap();
        return Err(errors::HttpError::new(
            err.to_string().as_str(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }
    Ok(res.unwrap())
}

#[web::post("/todo")]
pub async fn post(
    pool: web::types::State<DbPool>,
    info: web::types::Json<TodoReq>,
) -> Result<impl web::Responder, errors::HttpError> {
    log::info!("/todo POST Req {}", info);
    let res_model = info.to_model();
    if res_model.is_err() {
        let err = res_model.err().unwrap();
        return Err(err);
    }
    let res_conn = pool.get();
    if res_conn.is_err() {
        let err = res_conn.err().unwrap();
        return Err(errors::HttpError::new(
            err.to_string().as_str(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }
    let opt_conn = res_conn.ok();
    if opt_conn.is_none() {
        return Err(errors::HttpError::new(
            "Connection not found",
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }
    let mut conn = opt_conn.unwrap();
    let res: Result<web::HttpResponse, diesel::result::Error> = conn
        .build_transaction()
        .serializable()
        .deferrable()
        .run(|conn| {
            let opt_model = res_model.ok();
            let datas = todos
                .filter(id.ge(0))
                .select(Todo::as_select())
                .load(conn)?;
            let mut model = opt_model.unwrap();
            model.id = format!("{}", datas.len()).parse().unwrap();
            log::info!("Model {}", model);
            let data = diesel::insert_into(todos::table)
                .values(Todo::from(model))
                .get_result::<Todo>(conn)?;
            let result = BaseRes {
                msg: String::from("Success"),
                data,
            };
            Ok(web::HttpResponse::Created().json(&result))
        });
    if res.is_err() {
        let err = res.err().unwrap();
        return Err(errors::HttpError::new(
            err.to_string().as_str(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }
    Ok(res.unwrap())
}

#[web::get("/todo")]
pub async fn index(
    info: web::types::Query<TodoQuery>,
    pool: web::types::State<DbPool>,
) -> Result<impl web::Responder, errors::HttpError> {
    log::info!("/todo Query {}", info);
    let mut conn;
    if pool.get().is_err() {
        let err = pool.get().err().unwrap();
        return Err(errors::HttpError::new(
            err.to_string().as_str(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }
    let conn_opt = pool.get().ok();
    if conn_opt.is_none() {
        return Err(errors::HttpError::new(
            "Error Connection is none",
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }
    conn = conn_opt.unwrap();
    let mut datas = vec![];
    let query = todos
        .filter(id.ge(0))
        .select(Todo::as_select())
        .load(&mut conn);
    if query.is_err() {
        let err = query.err().unwrap();
        return Err(errors::HttpError::new(
            err.to_string().as_str(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }
    let datas_opt = query.ok();
    if datas_opt.is_none() {
        return Err(errors::HttpError::new(
            "Table todos is none",
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }
    for t in datas_opt.unwrap().iter().map(|t| t.to_dto()) {
        datas.push(t);
    }
    let result: ArrayRes<TodoDto> = errors::ArrayRes {
        msg: String::from("Success"),
        datas,
    };
    Ok(web::HttpResponse::Ok().json(&result))
}

impl std::fmt::Display for TodoQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TodoQuery[desc={}, ]",
            self.desc.clone().unwrap_or_else(|| String::from("`None`"))
        )
    }
}
