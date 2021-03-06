use actix_identity::Identity;
use actix_web::{get, HttpResponse, post, web};
use actix_web::http::header;
use log::info;
use r2d2::Pool;
use r2d2_oracle::OracleConnectionManager;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tera::{Context, Tera};

use crate::index::MyError;

// このstructのフィールド名はlogin.htmlで指定されているformのタグ名と一致している必要がある；さもないとparse error
#[derive(Serialize, Deserialize)]
pub struct LoginForm {
    userid: String,
    password: String,
}

// login画面の表示
#[get("/login")]
pub async fn login(
    id: Identity,
    tmpl: web::Data<Tera>,
) -> Result<HttpResponse, MyError> {
    let mut ctx = Context::new();

    if let Some(idstr) = id.identity() {
        let v: Value = serde_json::from_str(&idstr)?;

        // /loginform側でセットしたエラーを示す特別なユーザID
        if v["error"] == "nouser" {
            ctx.insert("NoUser", &String::from("そのアカウントは存在しません"));
        } else if v["error"] == "notequal" {
            ctx.insert("NoUser", &String::from("パスワードが間違っています"));
        } else if v["error"] == "noreg" {
            ctx.insert("NoUser", &String::from("新規登録は停止中です"));
        }
        id.forget();
    }

    ctx.insert("Title", "体温記録システム");
    ctx.insert("CSS", "'/static/css/login.css'");

    let html = tmpl.render("login.html", &ctx).unwrap();
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(html))
}

// POST を処理  ただし SSLでは cookieが secureなときのみ転送できるので、debug環境ではauth cookieが保持できず、loginできない
#[post("/loginform")]
pub(crate) async fn post_login(db: web::Data<Pool<OracleConnectionManager>>,
                               id: Identity,
                               params: web::Form<LoginForm>,
) -> Result<HttpResponse, MyError> {
    let username = params.userid.to_owned();                // form から useridを読み込む
    let conn = db.get()?;
    let sql = "SELECT USERPASS,ID FROM BTUSERS WHERE USERID = :1";        // dbをアクセスし PASSWORDを得る

    match conn.query_row_as::<(String, i64)>(sql, &[&username]) {
        Ok((dbpass, dbuserid)) => {   // db上にユーザが存在した

            // bcrypt::verify を使い 一致するか検証する ; golang での bcryptと互換性があった
            let equal = bcrypt::verify(params.password.to_owned(), &dbpass).unwrap();

            // passwordが一致しなかった場合
            if equal == false {
                info!("password wrong");
                let json = json!({"error": "notequal"});  // wrong password　を login formに表示させる
                id.remember(json.to_string());
                return Ok(HttpResponse::SeeOther()
                    .header(header::LOCATION, "/login")
                    .finish());
            }

            // id, password が一致した!
            let json = json!({
                    "userid": dbuserid,
                    "username": &username,
                });
            id.remember(json.to_string());
            return Ok(HttpResponse::SeeOther()
                .header(header::LOCATION, "/")
                .finish());
        }
        Err(_) => {
            // dbにユーザーIDが存在しない
            info!("user not found");
            let json = json!({"error": "nouser"});  // wrong password　を login formに表示させる
            id.remember(json.to_string());
            return Ok(HttpResponse::SeeOther()
                .header(header::LOCATION, "/login")
                .finish());
        }
    }
}
