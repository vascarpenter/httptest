// actix-web, oracle access (r2d2 pool), template の使用のサンプル

extern crate openssl;
extern crate r2d2;
extern crate tera;

use std::process::exit;
use std::sync::{Arc, Mutex};

use actix_files as fs;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
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
    domain: String,

    /// port
    #[structopt(short, long, default_value = "443")]
    port: u16,

    /// privkey for SSL certificate
    #[structopt(short = "k", long = "privkey")]
    privkey: String,

    /// certkey for SSL certificate
    #[structopt(short = "c", long = "certkey")]
    certkey: String,

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let options = HttpTest::from_args();


    let templates = Tera::new("templates/**/*").unwrap();
    //let in_docker = Path::new("/.dockerenv").exists();

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_private_key_file(&options.privkey, SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file(&options.certkey).unwrap();

    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    // 起動方法
    // httptest -o $OCISTRING
    // httptest --dbenv OCISTRING → 環境変数 OCISTRING の中身のデータベースを使う (IntelliJ用..)
    // SSL化したため --certkey <certkey> --domain <domain> --privkey <privkey> の指定も必要

    let data = Arc::new(Mutex::new(GlobalData::default()));

    data.lock().unwrap().register = options.register;


    let mut ocistring: String = "".to_string();

    if let Some(t) = options.ocistring {
        ocistring = t;
    }

    if let Some(dbe) = options.dbenv {
        match std::env::var(&dbe)
        {
            Err(_) => eprintln!("Error get env var {}", dbe.to_string()),
            Ok(v) => ocistring = v,
        }
    }
    if ocistring == "" {
        eprintln!("--ocistring <ocistring> or --dbenv <ENV name which holds ocistring> needed");
        exit(1);
    }

    // oracle形式の connection string を分解して、username,password,connect stringの形式にする
    let atmarksep: Vec<&str> = ocistring.split("@").collect();
    let userpass = atmarksep[0];
    let slashsep: Vec<&str> = userpass.split("/").collect();

    let manager = OracleConnectionManager::new(
        slashsep[0],
        slashsep[1],
        atmarksep[1]);

    let pool = r2d2::Pool::builder()
        .max_size(15)
        .build(manager)
        .expect("Failed to create pool");

    let host = format!("0.0.0.0:{}", &options.port);
    let domain = options.domain.to_owned();


    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])    // <- create cookie identity policy
                    .domain(&domain)
                    .name("auth-cookie")
                    .secure(true)))
            .service(index::index)
            .service(index::post_index)
            .service(login::login)
            .service(login::post_login)
            .service(logout::logout)
            .service(register::register)
            .service(register::post_register)
            .service(fs::Files::new("/static", "./static"))
            .data(pool.clone())
            .data(templates.clone())
            .data(data.clone())
    })
        .bind_openssl(&host, builder)
        .expect(&format!("cannot run server at  {}", &host))
        .run()
        .await
}
