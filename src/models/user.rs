#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserSchema {
    pub _id: ObjectId,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub account_created_at: DateTime,
    pub account_updated_at: DateTime,
}
