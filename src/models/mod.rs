use chrono::{DateTime, Local};
use chrono_tz::Asia::Jakarta;
use diesel::prelude::*;
use ntex::http::StatusCode;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

use crate::errors;

#[derive(Queryable, Selectable, Identifiable, AsChangeset, Serialize, Insertable)]
#[diesel(table_name = crate::schema::todos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub start_time: SystemTime,
    pub created_at: SystemTime,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TodoDto {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub start_time: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TodoReq {
    pub title: Option<String>,
    pub body: Option<String>,
    pub start_time: Option<String>,
}

// impl diesel::associations::HasTable for Todo {
//     type Table;

//     fn table() -> Self::Table {
//         todo!()
//     }
// }

impl std::fmt::Display for TodoReq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let title = self.title.clone().unwrap_or_else(|| "`None`".to_string());
        let body = self.body.clone().unwrap_or_else(|| "`None`".to_string());
        let start_time = self
            .start_time
            .clone()
            .unwrap_or_else(|| "`None`".to_string());
        write!(
            f,
            "TodoReq[title={}, body={}, start_time={}]",
            title, body, start_time
        )
    }
}

impl TodoReq {
    pub fn put_model(&self, model: &mut Todo) -> Option<errors::HttpError> {
        match &self.title {
            Some(s) => model.title = s.clone(),
            None => {}
        }
        match &self.body {
            Some(s) => model.body = s.clone(),
            None => {}
        }
        let opt_time = self.start_time.clone();
        if opt_time.is_some() {
            let time = opt_time.unwrap();
            let res_start_time = DateTime::parse_from_str(&time, "%Y-%m-%d %H:%M:%S %z");
            if res_start_time.is_err() {
                let err = res_start_time.err().unwrap();
                return Some(errors::HttpError::new(
                    err.to_string().as_str(),
                    StatusCode::BAD_REQUEST,
                ));
            }
            model.start_time = res_start_time.ok().unwrap().into();
        }
        None
    }

    pub fn to_model(&self) -> Result<Todo, errors::HttpError> {
        let opt_title = self.title.clone();
        if opt_title.is_none() {
            return Err(errors::HttpError::new(
                "title is required",
                StatusCode::BAD_REQUEST,
            ));
        }
        let title = opt_title.unwrap();
        if title.is_empty() {
            return Err(errors::HttpError::new(
                "title is required",
                StatusCode::BAD_REQUEST,
            ));
        }
        let opt_body = self.body.clone();
        if opt_body.is_none() {
            return Err(errors::HttpError::new(
                "body is required",
                StatusCode::BAD_REQUEST,
            ));
        }
        let body = opt_body.unwrap();
        if body.is_empty() {
            return Err(errors::HttpError::new(
                "body is required",
                StatusCode::BAD_REQUEST,
            ));
        }
        let opt_time = self.start_time.clone();
        if opt_time.is_none() {
            return Err(errors::HttpError::new(
                "start_time is required",
                StatusCode::BAD_REQUEST,
            ));
        }
        let time = opt_time.unwrap();
        if time.is_empty() {
            return Err(errors::HttpError::new(
                "start_time is required",
                StatusCode::BAD_REQUEST,
            ));
        }
        let opt_start_time = DateTime::parse_from_str(&time, "%Y-%m-%d %H:%M:%S %z");
        if opt_start_time.is_err() {
            let err = opt_start_time.err().unwrap();
            return Err(errors::HttpError::new(
                err.to_string().as_str(),
                StatusCode::BAD_REQUEST,
            ));
        }
        let start_time: SystemTime = opt_start_time.unwrap().with_timezone(&Jakarta).into();
        Ok(Todo {
            id: -1,
            title,
            body,
            start_time,
            created_at: SystemTime::now(),
        })
    }
}

impl std::fmt::Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let start_dt: DateTime<Local> = self.start_time.clone().into();
        let created_dt: DateTime<Local> = self.created_at.clone().into();
        write!(
            f,
            "Todo[id={}, title={}, body={}, start_time={}, created_at={}]",
            self.id, self.title, self.body, start_dt, created_dt
        )
    }
}

impl Todo {
    pub fn to_dto(&self) -> TodoDto {
        let start_dt: DateTime<Local> = self.start_time.clone().into();
        let created_dt: DateTime<Local> = self.created_at.clone().into();
        TodoDto {
            id: self.id,
            title: self.title.clone(),
            body: self.body.clone(),
            start_time: start_dt.to_string(),
            created_at: created_dt.to_string(),
        }
    }
}
