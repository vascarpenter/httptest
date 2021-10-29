use actix_identity::Identity;
use actix_web::{get, HttpResponse, post, ResponseError, web};
use actix_web::http::header;
use r2d2::Pool;
use r2d2_oracle::OracleConnectionManager;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tera::{Context, Tera};
use thiserror::Error;

// HTML構成用のstruct
#[derive(Serialize)]
pub struct TempStruct {
	date: String,
	temp: String,
	memo: Option<String>,
}

#[derive(Error, Debug)]
pub enum MyError {
	#[error("failed to get connection")]
	ConnectionPoolError(#[from] r2d2::Error),

	#[error("failed SQL")]
	SQLError(#[from] r2d2_oracle::oracle::Error),

	#[error("template error")]
	RendererError(#[from] tera::Error),

	#[error("json convert error")]
	JSONError(#[from] serde_json::Error),
}

impl ResponseError for MyError {}

#[get("/")]
pub async fn index(db: web::Data<Pool<OracleConnectionManager>>,
                   tmpl: web::Data<Tera>,
                   id: Identity,
) -> Result<HttpResponse, MyError> {
	let mut ctx = Context::new();
	let conn = db.get()?;
	let mut userid = 0;
	if let Some(idstr) = id.identity() {
		let v: Value = serde_json::from_str(idstr.as_str())?;
		let str = format!("ようこそ、 {} さん", v["username"]);
		ctx.insert("Title", &str);
		userid = v["userid"].as_i64().map_or(0, |v| v);
	}
	if userid == 0 {
		id.forget();
		// not login; redirect to login page
		return Ok(HttpResponse::SeeOther()
			.header(header::LOCATION, "/login")
			.finish())
	}
	ctx.insert("CSS", "'/static/css/index.css'");

	let sql = r##"SELECT TO_CHAR("DATE"),"TEMP","MEMO" FROM (SELECT * FROM BTDATA ORDER BY "DATE" DESC) t WHERE ROWNUM<14 AND "ID"=:1 ORDER BY t."DATE""##;
	let rows = conn.query_as::<(String, String, Option<String>)>(sql, &[&userid])?;

	let mut entries = Vec::new();
	for row_result in rows {
		if let Ok((a, b, c)) = row_result {
			let entry = TempStruct {
				date: a,
				temp: b,
				memo: c,
			};
			entries.push(entry);
		}
	}

	ctx.insert("entries", &entries);

	let html = tmpl.render("index.html", &ctx)?;

	Ok(HttpResponse::Ok()
		.content_type("text/html")
		.body(html))
}

// form解析用struct このstructのフィールド名はformのタグ名と一致している必要がある；さもないとparse error
#[derive(Serialize, Deserialize)]
pub struct IndexInfo {
	temp: String,
	memo: String,
}

// POST /form を処理
#[post("/form")]
pub(crate) async fn post_index(db: web::Data<Pool<OracleConnectionManager>>,
                               id: Identity,
                               params: web::Form<IndexInfo>,
) -> Result<HttpResponse, MyError> {
	let conn = db.get()?;
	let mut userid = 0;
	if let Some(idstr) = id.identity() {
		let v: Value = serde_json::from_str(idstr.as_str())?;
		userid = v["userid"].as_i64().map_or(0, |v| v);

		//println!("{:?} userid={}", v, userid);
	}

	if userid == 0 {
		id.forget();
		// not login; redirect to login page
		return Ok(HttpResponse::SeeOther()
			.header(header::LOCATION, "/login")
			.finish())
	}

	let temp = params.temp.to_owned();                // form から tempを読み込む
	let memo = params.memo.to_owned();
	let sql = r##"INSERT INTO BTDATA ("ID","DATE","TEMP","MEMO") VALUES (:1,localtimestamp,:2,:3)"##;
	conn.execute(sql, &[&userid, &temp, &memo])?;
	conn.commit()?;

	return Ok(HttpResponse::SeeOther()
		.header(header::LOCATION, "/")
		.finish())
}
