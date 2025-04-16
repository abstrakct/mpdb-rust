#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;
mod m20220101_000001_users;

mod m20250203_181428_countries;
mod m20250207_194001_make_country_name_unique;
mod m20250207_204004_cities;
mod m20250208_120854_venues;
mod m20250209_104354_artists;
mod m20250211_175025_concerts;
mod m20250211_175304_songs;
mod m20250211_180236_songtitles;
mod m20250211_181553_sets;
mod m20250211_181843_performances;
mod m20250416_101511_country_stats;
mod m20250416_143314_city_stats;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20250203_181428_countries::Migration),
            Box::new(m20250207_194001_make_country_name_unique::Migration),
            Box::new(m20250207_204004_cities::Migration),
            Box::new(m20250208_120854_venues::Migration),
            Box::new(m20250209_104354_artists::Migration),
            Box::new(m20250211_175025_concerts::Migration),
            Box::new(m20250211_175304_songs::Migration),
            Box::new(m20250211_180236_songtitles::Migration),
            Box::new(m20250211_181553_sets::Migration),
            Box::new(m20250211_181843_performances::Migration),
            Box::new(m20250416_101511_country_stats::Migration),
            Box::new(m20250416_143314_city_stats::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}