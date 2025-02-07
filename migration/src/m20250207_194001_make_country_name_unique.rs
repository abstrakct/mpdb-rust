#[allow(unused_imports)]
use sea_orm_migration::{prelude::*, schema::*, sea_orm::Statement};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let db = m.get_connection();

        let sql = r#"
            ALTER TABLE "countries" ALTER COLUMN "name" SET NOT NULL;
        "#;
        db.execute(Statement::from_string(db.get_database_backend(), sql))
            .await?;

        let sql = r#"
            ALTER TABLE "countries" ADD CONSTRAINT "countries_name_unique" UNIQUE ("name");
        "#;
        db.execute(Statement::from_string(db.get_database_backend(), sql))
            .await?;

        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let db = m.get_connection();

        let sql = r#"
            ALTER TABLE "countries" DROP CONSTRAINT "countries_name_unique";
        "#;
        db.execute(Statement::from_string(db.get_database_backend(), sql))
            .await?;

        let sql = r#"
            ALTER TABLE "countries" ALTER COLUMN "name" DROP NOT NULL;
        "#;
        db.execute(Statement::from_string(db.get_database_backend(), sql))
            .await?;

        Ok(())
    }
}
