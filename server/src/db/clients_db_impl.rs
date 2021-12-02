use crate::error;
use crate::error::Error::*;
use crate::models::client::{ClientRequest, ClientResponse};

use bson::Document;
use futures::StreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{self, doc, Bson};
use mongodb::Collection;

use super::{DB, DB_NAME};

impl DB {
    fn get_clients_collection(&self) -> Collection<Document> {
        self.client.database(DB_NAME).collection("clients")
    }

    pub fn doc_to_client(&self, doc: &Document) -> Result<ClientResponse, error::Error> {
        let id = doc.get_object_id("_id")?;
        let name = doc.get_str("name")?;
        let created_at = doc.get_datetime("created_at")?;
        let updated_at = doc.get_datetime("updated_at")?;

        let client = ClientResponse {
            _id: id.to_hex(),
            name: name.to_owned(),
            created_at: created_at.to_chrono().to_rfc3339(),
            updated_at: updated_at.to_chrono().to_rfc3339(),
        };

        Ok(client)
    }

    pub async fn get_all_clients(&self) -> Result<Vec<ClientResponse>, error::Error> {
        let mut cursor = self
            .get_clients_collection()
            .find(None, None)
            .await
            .map_err(MongoQueryError)?;

        let mut result: Vec<ClientResponse> = Vec::new();

        while let Some(doc) = cursor.next().await {
            result.push(self.doc_to_client(&doc?)?);
        }

        Ok(result)
    }

    pub async fn find_client(&self, id: &str) -> Result<ClientResponse, error::Error> {
        let oid = ObjectId::parse_str(id).map_err(|_| InvalidIDError(id.to_owned()))?;
        let query = doc! {
            "_id": oid,
        };

        let document = self
            .get_clients_collection()
            .find_one(query, None)
            .await
            .map_err(MongoQueryError)?;

        if document.is_none() {
            // return error::Err(warp::reject::not_found());
            return Err(ObjNotFound);
        }

        let result = self.doc_to_client(&document.unwrap())?;

        Ok(result)
    }

    pub async fn create_client(&self, _entry: &ClientRequest) -> Result<Bson, error::Error> {
        let new_client = self
            .get_clients_collection()
            .insert_one(
                doc! {
                "name": _entry.name.clone(),
                "created_at": chrono::Utc::now().clone(),
                "updated_at": chrono::Utc::now().clone(),
                },
                None,
            )
            .await
            .map_err(MongoQueryError)?;

        Ok(new_client.inserted_id)
    }

    pub async fn create_many_clients(
        &self,
        _entry: Vec<mongodb::bson::Document>,
    ) -> Result<(), error::Error> {
        self.get_clients_collection()
            .insert_many(_entry, None)
            .await
            .map_err(MongoQueryError)?;
        Ok(())
    }

    pub async fn delete_client(&self, id: &str) -> Result<String, error::Error> {
        let oid = ObjectId::parse_str(id).map_err(|_| InvalidIDError(id.to_owned()))?;
        let query = doc! {
            "_id": oid,
        };
        let deleted_result = self
            .get_clients_collection()
            .delete_one(query, None)
            .await
            .map_err(MongoQueryError)?;

        if deleted_result.deleted_count == 0 {
            return Err(ObjNotFound);
        }

        Ok(oid.to_hex())
    }

    pub async fn delete_all_clients(&self) -> Result<(), error::Error> {
        self.get_clients_collection()
            .delete_many(doc! {}, None)
            .await
            .map_err(MongoQueryError)?;

        Ok(())
    }

    pub async fn get_all_clients_ids(&self) -> Result<Vec<String>, error::Error> {
        let clients_ids = self
            .get_clients_collection()
            .distinct("_id", None, None)
            .await
            .map_err(MongoQueryError)?;

        let mut string_vec: Vec<String> = vec![];
        for item in &clients_ids {
            string_vec.push(item.as_object_id().unwrap().to_hex());
        }

        Ok(string_vec)
    }
}
