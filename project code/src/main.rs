use crate::lib::{
    generate_key, get_photos, get_user, save_photo, save_user, Photo, PhotoId, UserData, UserId,
};
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use serde_json::{json, to_string, Value};
use std::borrow::Borrow;
mod lib;

//parsed information holds information sent from and to webserver application
#[derive(Deserialize, Serialize, Debug)]
struct ParsedInformation {
    key: u128,
    username: String,
    password: String,
    name: String,
    photo: Vec<u8>,
    intent: String,
}

//this is just for example and testing purposes until we hook up application to server to send json object
//creates json object
async fn example_data() -> String {
    let user_name = "Jdonaldson40505".to_string();
    let user_password = "joshua12309".to_string();
    let name = "Joshua Donaldson".to_string();
    let id = generate_key(user_name, user_password, name);
    let image = vec![0, 50];
    let value_enum = serde_json::json!({
    //    "key": id,
        "user_name": user_name,
        "password": user_password,
        "name": name,
        //"photo": image
        });
    value_enum.to_string()
}

//takes a json string and determines which process to run.
//async fn interpreter(data: String) -> ParsedInformation {
//    let mut info: ParsedInformation = serde_json::from_str(&data).unwrap();
//    match info.intent {
//        String::from("new user") => generate_key(info.username, info.password, info.name),
//        String::from("get user") => get_user(info.username, info.password),
//        String::from("save user") => save_user(UserData {
//            key: UserId(info.key),
//            user_name: info.username.to_string(),
//            password: info.password.to_string(),
//            name: info.name.to_string(),
//        }),
//        String::from("save photo") => save_photo(UserId(info.key), Photo(info.photo)),
//        String::from("get photo") => {
//            get_photos(UserId(info.key).borrow(), PhotoId(info.key).borrow())
//        }
//        _ => {}
//    }
//}

//index is the content to be passsed into data and the format to display is on local host 8088
async fn index(info: web::Data<ParsedInformation>) -> String {
    //    let data = example_data();
    let username = &info.username;
    let password = &info.password;
    let id = &info.key;
    let name = &info.name;
    let photo = &info.photo;
    format!(
        "key: {}, username: {}, password: {}, name: {}, photo: {:?}",
        id, username, password, name, photo
    )
}

//    let parsed_information = serde_json::from_str(server_response).unwrap();
//main will get our json example data then send it to our interpreter to decide what method to run it will then display the results on local host 8088
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let data = example_data();
    //    let info = interpreter(data);
    HttpServer::new(|| {
        App::new()
            .data(ParsedInformation {
                key: 0,
                username: "jdonaldson40505".to_string(),
                password: "joshua12309".to_string(),
                name: "Joshua Donaldson".to_string(),
                photo: vec![0; 50],
                intent: "new user".to_string(),
            })
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
