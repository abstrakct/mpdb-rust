#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;
use sea_orm::{sea_query::Order, QueryOrder};
use serde::{Deserialize, Serialize};

use crate::models::_entities::cities::{ActiveModel, Column, Entity, Model};
use crate::models::_entities::countries;
use crate::views;

use super::countries::CountryResponse;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub name: Option<String>,
    pub country_id: i32,
    pub slug: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct CityResponse {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub country: CountryResponse,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.name = Set(self.name.clone().unwrap());
        item.country_id = Set(self.country_id);
        item.slug = Set(self.slug.clone().unwrap());
    }
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

async fn load_item_with_country(
    ctx: &AppContext,
    id: i32,
) -> Result<(Model, Option<countries::Model>)> {
    let item = Entity::find_by_id(id)
        .find_also_related(countries::Entity)
        .one(&ctx.db)
        .await?;
    // let country = item?.find_related(countries::Entity).one(&ctx.db).await?;

    item.ok_or_else(|| Error::NotFound)
}

pub async fn list_cities(State(ctx): State<AppContext>) -> Result<Response> {
    format::json(Entity::find().all(&ctx.db).await?)
}

pub async fn list_cities_with_countries(State(ctx): State<AppContext>) -> Result<Response> {
    let data = Entity::find()
        .find_also_related(countries::Entity)
        .all(&ctx.db)
        .await?;

    let result = data
        .into_iter()
        .map(|(city, country)| CityResponse {
            id: city.id,
            name: city.name,
            slug: city.slug,
            country: country.unwrap().into(),
        })
        .collect::<Vec<CityResponse>>();

    format::json(result)
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

#[debug_handler]
pub async fn list(
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let items = Entity::find()
        .order_by(Column::Name, Order::Desc)
        .all(&ctx.db)
        .await?;

    views::cities::list(&v, &items)
}

pub fn api_routes() -> Routes {
    Routes::new()
        .prefix("api/cities/")
        .add("/", get(list_cities))
        .add("/", post(add_city))
        .add("/{id}", get(get_one_city))
        .add("/{id}", delete(remove_city))
        .add("/{id}", patch(update_city))
        .add("/with-countries", get(list_cities_with_countries))
}

pub fn web_routes() -> Routes {
    Routes::new().prefix("cities/").add("/", get(list))
}
