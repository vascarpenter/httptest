use actix_web::{get, HttpResponse, post, Responder, web};
use r2d2::Pool;
use r2d2_oracle::OracleConnectionManager;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use super::index::TempStruct;
use super::index::MyError;
use crate::GlobalData;

#[derive(Deserialize)]
pub struct APIInfo {
	pub apikey: String,
}

#[get("/api")]
pub async fn api(db: web::Data<Pool<OracleConnectionManager>>,
	params: web::Query<APIInfo>,
	globals: web::Data<Arc<Mutex<GlobalData>>>,
)  -> Result<HttpResponse, MyError>  {
    let conn = db.get()?;
	let mut vec:Vec<TempStruct> = Vec::new();
	// println!("param:{} global:{}", params.apikey, globals.lock().unwrap().apikey);

	if params.apikey == globals.lock().unwrap().apikey 
	{
		let sql = r##"SELECT TO_CHAR("DATE"),"TEMP","MEMO" FROM (SELECT * FROM BTDATA ORDER BY "DATE" DESC) t WHERE ROWNUM<14 ORDER BY t."DATE""##;
		let rows = conn.query_as::<(String, String, Option<String>)>(sql,&[])?;

		for row_result in rows {
			if let Ok((a, b, c)) = row_result {
				let entry = TempStruct {
					date: a,
					temp: b,
					memo: c,
				};
				vec.push(entry);
			}
		}
	}

    return Ok(HttpResponse::Ok()
		.content_type("application/json")
		.json(&vec));
}

#[derive(Deserialize)]
pub struct Info {
	temp: String,
	memo: String,
	apikey: String,
}

#[post("/apipost")]
pub async fn apipost(db: web::Data<Pool<OracleConnectionManager>>,
	params: web::Json<Info>,
	globals: web::Data<Arc<Mutex<GlobalData>>>,
)  -> Result<HttpResponse, MyError>  {
    let conn = db.get()?;

	if params.apikey == globals.lock().unwrap().apikey 
	{
		let sql = r##"INSERT INTO BTDATA ("ID","DATE","TEMP","MEMO") VALUES (1,localtimestamp,:1,:2)"##;
		conn.execute(sql, &[&params.temp, &params.memo])?;
		conn.commit()?;	
	}
	return Ok(HttpResponse::Ok()
		.content_type("text/html")
		.body("OK"));
}
