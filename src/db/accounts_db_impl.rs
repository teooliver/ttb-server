use crate::handle_errors;
use crate::handle_errors::Error::*;
use crate::models::account::{Account, NewAccount, Role};
use chrono::prelude::*;
use futures::StreamExt;
use mongodb::bson::{self, document, Bson};
use mongodb::bson::{doc, document::Document, oid::ObjectId};
use mongodb::options::IndexOptions;
use mongodb::{Collection, IndexModel};
use std::error::Error;
use std::str::FromStr;

use super::DB;

impl DB {
    pub fn get_accounts_collection(&self) -> Collection<Document> {
        self.client.database(&self.db_name).collection("accounts")
    }

    fn doc_to_account(&self, doc: &Document) -> Result<Account, handle_errors::Error> {
        let id = doc.get_object_id("_id")?;
        let first_name = doc.get_str("first_name").unwrap_or_else(|_| "");
        let last_name = doc.get_str("last_name").unwrap_or_else(|_| "");
        let email = doc.get_str("email")?;
        let password = doc.get_str("password")?;
        let created_at = doc.get_datetime("created_at")?;
        let updated_at = doc.get_datetime("updated_at")?;
        let role = doc.get_str("role")?;

        let account = Account {
            _id: Some(id),
            first_name: Some(first_name.to_owned()),
            last_name: Some(last_name.to_owned()),
            email: email.to_owned(),
            password: password.to_owned(),
            created_at: Some(*created_at),
            updated_at: Some(*updated_at),
            role: Role::from_str(role).unwrap(),
        };

        Ok(account)
    }

    pub async fn create_account(&self, _entry: &NewAccount) -> Result<Bson, handle_errors::Error> {
        let new_account = self
            .get_accounts_collection()
            .insert_one(
                doc! {
                "email": _entry.email.clone(),
                "password": _entry.password.clone(),
                "role": _entry.role.clone(),
                "created_at": chrono::Utc::now().clone(),
                "updated_at": chrono::Utc::now().clone(),
                },
                None,
            )
            .await
            .map_err(MongoQueryError)?;

        Ok(new_account.inserted_id)
    }

    pub async fn get_account(&self, email: &str) -> Result<Account, handle_errors::Error> {
        let query = doc! {
            "email": email,
        };

        let document = self
            .get_accounts_collection()
            .find_one(query, None)
            .await
            .map_err(MongoQueryError)?;

        if document.is_none() {
            return Err(ObjNotFound);
        }

        let result = self.doc_to_account(&document.unwrap())?;

        Ok(result)
    }

    pub async fn is_admin(&self, account_id: &ObjectId) -> Result<bool, handle_errors::Error> {
        let query = doc! {
            "_id": account_id,
        };

        let document = self
            .get_accounts_collection()
            .find_one(query, None)
            .await
            .map_err(MongoQueryError)?;

        if document.is_none() {
            return Err(ObjNotFound);
        }

        let result = self.doc_to_account(&document.unwrap())?;

        if result.role == Role::Admin {
            return Ok(true);
        } else {
            return Ok(false);
        }
    }
}
