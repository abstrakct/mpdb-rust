use crate::models::_entities::city_stats::Entity;
use loco_rs::prelude::*;
use sea_orm::Statement;

pub struct RefreshCityStats;

#[async_trait]
impl Task for RefreshCityStats {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "refresh_city_stats".to_string(),
            detail: "Refresh city_stats table".to_string(),
        }
    }
    async fn run(&self, ctx: &AppContext, _vars: &task::Vars) -> Result<()> {
        let sql = r#"
            INSERT INTO city_stats (city_id, num_venues, num_concerts)
            SELECT
                ci.id,
                COUNT(DISTINCT v.id) as num_venues,
                COUNT(DISTINCT co.id) as num_concerts
            FROM cities ci
            LEFT JOIN venues v ON v.city_id = ci.id
            LEFT JOIN concerts co ON co.venue_id = v.id
            GROUP BY ci.id;
        "#;

        // Delete all entries in country_stats table
        Entity::delete_many().exec(&ctx.db).await?;

        // Execute statement to populate country_stats table
        ctx.db
            .execute(Statement::from_string(
                ctx.db.get_database_backend(),
                sql.to_owned(),
            ))
            .await?;

        Ok(())
    }
}
