#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;
// use sea_orm::Statement;
use serde::{Deserialize, Serialize};
use tracing::log::debug;

use super::venues::VenueResponse;
use crate::models::_entities::{
    cities,
    concerts::{ActiveModel, Entity, Model},
    venues,
};

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
    // use futures::future::try_join_all;
    // let sql = Statement {
    //     sql: "select c.id, c.date, c.slug, v.name as venue_name, cit.name as city_name, co.name as country_name
    //             from concerts c
    //             left join venues v on c.venue_id = v.id
    //             left join cities cit on v.city_id = cit.id
    //             left join countries co on cit.country_id = co.id
    //             ".to_string(),
    //     values: None,
    //     db_backend: ctx.db.get_database_backend(),
    // };

    // let query_result = ctx.db.query_all(sql).await?;

    let ctx = &ctx;
    let items = Entity::find()
        .find_also_related(venues::Entity)
        .and_also_related(cities::Entity)
        .all(&ctx.db)
        .await?;

    // let result = try_join_all(
    //     items
    //         .into_iter()
    //         .map(async move |(concert, venue)| {
    //             let venue = super::venues::load_by_slug(ctx, venue.unwrap().slug.clone()).await?;

    //             Ok::<_, Error>(ConcertResponse {
    //                 id: concert.id,
    //                 date: concert.date,
    //                 disambiguation: concert.disambiguation,
    //                 source: concert.source,
    //                 sort_order: concert.sort_order,
    //                 venue,
    //                 artist_id: concert.artist_id,
    //                 slug: concert.slug,
    //             })
    //         })
    //         .collect::<Vec<_>>(),
    // )
    // .await?;

    format::json(items)
    // format::json(Entity::find().all(&ctx.db).await?)
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

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/concerts/")
        .add("/", get(list))
        .add("/", post(add))
        .add("/new", post(add_new))
        .add("{id}", get(get_one))
        .add("{id}", delete(remove))
        .add("{id}", put(update))
        .add("{id}", patch(update))
}
