use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{self, Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProjectSchema {
    pub _id: String, //ObjectId
    pub client: String,
    pub name: String,
    pub color: String,
    // pub estimate: String,
    // pub status: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProjectRequest {
    pub name: String,
    pub client: ObjectId,
    // pub estimate: String,
    // pub status: String,
    pub color: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProjectResponse {
    pub _id: String, //ObjectId
    pub client: String,
    pub name: String,
    pub color: String,
    // pub estimate: String,
    // pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProjectsGroupedByClient {
    pub _id: String,
    pub projects: Vec<ProjectAfterAggregation>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProjectAfterAggregation {
    pub _id: String,
    pub name: String,
    pub color: String,
    pub client_name: String,
    // pub estimate: String,
    // pub status: String,
}
