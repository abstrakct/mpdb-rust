use crate::models::_entities::country_stats::Entity;
use loco_rs::prelude::*;
use sea_orm::Statement;

pub struct RefreshCountryStats;

#[async_trait]
impl Task for RefreshCountryStats {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "refresh_country_stats".to_string(),
            detail: "Refresh country_stats table".to_string(),
        }
    }
    async fn run(&self, ctx: &AppContext, _vars: &task::Vars) -> Result<()> {
        let sql = r#"
            INSERT INTO country_stats (country_id, num_cities, num_venues, num_concerts)
            SELECT
                c.id,
                COUNT(DISTINCT ci.id) AS num_cities,
                COUNT(DISTINCT v.id) AS num_venues,
                COUNT(DISTINCT co.id) AS num_concerts
            FROM countries c
            LEFT JOIN cities ci ON ci.country_id = c.id
            LEFT JOIN venues v ON v.city_id = ci.id
            LEFT JOIN concerts co ON co.venue_id = v.id
            GROUP BY c.id;
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
