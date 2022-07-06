use chrono::prelude::*;
use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{self, Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Account {
    pub _id: Option<ObjectId>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    pub password: String,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub role: Role,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NewAccount {
    pub email: String,
    pub password: String,
    pub role: String,
}

// #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
// pub struct AccountId(ObjectId);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    pub exp: chrono::DateTime<Utc>,
    pub account_id: ObjectId,
    pub nbf: chrono::DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Role {
    Admin,
    Manager,
    Employee,
}

impl FromStr for Role {
    type Err = ();

    fn from_str(input: &str) -> Result<Role, Self::Err> {
        match input {
            "Admin" => Ok(Role::Admin),
            "Manager" => Ok(Role::Manager),
            "Employee" => Ok(Role::Employee),
            _ => Err(()),
        }
    }
}
