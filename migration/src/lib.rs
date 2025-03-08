#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;
mod m20220101_000001_users;

mod m20250203_181428_countries;
mod m20250207_184744_remove_uuid_from_countries;
mod m20250207_194001_make_country_name_unique;
mod m20250207_204004_cities;
mod m20250208_120854_venues;
mod m20250208_123050_add_unique_name_to_venues;
mod m20250209_104354_artists;
mod m20250211_175025_concerts;
mod m20250211_175304_songs;
mod m20250211_180236_songtitles;
mod m20250211_181553_sets;
mod m20250211_181843_performances;
mod m20250223_115919_add_slug_to_countries;
mod m20250223_120940_add_slug_to_cities;
mod m20250223_130036_add_slug_to_venues;
mod m20250223_143004_add_slug_to_songtitles;
mod m20250223_143837_add_slug_to_artists;
mod m20250228_192109_add_alias_for_to_songtitles;
mod m20250308_154450_add_artist_id_to_concerts;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20250203_181428_countries::Migration),
            Box::new(m20250207_184744_remove_uuid_from_countries::Migration),
            Box::new(m20250207_194001_make_country_name_unique::Migration),
            Box::new(m20250207_204004_cities::Migration),
            Box::new(m20250208_120854_venues::Migration),
            Box::new(m20250208_123050_add_unique_name_to_venues::Migration),
            Box::new(m20250209_104354_artists::Migration),
            Box::new(m20250211_175025_concerts::Migration),
            Box::new(m20250211_175304_songs::Migration),
            Box::new(m20250211_180236_songtitles::Migration),
            Box::new(m20250211_181553_sets::Migration),
            Box::new(m20250211_181843_performances::Migration),
            Box::new(m20250223_115919_add_slug_to_countries::Migration),
            Box::new(m20250223_120940_add_slug_to_cities::Migration),
            Box::new(m20250223_130036_add_slug_to_venues::Migration),
            Box::new(m20250223_143004_add_slug_to_songtitles::Migration),
            Box::new(m20250223_143837_add_slug_to_artists::Migration),
            Box::new(m20250228_192109_add_alias_for_to_songtitles::Migration),
            Box::new(m20250308_154450_add_artist_id_to_concerts::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}