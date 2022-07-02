use crate::models::account::Account;
use crate::{error::Error::*, Result};
use chrono::prelude::*;
use futures::StreamExt;
use mongodb::bson::{self, Bson};
use mongodb::bson::{doc, document::Document, oid::ObjectId};
use mongodb::Collection;

use super::DB;

impl DB {
    pub fn get_accounts_collection(&self) -> Collection<Document> {
        self.client.database(&self.db_name).collection("accounts")
    }

    // fn doc_to_account(&self, doc: &Document) -> Result<Account> {
    //     //      pub _id: Option<ObjectId>,
    //     // pub first_name: String,
    //     // pub last_name: String,
    //     // pub email: String,
    //     // pub password: String,
    //     // pub created_at: Option<DateTime>,
    //     // pub updated_at: Option<DateTime>,

    //     let id = doc.get_object_id("_id")?;
    //     let first_name = doc.get_str("first_name")?;
    //     let last_name = doc.get_str("last_name")?;
    //     let email = doc.get_str("email")?;
    //     let password = doc.get_str("password")?;
    //     let created_at = doc.get_datetime("created_at")?;
    //     let updated_at = doc.get_datetime("updated_at")?;

    //     fn proj_id(proj: Option<ObjectId>) -> Option<String> {
    //         match proj {
    //             Some(proj) => Some(proj.to_hex()),
    //             None => None,
    //         }
    //     }

    //     let account = Account {
    //         _id: Some(id.to_hex()),
    //         first_name: Some(first_name.to_owned()),
    //         last_name: Some(last_name.to_owned()),
    //         email: email.to_owned(),
    //         password: password.to_owned(),
    //         created_at: Some(
    //             created_at
    //                 .to_chrono()
    //                 .to_rfc3339_opts(SecondsFormat::Secs, true),
    //         ),
    //         updated_at: updated_at
    //             .to_chrono()
    //             .to_rfc3339_opts(SecondsFormat::Secs, true),
    //     };

    //     Ok(account)
    // }

    pub async fn create_account(&self, _entry: &Account) -> Result<Bson, error::Error> {
        let new_account = self
            .get_accounts_collection()
            .insert_one(
                doc! {
                "email": _entry.name.clone(),
                "password": _entry.password.clone(),
                "created_at": chrono::Utc::now().clone(),
                "updated_at": chrono::Utc::now().clone(),
                },
                None,
            )
            .await
            .map_err(MongoQueryError)?;

        Ok(new_account.inserted_id)
    }
}
