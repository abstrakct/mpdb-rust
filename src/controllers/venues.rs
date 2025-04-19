#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use super::{cities::CityResponse, countries::CountryResponse};
use crate::models::_entities::venues::{ActiveModel, Entity, Model};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub name: String,
    pub city_id: i32,
    pub unique_name: String,
    pub slug: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VenueResponse {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub city: CityResponse,
}

impl From<(Model, CityResponse)> for VenueResponse {
    fn from((venue, city): (Model, CityResponse)) -> Self {
        Self {
            id: venue.id,
            name: venue.name,
            slug: venue.slug,
            city,
        }
    }
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.name = Set(self.name.clone());
        item.city_id = Set(self.city_id);
        item.unique_name = Set(self.unique_name.clone());
        item.slug = Set(self.slug.clone());
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

#[allow(clippy::unwrap_used)]
pub async fn list_with_details(State(ctx): State<AppContext>) -> Result<Response> {
    let items = Entity::find().all(&ctx.db).await?;

    let mut responses = Vec::new();
    for venue in items {
        // Get venue, city and country data
        let city = venue.city(&ctx.db).await?;
        let country = city.country(&ctx.db).await?;

        // Convert models to responses
        let country_response = CountryResponse::from(country);
        let city_response = CityResponse::from((city, country_response.clone()));

        responses.push(VenueResponse {
            id: venue.id,
            name: venue.name,
            slug: venue.slug,
            city: city_response,
        });
    }

    format::json(responses)
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

pub async fn load_by_slug(ctx: &AppContext, slug: String) -> Result<VenueResponse> {
    // TODO: maybe this can be moved to the Model?
    use crate::models::_entities::{cities, countries};
    let venue = Entity::find()
        .filter(crate::models::_entities::venues::Column::Slug.eq(slug))
        .find_also_related(cities::Entity)
        .and_also_related(countries::Entity)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    let city = venue.1.ok_or_else(|| Error::NotFound)?;
    let country = venue.2.ok_or_else(|| Error::NotFound)?;

    Ok(VenueResponse {
        id: venue.0.id,
        name: venue.0.name,
        slug: venue.0.slug,
        city: CityResponse {
            id: city.id,
            name: city.name.clone(),
            slug: city.slug.clone(),
            country: country.into(),
            stats: None,
        },
    })
}

#[debug_handler]
pub async fn get_one_by_slug(
    Path(slug): Path<String>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let result = load_by_slug(&ctx, slug).await?;
    format::json(result)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/venues/")
        .add("/", get(list))
        .add("/", post(add))
        .add("{id}", get(get_one))
        .add("{id}", delete(remove))
        .add("{id}", put(update))
        .add("{id}", patch(update))
        .add("/with-details", get(list_with_details))
}

pub fn api_by_slug_routes() -> Routes {
    Routes::new()
        .prefix("api/venues/by-slug/")
        .add("/{slug}", get(get_one_by_slug))
}
