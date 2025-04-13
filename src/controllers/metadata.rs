#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;
use sea_orm::PaginatorTrait;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StatisticsMetadata {
    pub concerts_count: u64,
    pub countries_count: u64,
    pub cities_count: u64,
    pub venues_count: u64,
    pub performances_count: u64,
    pub songs_count: u64,
}

#[debug_handler]
pub async fn statistics(State(ctx): State<AppContext>) -> Result<Response> {
    let concerts_count = crate::models::_entities::concerts::Entity::find()
        .count(&ctx.db)
        .await?;
    let countries_count = crate::models::_entities::countries::Entity::find()
        .count(&ctx.db)
        .await?;
    let cities_count = crate::models::_entities::cities::Entity::find()
        .count(&ctx.db)
        .await?;
    let venues_count = crate::models::_entities::venues::Entity::find()
        .count(&ctx.db)
        .await?;
    let performances_count = crate::models::_entities::performances::Entity::find()
        .count(&ctx.db)
        .await?;
    let songs_count = crate::models::_entities::songs::Entity::find()
        .count(&ctx.db)
        .await?;

    format::json(StatisticsMetadata {
        concerts_count,
        countries_count,
        cities_count,
        venues_count,
        performances_count,
        songs_count,
    })
}

pub fn api_routes() -> Routes {
    Routes::new()
        .prefix("api/metadata/")
        .add("/statistics", get(statistics))
}
