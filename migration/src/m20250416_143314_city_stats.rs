use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "city_stats",
            &[
                ("id", ColType::PkAuto),
                ("num_venues", ColType::IntegerNull),
                ("num_concerts", ColType::IntegerNull),
            ],
            &[("city", "")],
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "city_stats").await
    }
}
