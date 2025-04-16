use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "songtitles",
            &[
                ("id", ColType::PkAuto),
                ("title", ColType::String),
                ("is_default", ColType::Boolean),
                ("slug", ColType::StringUniq),
                ("alias_for", ColType::IntegerNull),
            ],
            &[("song", "")],
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "songtitles").await
    }
}
