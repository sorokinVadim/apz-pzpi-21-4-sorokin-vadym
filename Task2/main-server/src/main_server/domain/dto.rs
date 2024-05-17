use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", crate = "rocket::serde")]
pub struct DefaultResponse {
    pub id: Option<i32>,
    pub message: String,
}

impl DefaultResponse {
    pub fn new(message: String) -> Self {
        DefaultResponse { id: None, message }
    }

    pub fn new_with_id(id: i32, message: String) -> Self {
        DefaultResponse {
            id: Some(id),
            message,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserRewardPair {
    pub user_id: Option<i32>,
    pub reward_id: i32,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserExercisePair {
    pub user_id: Option<i32>,
    pub exercice_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Exercise {
    pub id: Option<i32>,
    pub name: String,
    pub measurement: String,
    pub exercice_type_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ExerciceType {
    pub id: Option<i32>,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Reward {
    pub id: Option<i32>,
    pub name: String,
    pub condition: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Admin {
    pub id: Option<i32>,
    pub user_id: i32,
    pub access_level: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Personal {
    pub id: Option<i32>,
    pub user_id: i32,
    pub specification_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Specification {
    pub id: Option<i32>,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub surname: Option<String>,
    pub email: String,
    pub password: Option<String>,
}

impl User {
    pub fn new_basic(email: String, name: String, surname: String) -> Self {
        User {
            id: None,
            name,
            surname: Some(surname),
            email,
            password: None,
        }
    }
}
