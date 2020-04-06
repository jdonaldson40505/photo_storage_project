use crate::lib::generate_key;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use serde_json::{json, Value};

mod lib;
#[derive(Deserialize, Debug)]
struct ParsedInformation {
    key: u128,
    username: String,
    password: String,
    name: String,
    photo: Vec<u8>,
}

fn example_data() -> String {
    let user_name = "Jdonaldson40505";
    let user_password = "joshua12309";
    let name = "Joshua Donaldson";
    let id = generate_key(user_name, user_password, name);
    let image = vec![0, 50];
    let Value_enum = serde_json::json!({
    "key": id,
    "user_name": user_name,
    "password": user_password,
    "photo": image
    });
    Value_enum.to_string()
}
async fn interpreter(data: String) -> ParsedInformation {
    serde_json::from_str(&data).unwrap()
}
async fn index(info: web::Data<ParsedInformation>) -> String {
    //    let data = example_data();
    let username = &info.username;
    let password = &info.password;
    let id = &info.key;
    //    let photo = &info.photo;
    format!(
        "key: {}, username: {}, password: {}",
        id, username, password,
    )
}

//    let parsed_information = serde_json::from_str(server_response).unwrap();
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let data = example_data();
    let info = interpreter(data);
    HttpServer::new(|| {
        App::new()
            .data(ParsedInformation {
                key: 0,
                username: "jdonaldson40505".to_string(),
                password: "joshua".to_string(),
                name: "Joshua".to_string(),
                photo: vec![0; 50],
            })
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
