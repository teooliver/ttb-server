use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{self, Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Account {
    pub _id: Option<ObjectId>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    pub password: String,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

// #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
// pub struct AccountId(ObjectId);
