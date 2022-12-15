use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};


#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(||{
        App::new()
            .service(hello)
            .route("/hey", web::get().to(manual_hello))
    })
        .bind(("app", 8080))?
        .run()
        .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!!!!")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}