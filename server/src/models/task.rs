use mongodb::bson::oid::ObjectId;
use mongodb::bson::serde_helpers::bson_datetime_as_rfc3339_string;
use mongodb::bson::DateTime;
use serde::{self, Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TaskSchema {
    pub _id: ObjectId,
    pub name: String,
    pub project: Option<ObjectId>,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub initial_time: DateTime,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub end_time: DateTime,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub created_at: DateTime,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub updated_at: DateTime,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TaskRequest {
    pub name: String,
    pub initial_time: String,
    pub end_time: String,
    pub project: Option<ObjectId>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TaskResponse {
    pub _id: String,
    pub name: String,
    pub initial_time: String,
    pub end_time: String,
    pub project: Option<String>, //hex
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TasksGroupedByDate {
    pub _id: String,
    pub tasks: Vec<TaskAfterGrouped>,
    pub total_time: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TaskAfterGrouped {
    pub _id: String,
    pub name: String,
    pub initial_time: String,
    pub end_time: String,
    pub project: Option<String>,
    pub project_color: Option<String>,
    pub client: Option<String>,
}
