use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "concerts",
            &[
                ("date", ColType::Date),
                ("disambiguation", ColType::StringNull),
                ("sort_order", ColType::IntegerNull),
                ("source", ColType::StringNull),
            ],
            &[("venue", ""), ("artist", "")],
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "concerts").await
    }
}
