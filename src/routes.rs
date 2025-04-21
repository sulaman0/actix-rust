use actix_web::{get, HttpResponse, Responder};
use crate::service::{{test1}};


// main function!
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body(test1())
}

#[get("/profile")]
async f
// async fn profile() -> impl Responder {
//     HttpResponse::Ok().body(test1())
// }
