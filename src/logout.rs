use actix_identity::Identity;
use actix_web::{get, HttpResponse};
use actix_web::http::header;

#[get("/logout")]
pub async fn logout(id: Identity,
) -> HttpResponse {
    id.forget();                      // <- remove identity

    // loginページへ遷移
    HttpResponse::SeeOther()
        .header(header::LOCATION, "/login")
        .finish()
}
