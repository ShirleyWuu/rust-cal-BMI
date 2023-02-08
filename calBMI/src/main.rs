// web microservice for calculationg BMI

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello! Welcome to BMI calculator!")
}

#[get("/bmi/{w}/{h}")]
async fn bmi(info: web::Path<(f32, f32)>) -> impl Responder {
    let bmi = calBMI::calculate_bmi(info.0, info.1);
    HttpResponse::Ok().body(format!("Your BMI is {}", bmi))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(bmi))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
