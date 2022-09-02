use crate::error::Result;
use crate::models::task::task::{NewTask, Task};

use actix_web::{
    get, post,
    web::{Data, Json, ServiceConfig},
    Responder,
};
use serde::Deserialize;
use sqlx::PgPool;
use validator::Validate;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(index);
    cfg.service(create);
}

#[get("/tasks")]
async fn index(pool: Data<PgPool>) -> Result<impl Responder> {
    let tasks = Task::find_all(&pool).await?;
    Ok(Json(tasks))
}

#[derive(Deserialize, Validate)]
pub struct CreateForm {
    #[validate(length(min = 1))]
    description: String,
}

#[post("/tasks")]
async fn create(pool: Data<PgPool>, form: Json<CreateForm>) -> Result<impl Responder> {
    form.validate()?;
    let new_task = NewTask {
        description: form.description.clone(),
    };
    let task = Task::insert(&pool, new_task).await?;
    Ok(Json(task))
}
