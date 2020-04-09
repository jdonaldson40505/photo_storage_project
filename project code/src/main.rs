use project::*;
//use crate::lib::{
//    generate_key, get_photos, get_user, save_photo, save_user, Photo, PhotoId, UserData, UserId,
//};
use actix_web::{web, App, HttpServer};
use serde::Deserialize;
use serde::Serialize;
use std::borrow::Borrow;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

//parsed information holds information sent from and to webserver application
#[derive(Deserialize, Serialize, Debug, Clone)]
struct ParsedInformation {
    userid: UserId,
    username: String,
    password: String,
    name: String,
    photoid: PhotoId,
    photo: Photo,
    intent: String,
}
//puts test users in our storage system
async fn input_users() {
    let mut users = USERS.lock().await;
    let mut photos = PHOTOS.lock().await;
    let user = UserData {
        key: UserId(0),
        user_name: "josh".to_string(),
        password: "donaldson".to_string(),
        name: "Joshua Donaldson".to_string(),
    };
    users.insert(user.key, user);
    let image_holder = photos.entry(UserId(0));
    let photo_id = PhotoId(0);
    let image = Photo(vec![0; 50]);
    match image_holder {
        Entry::Occupied(mut o) => {
            o.get_mut().insert(photo_id, image);
        }
        Entry::Vacant(v) => {
            let mut new_photo_album = HashMap::new();
            new_photo_album.insert(photo_id, image);
            v.insert(new_photo_album);
        }
    };
}
//this is just for example and testing purposes until we hook up application to server to send json object
//creates json object
async fn example_data() -> String {
    //simulates a request from server in the form of json string
    let username = "josh".to_string();
    let password = "donaldson".to_string();
    let name = "Jake Donaldson".to_string();
    let userid = UserId(1);
    let photo = Photo(vec![1, 50]);
    let photoid = PhotoId(0);
    let intent = "get user".to_string();
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
            info.intent = "new user created".to_string();
            return info;
            //return all values as strings to serialize or deserialize
        }
        "get user" => {
            let saved_user = get_user(info.username.clone(), info.password.clone())
                .await
                .unwrap();
            info.name = saved_user.name;
            info.userid = saved_user.key;
            info.intent = "retrieved".to_string();
            serde_json::to_string(&info);
            println!("here we are {:?}", info,);
        }
        "save user" => {
            save_user(UserData {
                key: info.userid.clone(),
                user_name: info.username.to_string(),
                password: info.password.to_string(),
                name: info.name.to_string(),
            })
            .await;
            info.intent = "saved".to_string();
            return info;
        }
        "save photo" => {
            let saved_photo = save_photo(info.userid, Photo(info.photo.0.clone())).await;
            info.photoid = saved_photo;
            return info;
        }
        "get photo" => {
            let returned_photo = get_photos(info.userid.borrow(), info.photoid.borrow())
                .await
                .unwrap();
            info.photo = returned_photo;
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
    let photo = &info.photo;
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
    input_users().await;
    let data = example_data().await;
    let info = interpreter(data).await;
    HttpServer::new(move || {
        App::new()
            .data(ParsedInformation {
                userid: UserId(0),
                username: info.username.clone(),
                password: info.password.clone(),
                name: info.name.clone(),
                photoid: info.photoid.clone(),
                photo: info.photo.clone(),
                intent: info.intent.clone(),
            })
            .route("/", web::get().to(index))
    })
    //post instead of get
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
