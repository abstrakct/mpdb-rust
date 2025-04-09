#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use super::performances::PerformanceResponse;
use crate::models::_entities::sets::{ActiveModel, Entity, Model};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub name: Option<String>,
    pub unique_name: String,
    pub concert_id: i32,
    pub sort_order: i32,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.name = Set(self.name.clone());
        item.unique_name = Set(self.unique_name.clone());
        item.concert_id = Set(self.concert_id);
        item.sort_order = Set(self.sort_order);
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SetResponse {
    pub id: i32,
    pub name: Option<String>,
    pub unique_name: String,
    pub sort_order: i32,
    pub performances: Option<Vec<PerformanceResponse>>,
}

impl From<Model> for SetResponse {
    fn from(item: Model) -> Self {
        Self {
            id: item.id,
            name: item.name,
            unique_name: item.unique_name,
            sort_order: item.sort_order,
            performances: None,
        }
    }
}

impl From<(Model, Vec<PerformanceResponse>)> for SetResponse {
    fn from((item, performances): (Model, Vec<PerformanceResponse>)) -> Self {
        Self {
            id: item.id,
            name: item.name,
            unique_name: item.unique_name,
            sort_order: item.sort_order,
            performances: Some(performances),
        }
    }
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

#[debug_handler]
pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    format::json(Entity::find().all(&ctx.db).await?)
}

#[debug_handler]
pub async fn add(State(ctx): State<AppContext>, Json(params): Json<Params>) -> Result<Response> {
    let mut item = ActiveModel {
        ..Default::default()
    };
    params.update(&mut item);
    let item = item.insert(&ctx.db).await?;
    format::json(item)
}

#[debug_handler]
pub async fn update(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Response> {
    let item = load_item(&ctx, id).await?;
    let mut item = item.into_active_model();
    params.update(&mut item);
    let item = item.update(&ctx.db).await?;
    format::json(item)
}

#[debug_handler]
pub async fn remove(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    load_item(&ctx, id).await?.delete(&ctx.db).await?;
    format::empty()
}

#[debug_handler]
pub async fn get_one(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    format::json(load_item(&ctx, id).await?)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/sets/")
        .add("/", get(list))
        .add("/", post(add))
        .add("{id}", get(get_one))
        .add("{id}", delete(remove))
        .add("{id}", put(update))
        .add("{id}", patch(update))
}
