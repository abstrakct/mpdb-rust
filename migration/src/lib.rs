#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;
mod m20220101_000001_users;

mod m20250203_181428_countries;
mod m20250207_184744_remove_uuid_from_countries;
mod m20250207_194001_make_country_name_unique;
mod m20250207_204004_cities;
mod m20250208_120854_venues;
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
            // inject-above (do not remove this comment)
        ]
    }
}
