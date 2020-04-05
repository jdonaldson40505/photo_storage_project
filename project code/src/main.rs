use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

mod lib;
struct parsed_information {
    key:

}
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

//    let parsed_information = serde_json::from_str(server_response).unwrap();
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind("127.0.0.1:8088")?
        .run()
        .await
}
