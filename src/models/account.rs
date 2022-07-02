#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Account {
    pub _id: Option<ObjectId>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub account_created_at: DateTime,
    pub account_updated_at: DateTime,
}

// #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
// pub struct AccountId(ObjectId);
