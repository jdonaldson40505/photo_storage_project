use futures;
use futures::lock::Mutex;
use serde::*;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

//UserId holds tuple of data type that will hold id number
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Copy, Clone, Deserialize, Serialize)]
pub struct UserId(pub u128);
//Photo id holds tuple of data type that will hold photo id number
#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq, Clone, Copy, Deserialize, Serialize)]
pub struct PhotoId(pub u128);
//Photo tuple that will hold a vec or our image
#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct Photo(pub Vec<u8>);

//static ar usize
lazy_static::lazy_static! {
    //USERS and PHOTOS holds user information and photos in place of our database
    static ref USERS: Mutex<HashMap<UserId, UserData>> = Mutex::new(HashMap::new());
    static ref PHOTOS: Mutex<HashMap<UserId, HashMap<PhotoId, Photo>>> = Mutex::new(HashMap::new());
}

//UserData is used to hold users information to be stored in users
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct UserData {
    pub key: UserId,
    pub user_name: String,
    pub password: String,
    pub name: String,
}

impl UserData {
    fn new(
        a_key: UserId,
        user_name: impl Into<String>,
        password: impl Into<String>,
        name: impl Into<String>,
    ) -> UserData {
        Self {
            key: a_key,
            user_name: user_name.into(),
            password: password.into(),
            name: name.into(),
        }
    }
}

//gets images from lazy static PHOTOS
pub async fn get_photos(id: &UserId, image: &PhotoId) -> Option<Photo> {
    let photos = PHOTOS.lock().await;
    for (key, value) in photos.iter() {
        if key == id {
            for (key2, value2) in value.iter() {
                if key2 == image {
                    return Some(value2.clone());
                }
            }
        }
    }
    None
}
//stores photo in lazy static PHOTOS
pub async fn save_photo(id: UserId, image: Photo) -> PhotoId {
    let mut photos = PHOTOS.lock().await;
    let image_holder = photos.entry(id);
    let photo_id = PhotoId(uuid::Uuid::new_v4().as_u128());
    match image_holder {
        Entry::Occupied(mut o) => {
            o.get_mut().insert(photo_id, image);
        }
        Entry::Vacant(v) => {
            let mut new_photo_album = HashMap::new();
            new_photo_album.insert(photo_id, image);
            v.insert(new_photo_album);
        }
    }
    photo_id
}

//gets user from lazy static USERS
pub async fn get_user(username: impl AsRef<str>, password: impl AsRef<str>) -> Option<UserData> {
    let users = USERS.lock().await;
    let username = username.as_ref();
    let password = password.as_ref();
    for (_key, value) in users.iter() {
        if value.user_name == username && value.password == password {
            return Some(value.clone());
        }
    }
    None
}

//Stores users in lazy static USERS
pub async fn save_user(user: UserData) {
    let mut users = USERS.lock().await;
    users.insert(user.key, user);
}

