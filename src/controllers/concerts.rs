#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;
// use sea_orm::Statement;
use serde::{Deserialize, Serialize};
use tracing::log::debug;

use super::{
    cities::CityResponse, countries::CountryResponse, performances::PerformanceResponse,
    sets::SetResponse, venues::VenueResponse,
};
use crate::models::_entities::concerts::{ActiveModel, Entity, Model};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub date: Date,
    pub disambiguation: Option<String>,
    pub source: Option<String>,
    pub sort_order: Option<i32>,
    pub venue_id: i32,
    pub artist_id: i32,
    pub slug: String,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.date = Set(self.date);
        item.disambiguation = Set(self.disambiguation.clone());
        item.source = Set(self.source.clone());
        item.sort_order = Set(self.sort_order);
        item.venue_id = Set(self.venue_id);
        item.artist_id = Set(self.artist_id);
        item.slug = Set(self.slug.clone());
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConcertResponse {
    pub id: i32,
    pub date: Date,
    pub disambiguation: Option<String>,
    pub source: Option<String>,
    pub sort_order: Option<i32>,
    pub venue: VenueResponse,
    pub artist_id: i32,
    pub slug: String,
    pub sets: Option<Vec<SetResponse>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputData {
    pub status: String,
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

#[debug_handler]
pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    let ctx = &ctx;
    let items = Entity::find().all(&ctx.db).await?;

    format::json(items)
}

#[debug_handler]
pub async fn list_with_details(State(ctx): State<AppContext>) -> Result<Response> {
    let ctx = &ctx;
    let items = Model::find_all_with_venue_and_artist(&ctx.db).await?;

    let mut responses = Vec::new();
    for (concert, venue_opt, _artist_opt) in items {
        // Get venue, city and country data
        if let Some(venue) = venue_opt {
            let city = venue.city(&ctx.db).await?;
            let country = city.country(&ctx.db).await?;

            // Convert models to responses
            let country_response = CountryResponse::from(country);
            let city_response = CityResponse::from((city, country_response.clone()));
            let venue_response = VenueResponse::from((venue, city_response.clone()));

            responses.push(ConcertResponse {
                id: concert.id,
                date: concert.date,
                disambiguation: concert.disambiguation,
                source: concert.source,
                sort_order: concert.sort_order,
                venue: venue_response,
                artist_id: concert.artist_id,
                slug: concert.slug,
                sets: None,
            });
        }
    }

    format::json(responses)
}

#[debug_handler]
pub async fn add(State(ctx): State<AppContext>, Json(params): Json<Params>) -> Result<Response> {
    let mut item = ActiveModel {
        ..Default::default()
    };
    debug!("Received: {:?}", params);
    params.update(&mut item);
    let item = item.insert(&ctx.db).await?;
    format::json(item)
}

#[debug_handler]
pub async fn add_new(
    State(ctx): State<AppContext>,
    Json(_input): Json<InputData>,
) -> Result<Response> {
    let item = ActiveModel {
        ..Default::default()
    };
    //debug!("Received: {:?}", params);
    //params.update(&mut item);
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

#[debug_handler]
pub async fn get_one_by_slug(
    Path(slug): Path<String>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let concert = Model::find_by_slug(&ctx.db, &slug).await?;
    let venue = crate::models::venues::Entity::find_by_id(concert.venue_id)
        .one(&ctx.db)
        .await?;
    let venue = venue.ok_or_else(|| Error::NotFound)?;
    let city = venue.city(&ctx.db).await?;
    let country = city.country(&ctx.db).await?;
    let sets = concert.sets(&ctx.db).await?;

    let mut setsdata: Vec<(crate::models::sets::Model, Vec<PerformanceResponse>)> = Vec::new();
    for set in sets.clone() {
        let performances = set.performances(&ctx.db).await?;
        let mut perf_responses: Vec<PerformanceResponse> = Vec::new();
        for p in performances {
            let songtitle = p.songtitle_as_str(&ctx.db).await?;
            let perftitle = p.performancetitle_as_str(&ctx.db).await?;
            let perfslug = p.performancetitle_slug(&ctx.db).await?;
            perf_responses.push(PerformanceResponse {
                id: p.id,
                sort_order: p.sort_order,
                performance_title: if perftitle.as_deref() != songtitle.as_deref() {
                    Some(perftitle.unwrap())
                } else {
                    None
                },
                songtitle: songtitle.unwrap(),
                slug: perfslug.unwrap(),
            });
        }
        setsdata.push((set.clone(), perf_responses));
    }

    // Convert models to responses
    let country_response = CountryResponse::from(country);
    let city_response = CityResponse::from((city, country_response.clone()));
    let venue_response = VenueResponse::from((venue, city_response.clone()));
    let sets_response: Vec<SetResponse> = setsdata.into_iter().map(SetResponse::from).collect();

    let response = ConcertResponse {
        id: concert.id,
        date: concert.date,
        disambiguation: concert.disambiguation,
        source: concert.source,
        sort_order: concert.sort_order,
        venue: venue_response,
        artist_id: concert.artist_id,
        slug: concert.slug,
        sets: Some(sets_response),
    };

    format::json(response)
}

pub fn api_routes() -> Routes {
    Routes::new()
        .prefix("api/concerts/")
        .add("/", get(list))
        .add("/with-details", get(list_with_details))
        .add("/", post(add))
        .add("/new", post(add_new))
        .add("{id}", get(get_one))
        .add("{id}", delete(remove))
        .add("{id}", put(update))
        .add("{id}", patch(update))
}

pub fn api_by_slug_routes() -> Routes {
    Routes::new()
        .prefix("api/concerts/by-slug/")
        .add("{slug}", get(get_one_by_slug))
}
