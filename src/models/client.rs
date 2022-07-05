use mongodb::bson::DateTime;
use serde::{self, Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClientSchema {
    pub _id: String, //ObjectId
    pub name: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    // TODO: created_by (acount who created this task)
}

// TODO: Rename to NewClient
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClientRequest {
    pub name: String,
}

// TODO: ClientResponse is almost the same as ClientSchema, take a better look and try to remove it.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClientResponse {
    pub _id: String, //ObjectId
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}
