#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;
use sea_orm::PaginatorTrait;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StatisticsMetadata {
    pub num_concerts: u64,
    pub num_countries: u64,
    pub num_cities: u64,
    pub num_venues: u64,
    pub num_performances: u64,
    pub num_songs: u64,
}

#[debug_handler]
pub async fn statistics(State(ctx): State<AppContext>) -> Result<Response> {
    let num_concerts = crate::models::_entities::concerts::Entity::find()
        .count(&ctx.db)
        .await?;
    let num_countries = crate::models::_entities::countries::Entity::find()
        .count(&ctx.db)
        .await?;
    let num_cities = crate::models::_entities::cities::Entity::find()
        .count(&ctx.db)
        .await?;
    let num_venues = crate::models::_entities::venues::Entity::find()
        .count(&ctx.db)
        .await?;
    let num_performances = crate::models::_entities::performances::Entity::find()
        .count(&ctx.db)
        .await?;
    let num_songs = crate::models::_entities::songs::Entity::find()
        .count(&ctx.db)
        .await?;

    format::json(StatisticsMetadata {
        num_concerts,
        num_countries,
        num_cities,
        num_venues,
        num_performances,
        num_songs,
    })
}

pub fn api_routes() -> Routes {
    Routes::new()
        .prefix("api/metadata/")
        .add("/statistics", get(statistics))
}