//creates unique key for a new user and then passes new information to save user to be stored
pub async fn generate_key(
    user_name: impl Into<String>,
    password: impl Into<String>,
    name: impl Into<String>,
) -> UserId {
    let id = uuid::Uuid::new_v4().as_u128();
    let new_user = UserData::new(UserId(id), user_name, password, name);
    save_user(new_user).await;
    UserId(id)
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static::lazy_static! {
        static ref TEST_MUTEX: Mutex<()> = Mutex::new(());
    }

    #[tokio::test]
    async fn test_user_id_creation() {
        let _guard = TEST_MUTEX.lock().await;
        //check to make sure it can handle multiple values correctly
        for i in 0..15 {
            generate_key(
                format!("jdonaldson{}", i),
                "joshua12309",
                "Joshua Donaldson",
            )
            .await;
        }
        let mut users = USERS.lock().await;

        assert_eq!(users.len(), 15);
        let mut user_id_holder = Vec::new();
        //    make sure that the keys aren't the same value
        for key in users.keys() {
            user_id_holder.push(key);
        }
        for x in user_id_holder.iter() {
            for y in user_id_holder.iter() {
                assert_ne!(x, y);
            }
        }
        users.clear();
    }

    #[tokio::test]
    async fn can_save_user() {
        let _guard = TEST_MUTEX.lock().await;
        let mut test_user_data = UserData {
            key: UserId(1000),
            user_name: "Jdonaldson40505".to_string(),
            password: "joshua12309".to_string(),
            name: "Joshua Donaldson".to_string(),
        };
        save_user(test_user_data).await; // normal name and password can be stored

        test_user_data = UserData {
            key: UserId(132),
            user_name: "catlover49".to_string(),
            password: "I Love Meow Mix".to_string(),
            name: "gertrude von lichtenschtien".to_string(),
        };
        save_user(test_user_data).await; // don't allow spaces in password

        test_user_data = UserData {
            key: UserId(11422),
            user_name: "supercalafragalisciousexpialadocious".to_string(),
            password: "password".to_string(),
            name: "".to_string(),
        };
        save_user(test_user_data).await; //see if it can handle absurdly long usernames

        test_user_data = UserData {
            key: UserId(1322),
            user_name: "I Have 20 dogs".to_string(),
            password: "humptydumpty".to_string(),
            name: "carl lindenburger".to_string(),
        };
        save_user(test_user_data).await; //ensure no usernames with spaces can be stored

        test_user_data = UserData {
            key: UserId(9001),
            user_name: "Jdonaldson40505".to_string(),
            password: "password".to_string(),
            name: "ken flittertail".to_string(),
        };
        save_user(test_user_data).await; //make sure no repeats usernames can be made
        let mut users = USERS.lock().await;
        assert_eq!(users.contains_key(&UserId(1000)), true);
        //        assert_eq!(users.contains_key(&UserId(1322)), false);
        assert_eq!(users.contains_key(&UserId(11422)), true);
        //        assert_eq!(users.contains_key(&UserId(9001)), false);
        //        assert_eq!(users.contains_key(&UserId(132)), false);
        users.clear();
    }
    #[tokio::test]
    async fn can_get_user_info() {
        let _guard = TEST_MUTEX.lock().await;
        let test_user_data = UserData {
            key: UserId(1000),
            user_name: "Jdonaldson40505".to_string(),
            password: "joshua12309".to_string(),
            name: "Joshua Donaldson".to_string(),
        };
        let info = UserData {
            key: UserId(1000),
            user_name: "Jdonaldson40505".to_string(),
            password: "joshua12309".to_string(),
            name: "Joshua Donaldson".to_string(),
        };
        save_user(test_user_data).await;
        //        let mut userinfo = get_user("Jdonaldson40505", "joshua12309").await;
        assert_eq!(get_user("Jdonaldson40505", "joshua12309").await, Some(info));
        let mut users = USERS.lock().await;
        users.clear();
    }

    #[tokio::test]
    async fn can_store_photo() {
        let _guard = TEST_MUTEX.lock().await;
        let test_user_data = UserData {
            key: UserId(1000),
            user_name: "Jdonaldson40505".to_string(),
            password: "joshua12309".to_string(),
            name: "Joshua Donaldson".to_string(),
        };
        save_user(test_user_data).await;
        let vec_to_test_photos: Vec<u8> = vec![0; 50];
        let some_value: u128 = 1000;
        save_photo(UserId(some_value), Photo(vec_to_test_photos)).await;
        let mut images = PHOTOS.lock().await;
        let image_finder = images.get(&UserId(some_value));
        let mut passed = false;
        for val in image_finder.iter() {
            match val {
                vec_to_test_photos => passed = true,
                _ => {}
            }
        }
        assert_eq!(passed, true);
        images.clear();
    }

    #[tokio::test]
    async fn can_get_photo() {
        let _guard = TEST_MUTEX.lock().await;
        let test_user_data = UserData {
            key: UserId(1000),
            user_name: "Jdonaldson40505".to_string(),
            password: "joshua12309".to_string(),
            name: "Joshua Donaldson".to_string(),
        };
        save_user(test_user_data).await;
        let vec_to_test_photos: Vec<u8> = vec![0; 50];
        let id: u128 = 1000;
        let saved_photo_id = save_photo(UserId(id), Photo(vec_to_test_photos)).await;
        let image = get_photos(&UserId(id), &saved_photo_id).await;
        let test_image = Photo(vec![0; 50]);
        let mut users = USERS.lock().await;
        let mut photos = PHOTOS.lock().await;
        assert_eq!(image, Some(test_image));
        photos.clear();
        users.clear();
    }
    //        #[tokio::test]
    //        async fn test_Can_Handle_Multiple_requests() {
    //            let i :u8 = 0;
    //            while i < 15 {
    //                   generate_key(format!("jdonaldson{}", i), "joshua12309", "Joshua Donaldson").await;
    //                   let mut info = get_user(format!("jdonaldson{}", i), "joshua12309").await;
    //                   info.name = format!("joshua Donaldson{}", i);
    //                   let mut id = info.key;
    //                   save_user(info).await;
    //                   get_photos(id).await;
    //                   let mut image = vec![i; 50];
    //                   save_photo(id, Photo(image)).await;
    //               };
    //        }
}
