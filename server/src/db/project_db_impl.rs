use crate::models::project::{
    ProjectAfterAggregation, ProjectRequest, ProjectResponse, ProjectsGroupedByClient,
};
use crate::{error::Error::*, Result};
use futures::StreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, Bson, Document};
use mongodb::Collection;

use super::{DB, DB_NAME};

impl DB {
    pub fn get_projects_collection(&self) -> Collection<Document> {
        self.client.database(DB_NAME).collection("projects")
    }

    pub fn doc_to_project(&self, doc: &Document) -> Result<ProjectResponse> {
        let id = doc.get_object_id("_id")?;
        let client = doc.get_object_id("client")?;
        let name = doc.get_str("name")?;
        let color = doc.get_str("color")?;
        let created_at = doc.get_datetime("created_at")?;
        let updated_at = doc.get_datetime("updated_at")?;

        let project = ProjectResponse {
            _id: id.to_hex(),
            client: Some(client.to_hex()),
            name: name.to_owned(),
            color: color.to_owned(),
            created_at: created_at.to_chrono().to_rfc3339(),
            updated_at: updated_at.to_chrono().to_rfc3339(),
        };

        Ok(project)
    }

    fn doc_project_grouped_by_client(&self, doc: &Document) -> Result<ProjectsGroupedByClient> {
        let id = doc.get_str("_id")?;
        let projects = doc.get_array("projects")?;

        let mut projects_vec: Vec<ProjectAfterAggregation> = vec![];

        for item in projects {
            let project_doc = item.as_document().unwrap();
            let project_id = project_doc.get_object_id("_id")?;
            let name = project_doc.get_str("name")?;
            let color = project_doc.get_str("color")?;
            let client_name = project_doc.get_str("client_name")?;

            // Need Better Names
            let proj = ProjectAfterAggregation {
                _id: project_id.to_string(),
                name: name.to_string(),
                color: color.to_string(),
                client_name: client_name.to_string(),
            };

            projects_vec.push(proj);
        }

        let results = ProjectsGroupedByClient {
            _id: id.to_string(),
            projects: projects_vec,
        };

        Ok(results)
    }

    pub async fn find_project(&self, id: &str) -> Result<ProjectResponse> {
        let oid = ObjectId::parse_str(id).map_err(|_| InvalidIDError(id.to_owned()))?;
        let query = doc! {
            "_id": oid,
        };
        let document = self
            .get_projects_collection()
            .find_one(query, None)
            .await
            .map_err(MongoQueryError)?;

        // if document == None {
        //     // return error::Err(warp::reject::not_found());
        //     return Err(ObjNotFound);
        // }

        let result = self.doc_to_project(&document.expect("Document not found"))?;

        Ok(result)
    }

    pub async fn get_projects_grouped_by_client(&self) -> Result<Vec<ProjectsGroupedByClient>> {
        let lookup_clients = doc! {
            "$lookup": {
                "from": "clients",
                "localField": "client",
                "foreignField": "_id",
                "as": "client_name",
            }
        };

        let sort = doc! {
             "$sort": {
                "updatedAt": -1,
            },
        };

        let project = doc! {
            "$project": {
                "_id": "$_id",
                "name": "$name",
                "color": "$color",
                "client_name": { "$arrayElemAt": ["$client_name.name", 0] },
                "subprojects": "$subprojects",
            },
        };

        let group = doc! {
            "$group": {
                "_id": "$client_name",
                "projects": { "$push": "$$ROOT" },
             },
        };

        let pipeline = vec![lookup_clients, sort, project, group];

        let mut cursor = self
            .get_projects_collection()
            .aggregate(pipeline, None)
            .await?;

        let mut results: Vec<ProjectsGroupedByClient> = Vec::new();
        while let Some(doc) = cursor.next().await {
            results.push(self.doc_project_grouped_by_client(&doc?)?);
        }

        Ok(results)
    }

    pub async fn create_project(&self, _entry: &ProjectRequest) -> Result<Bson> {
        let mut client_opt: Option<ObjectId> = None;

        if !_entry.client.is_none() {
            let oid = ObjectId::parse_str(_entry.client.clone().unwrap().to_string())
                .map_err(|_| InvalidIDError(_entry.client.clone().unwrap().to_string()))?;
            client_opt = Some(oid);
        };

        let new_project = self
            .get_projects_collection()
            .insert_one(
                doc! {
                "name": _entry.name.clone(),
                "color": _entry.color.clone(),
                "client": client_opt,
                "created_at": chrono::Utc::now().clone(),
                "updated_at": chrono::Utc::now().clone(),
                },
                None,
            )
            .await
            .map_err(MongoQueryError)?;

        Ok(new_project.inserted_id)
    }

    pub async fn delete_project(&self, id: &str) -> Result<()> {
        let oid = ObjectId::parse_str(id).map_err(|_| InvalidIDError(id.to_owned()))?;
        let query = doc! {
            "_id": oid,
        };
        self.get_projects_collection()
            .delete_one(query, None)
            .await
            .map_err(MongoQueryError)?;

        Ok(())
    }

    pub async fn delete_all_projects(&self) -> Result<()> {
        self.get_projects_collection()
            .delete_many(doc! {}, None)
            .await
            .map_err(MongoQueryError)?;

        Ok(())
    }

    pub async fn create_many_projects(&self, _entry: Vec<mongodb::bson::Document>) -> Result<()> {
        self.get_projects_collection()
            .insert_many(_entry, None)
            .await
            .map_err(MongoQueryError)?;
        Ok(())
    }

    pub async fn get_all_projects_ids(&self) -> Result<Vec<String>> {
        let projects_ids = self
            .get_projects_collection()
            .distinct("_id", None, None)
            .await
            .map_err(MongoQueryError)?;

        let mut string_vec: Vec<String> = vec![];
        for item in &projects_ids {
            string_vec.push(item.as_object_id().unwrap().to_hex());
        }

        Ok(string_vec)
    }
}
