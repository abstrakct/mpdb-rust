#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::_entities::cities::{ActiveModel, Entity, Model};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub name: Option<String>,
    pub country_id: i32,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.name = Set(self.name.clone().unwrap());
        item.country_id = Set(self.country_id);
    }
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

pub async fn list_cities(State(ctx): State<AppContext>) -> Result<Response> {
    format::json(Entity::find().all(&ctx.db).await?)
}

#[allow(clippy::default_trait_access)]
pub async fn add_city(
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Response> {
    let mut item: ActiveModel = Default::default();
    params.update(&mut item); // Updates item with values from params (!)
    let item = item.insert(&ctx.db).await?;
    format::json(item)
}

pub async fn remove_city(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    load_item(&ctx, id).await?.delete(&ctx.db).await?;
    format::empty()
}

pub async fn update_city(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Response> {
    let item = load_item(&ctx, id).await?;
    let mut item = item.into_active_model();
    params.update(&mut item); // Updates item with values from params (!)
    let item = item.update(&ctx.db).await?;
    format::json(item)
}

pub async fn get_one_city(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    format::json(load_item(&ctx, id).await?)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/cities/")
        .add("/", get(list_cities))
        .add("/", post(add_city))
        .add("/{id}", get(get_one_city))
        .add("/{id}", delete(remove_city))
        .add("/{id}", patch(update_city))
}
