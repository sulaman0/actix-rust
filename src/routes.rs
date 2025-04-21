use actix_web::{get, HttpResponse, Responder};
use crate::service::{{test1}};


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body(test1())
}
// main functio