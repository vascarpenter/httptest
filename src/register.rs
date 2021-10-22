use std::sync::{Arc, Mutex};

use actix_identity::Identity;
use actix_web::{get, HttpResponse, post, web};
use actix_web::http::header;
use log::info;
use r2d2::Pool;
use r2d2_oracle::OracleConnectionManager;
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};

use crate::GlobalData;
use crate::index::MyError;

// このstructのフィールド名はregister.htmlで指定されているformのタグ名と一致している必要がある；さもないとparse error
#[derive(Serialize, Deserialize)]
pub struct RegisterForm {
	username: String,
	userpass: String,
}

// login画面の表示
#[get("/register")]
pub async fn register(
	id: Identity,
	tmpl: web::Data<Tera>,
	globals: web::Data<Arc<Mutex<GlobalData>>>,
) -> Result<HttpResponse, MyError> {
	let mut ctx = Context::new();

	if globals.lock().unwrap().register == false {
		// registration undefined; go to login
		return Ok(HttpResponse::SeeOther()
			.header(header::LOCATION, "/login")
			.finish());
	}

	if let Some(idstr) = id.identity() {

		// /registerform側でセットしたエラーを示す特別なユーザID
		if idstr == "#duplicate" {
			ctx.insert("NoUser", &String::from("アカウント名が重複しています"));
		} else if idstr == "#tooshort" {
			ctx.insert("NoUser", &String::from("アカウント名が短すぎます"));
		}
		id.forget();
	}

	ctx.insert("Title", "ユーザー登録");
	ctx.insert("CSS", "'/static/css/register.css'");

	let html = tmpl.render("register.html", &ctx).unwrap();
	Ok(HttpResponse::Ok()
		.content_type("text/html")
		.body(html))
}

// POST を処理
#[post("/registerform")]
pub(crate) async fn post_register(db: web::Data<Pool<OracleConnectionManager>>,
                                  id: Identity,
                                  params: web::Form<RegisterForm>,
) -> Result<HttpResponse, MyError> {
	let username = params.username.to_owned();                // form から useridを読み込む
	let userpass = params.userpass.to_owned();                // form から userpassを読み込む
	if username.len() < 4 {
		info!("user id too short");
		id.remember(String::from("#tooshort"));  // too short userid　を register formに表示させる

		return Ok(HttpResponse::SeeOther()
			.header(header::LOCATION, "/register")
			.finish());
	}
	let conn = db.get()?;
	let sql = "SELECT COUNT(*) FROM BTUSERS WHERE USERID = :1";        // dbをアクセスし 重複ないか探す

	if let Ok(count) = conn.query_row_as::<i64>(sql, &[&username]) {
		if count > 0 {
			// duplicate username in db
			info!("user duplicates");
			id.remember(String::from("#duplicate"));  // duplicate entry　を register formに表示させる
			return Ok(HttpResponse::SeeOther()
				.header(header::LOCATION, "/register")
				.finish());
		}
	}

	if let Ok(crypted) = bcrypt::hash(&userpass, 10) {
		let sql = r##"INSERT INTO BTUSERS (USERID, USERPASS) VALUES (:1,:2)"##;
		conn.execute(sql, &[&username, &crypted])?;
		conn.commit()?;
	}

	// 登録終了 login formへ
	return Ok(HttpResponse::SeeOther()
		.header(header::LOCATION, "/login")
		.finish());
}
