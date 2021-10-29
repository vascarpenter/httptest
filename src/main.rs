// actix-web, oracle access (r2d2 pool), template の使用のサンプル

extern crate openssl;
extern crate r2d2;
extern crate tera;

use std::sync::{Arc, Mutex};

use actix_files as fs;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
use r2d2_oracle::OracleConnectionManager;
use structopt::StructOpt;
use tera::Tera;

mod index;
mod logout;
mod login;
mod register;

#[derive(Debug, StructOpt)]
#[structopt(name = "httptest", about = "httptest example")]
pub struct HttpTest {
    /// domain name
    #[structopt(short, long)]
    domain: Option<String>,

    /// port
    #[structopt(short, long, default_value = "443")]
    port: u16,

    /// privkey for SSL certificate
    #[structopt(short = "k", long = "privkey")]
    privkey: Option<String>,

    /// certkey for SSL certificate
    #[structopt(short = "c", long = "certkey")]
    certkey: Option<String>,

    /// register user or not
    #[structopt(short, long)]
    register: bool,

    /// ssl or not
    #[structopt(short, long)]
    ssl: bool,

    /// oci connect string eg. admin/pass@//123.45.67.89/XEPDB1
    #[structopt(short, long, required_unless = "dbenv")]
    ocistring: Option<String>,

    /// environment variable name which contains oci connect string
    #[structopt(long)]
    dbenv: Option<String>,
}

#[derive(Debug, Default)]
pub struct GlobalData {
    register: bool,
}
// oracle形式の connection string を分解して、username,password,connect stringの形式にする

fn divide_ocistring(ocistring: String) -> Vec<String>
{
    let mut v = Vec::new();
    let atmarksep: Vec<&str> = ocistring.split("@").collect();
    let userpass = atmarksep[0];
    let slashsep: Vec<&str> = userpass.split("/").collect();
    v.push(slashsep[0].to_string());
    v.push(slashsep[1].to_string());
    v.push(atmarksep[1].to_string());
    return v;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let options = HttpTest::from_args();

    let templates = Tera::new("templates/**/*").unwrap();
    //let in_docker = Path::new("/.dockerenv").exists();
    let mut builder: SslAcceptorBuilder;
    builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    if options.ssl {
        builder.set_private_key_file(&options.privkey.unwrap(), SslFiletype::PEM).unwrap();
        builder.set_certificate_chain_file(&options.certkey.unwrap()).unwrap();
    }

    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    // 起動方法
    // httptest -o 接続文字列 あるいは
    // httptest --dbenv 環境変数名 → 環境変数に記載された接続文字列のデータベースを使う (IntelliJ用..)
    // SSLの場合は --ssl --certkey <certkey path> --domain <domain> --privkey <privkey path> の指定も必要

    // global data, fetched from each workers

    let data = Arc::new(Mutex::new(GlobalData::default()));

    data.lock().unwrap().register = options.register;

    let mut ocistring: String = options.ocistring.unwrap_or(String::from(""));

    if let Some(dbe) = options.dbenv {
        match std::env::var(&dbe) {
            Ok(v) =>
                ocistring = v,
            Err(_) =>
                panic!("Error get env var {}", dbe.to_string()),
        }
    }

    if ocistring == "" {
        panic!("--ocistring <oracle db connect string> or --dbenv <ENV name which holds oracle db connection string> needed");
    }

    // oracle形式の connection string を分解して、username,password,connect stringの形式にする
    let ocisep = divide_ocistring(ocistring);

    let manager = OracleConnectionManager::new(
        &ocisep[0],
        &ocisep[1],
        &ocisep[2]);


    let pool = r2d2::Pool::builder()
        .max_size(5)
        .build(manager)
        .expect("Failed to create pool");

    let host = format!("0.0.0.0:{}", &options.port);
    let domain = if options.ssl {
        options.domain.unwrap_or(String::from(""))
    } else {
        String::from("")
    };

    let sslmode = options.ssl;
    let server = HttpServer::new(move || {
        // ssl と non ssl 共通部分
        App::new()
            .wrap(Logger::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])    // <- create cookie identity policy
                    .domain(&domain)
                    .name("auth-cookie")
                    .secure(sslmode)))
            .service(login::login)
            .service(login::post_login)
            .service(logout::logout)
            .service(register::register)
            .service(register::post_register)
            .service(index::post_index)
            .service(index::index)
            .service(fs::Files::new("/static", "./static"))
            .data(pool.clone())
            .data(templates.clone())
            .data(data.clone())
    })
        .workers(2);
    if options.ssl {
        return server
            .bind_openssl(&host, builder)
            .expect(&format!("cannot run SSL server at  {}", &host))
            .run()
            .await;
    }
    server
        .bind(&host)
        .expect(&format!("cannot run server at  {} ", &host))
        .run()
        .await
}
