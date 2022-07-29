use actix_identity::Identity;
use actix_web::{get, HttpResponse, put, post, Responder, web};
use actix_web::http::header;
use r2d2::Pool;
use r2d2_oracle::OracleConnectionManager;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::index::TempStruct;

#[get("/api")]
pub async fn api()  -> impl Responder {
    let mut vec:Vec<TempStruct> = Vec::new();
    vec.push(TempStruct{date:"2022-07-29".to_string(), temp:"36.5".to_string(), memo:Some("".to_string()) });

    return web::Json(vec);
}
