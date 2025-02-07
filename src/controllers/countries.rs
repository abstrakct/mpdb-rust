#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::_entities::{
    cities,
    countries::{ActiveModel, Entity, Model},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub name: Option<String>,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.name = Set(self.name.clone().unwrap());
    }
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

pub async fn list_countries(State(ctx): State<AppContext>) -> Result<Response> {
    format::json(Entity::find().all(&ctx.db).await?)
}

pub async fn list_cities(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    let country = load_item(&ctx, id).await?;
    let cities = country.find_related(cities::Entity).all(&ctx.db).await?;
    format::json(cities)
}

pub async fn add_country(
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Response> {
    let mut item: ActiveModel = Default::default();
    params.update(&mut item); // Updates item with values from params (!)
    let item = item.insert(&ctx.db).await?;
    format::json(item)
}

pub async fn remove_country(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    load_item(&ctx, id).await?.delete(&ctx.db).await?;
    format::empty()
}

pub async fn update_country(
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

pub async fn get_one_country(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    format::json(load_item(&ctx, id).await?)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/countries/")
        .add("/", get(list_countries))
        .add("/", post(add_country))
        .add("/{id}", get(get_one_country))
        .add("/{id}", delete(remove_country))
        .add("/{id}", patch(update_country))
        .add("/{id}/cities", get(list_cities))
}
