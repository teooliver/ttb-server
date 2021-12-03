use crate::db::DB;
use crate::error::Error::InvalidIDError;
use crate::{models::project::ProjectRequest, WebResult};
use fake::{self, Fake};
use mongodb::bson::Document;
use mongodb::bson::{doc, oid::ObjectId};
use rand::Rng;
use warp::{http::StatusCode, Reply};

pub const PROJECT_COLORS: [&str; 10] = [
    "#61e294ff",
    "#7bcdbaff",
    "#9799caff",
    "#bd93d8ff",
    "#b47aeaff",
    "#d3d5d4ff",
    "#a2c5acff",
    "#9db5b2ff",
    "#878e99ff",
    "#7f6a93ff",
];

// pub const CLIENT_NAMES = [];
// pub const PROJECT_NAMES = [];
// pub const TASK_NAMES = [];

pub fn generate_clients_data(amount: u8) -> Vec<mongodb::bson::Document> {
    let mut clients: Vec<mongodb::bson::Document> = vec![];

    for _n in 1..amount {
        clients.push(doc! {
            "name": fake::faker::company::en::CompanyName().fake::<String>().to_string(),
            "created_at": chrono::Utc::now().clone(),
            "updated_at": chrono::Utc::now().clone(),
        });
    }

    clients
}

pub fn create_project(clients_ids: Vec<String>) -> ProjectRequest {
    let rng_color_index = rand::thread_rng().gen_range(0..(PROJECT_COLORS.len() - 1));
    let rng_client_index = rand::thread_rng().gen_range(0..(clients_ids.len() - 1));

    let client_id = ObjectId::parse_str(clients_ids[rng_client_index].to_string())
        .map_err(|_| InvalidIDError(clients_ids[rng_client_index].to_owned()))
        .unwrap();

    let new_project = ProjectRequest {
        client: client_id,
        name: fake::faker::company::en::CompanyName().fake(),
        color: PROJECT_COLORS[rng_color_index].to_string(),
    };

    new_project
}

pub fn generate_projects_data(
    amount: u8,
    clients_ids: Vec<String>,
) -> Vec<mongodb::bson::Document> {
    let mut projects: Vec<mongodb::bson::Document> = vec![];

    for _n in 1..amount {
        let project = create_project(clients_ids.clone());
        projects.push(doc! {
            "client": project.client,
            "name": project.name.to_string(),
            "color": project.color.to_string(),
            // "estimate": project.estimate.to_string(),
            "created_at": chrono::Utc::now().clone(),
            "updated_at": chrono::Utc::now().clone(),
        });
    }

    projects
}

fn create_task(project_ids: Vec<String>) -> Document {
    let rng_project_index = rand::thread_rng().gen_range(0..(project_ids.len() - 1));

    let project_id = ObjectId::parse_str(project_ids[rng_project_index].to_string())
        .map_err(|_| InvalidIDError(project_ids[rng_project_index].to_owned()))
        .unwrap();

    let new_task = doc! {
        "name": fake::faker::company::en::CompanyName().fake::<String>().to_string(),
        "initial_time": chrono::Utc::now(),
        "end_time": chrono::Utc::now(),
        "project": Some(project_id),
        "created_at": chrono::Utc::now(),
        "updated_at": chrono::Utc::now(),
    };

    new_task
}

pub fn generate_tasks_data(amount: u8, clients_ids: Vec<String>) -> Vec<mongodb::bson::Document> {
    let mut tasks: Vec<mongodb::bson::Document> = vec![];

    for _n in 1..amount {
        let task = create_task(clients_ids.clone());
        tasks.push(task);
    }

    tasks
}

pub async fn seed_clients(db: DB) -> WebResult<impl Reply> {
    db.delete_all_clients().await?;
    db.delete_all_projects().await?;
    db.delete_all_tasks().await?;

    db.create_many_clients(generate_clients_data(10)).await?;

    Ok(StatusCode::OK)
}

pub async fn seed_projects(db: DB) -> WebResult<impl Reply> {
    db.delete_all_projects().await?;
    db.delete_all_tasks().await?;

    let client_ids = db.get_all_clients_ids().await?;

    db.create_many_projects(generate_projects_data(10, client_ids))
        .await?;

    Ok(StatusCode::OK)
}

pub async fn seed_tasks(db: DB) -> WebResult<impl Reply> {
    db.delete_all_tasks().await?;

    let projects_ids = db.get_all_projects_ids().await?;

    db.create_many_tasks(generate_tasks_data(10, projects_ids))
        .await?;

    Ok(StatusCode::OK)
}

pub async fn seed_all_data(db: DB) -> WebResult<impl Reply> {
    seed_clients(db.clone()).await?;
    seed_projects(db.clone()).await?;
    seed_tasks(db.clone()).await?;

    Ok(StatusCode::OK)
}

pub async fn remove_all_data(db: DB) -> WebResult<impl Reply> {
    db.delete_all_clients().await?;
    db.delete_all_projects().await?;
    db.delete_all_tasks().await?;

    Ok(StatusCode::OK)
}
