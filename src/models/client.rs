use mongodb::bson::DateTime;
use serde::{self, Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClientSchema {
    pub _id: String, //ObjectId
    pub name: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClientRequest {
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClientResponse {
    pub _id: String, //ObjectId
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}
