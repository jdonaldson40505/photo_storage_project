use project::*;
//use crate::lib::{
//    generate_key, get_photos, get_user, save_photo, save_user, Photo, PhotoId, UserData, UserId,
//};
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use serde::Serialize;
use serde_json::{json, to_string, Value};
use std::borrow::Borrow;

//parsed information holds information sent from and to webserver application
#[derive(Deserialize, Serialize, Debug)]
struct ParsedInformation {
    userid: UserId,
    username: String,
    password: String,
    name: String,
    photoid: PhotoId,
    photo: String,
    intent: String,
}

//this is just for example and testing purposes until we hook up application to server to send json object
//creates json object
async fn example_data() -> String {
    let username = "Jdonaldson40505".to_string();
    let password = "joshua12309".to_string();
    let name = "Joshua Donaldson".to_string();
    let userid = generate_key(username.clone(), password.clone(), name.clone()).await;
    let photo = base64::encode(&vec![0, 50]);
    let photoid = PhotoId(0);
    let intent = "new user".to_string();
    let p_info = ParsedInformation {
        userid,
        username,
        password,
        name,
        photoid,
        photo,
        intent,
    };
    let stuff = serde_json::to_string(&p_info).unwrap();
    println!("this is what was passed in: {}", stuff);
    stuff
    //    let value_enum = serde_json::json!({
    //    //    "key": id,
    //        "user_name": user_name,
    //        "password": user_password,
    //        "name": name,
    //        "photo": image
    //        });
    //    value_enum.to_string()
}

//takes a json string and determines which process to run.
async fn interpreter(data: String) -> ParsedInformation {
    let mut info: ParsedInformation = serde_json::from_str(&data).unwrap();
    match info.intent.as_str() {
        "new user" => {
            let new_user = generate_key(
                info.username.clone(),
                info.password.clone(),
                info.name.clone(),
            )
            .await;
            info.userid = new_user;
            return info;
            //return all values as strings to serialize or deserialize
        }
        "get user" => {
            let stored_user = get_user(info.username.clone(), info.password.clone())
                .await
                .unwrap();
            info.name = stored_user.name;
            info.userid = stored_user.key;
            return info;
        }
        "save user" => {
            save_user(UserData {
                key: info.userid.clone(),
                user_name: info.username.to_string(),
                password: info.password.to_string(),
                name: info.name.to_string(),
            })
            .await;
            info.intent = "stored".to_string();
            return info;
        }
        "save photo" => {
            let saved_photo =
                save_photo(info.userid, Photo(base64::decode(&info.photo).unwrap())).await;
            info.photoid = saved_photo;
            return info;
        }
        "get photo" => {
            let returned_photo = get_photos(info.userid.borrow(), info.photoid.borrow())
                .await
                .unwrap();
            //            info.photo = base64::encode(&returned_photo);
            return info;
        }
        _ => {}
    }
    info
}

//index is the content to be passsed into data and the format to display is on local host 8088
async fn index(info: web::Data<ParsedInformation>) -> String {
    //    let data = example_data();
    let username = &info.username;
    let password = &info.password;
    let id = &info.userid;
    let name = &info.name;
    let photoid = &info.photoid;
    let photo = base64::decode(&info.photo);
    let intent = &info.intent;
    format!(
        "key: {:?}, username: {}, password: {}, name: {}, photoid: {:?}, photo: {:?}, intent: {}",
        id, username, password, name, photoid, photo, intent
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
                userid: UserId(0),
                username: "jdonaldson40505".to_string(),
                password: "joshua12309".to_string(),
                name: "Joshua Donaldson".to_string(),
                photoid: PhotoId(0),
                photo: base64::encode(&vec![0; 50]),
                intent: "new user".to_string(),
            })
            .route("/", web::get().to(index))
    })
    //post instead of get
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
