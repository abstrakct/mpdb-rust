use super::_entities::cities;
pub use super::_entities::countries::{self, ActiveModel, Entity, Model};
use super::_entities::venues;
use loco_rs::prelude::*;
use sea_orm::entity::prelude::*;
pub type Countries = Entity;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, insert: bool) -> std::result::Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if !insert && self.updated_at.is_unchanged() {
            let mut this = self;
            this.updated_at = sea_orm::ActiveValue::Set(chrono::Utc::now().into());
            Ok(this)
        } else {
            Ok(self)
        }
    }
}

// implement your read-oriented logic here
impl Model {
    /// Finds a country by the provided name
    ///
    /// # Errors
    ///
    /// When could not find country by the given name or DB query error
    pub async fn find_by_name(db: &DatabaseConnection, name: &str) -> ModelResult<Self> {
        let country = Countries::find()
            .filter(
                model::query::condition()
                    .eq(countries::Column::Name, name)
                    .build(),
            )
            .one(db)
            .await?;
        country.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// Finds a country by the provided slug
    ///
    /// # Errors
    ///
    /// When could not find country by the given slug or DB query error
    pub async fn find_by_slug(db: &DatabaseConnection, slug: &str) -> ModelResult<Self> {
        let country = Countries::find()
            .filter(
                model::query::condition()
                    .eq(countries::Column::Slug, slug)
                    .build(),
            )
            .one(db)
            .await?;
        country.ok_or_else(|| ModelError::EntityNotFound)
    }
}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {
    /// Find all cities in a country
    ///
    /// # Arguments
    ///
    /// * `db` - Database connection
    /// * `country_id` - ID of the country to find cities for
    ///
    /// # Returns
    ///
    /// Vector of cities in the country
    pub async fn find_all_cities(
        db: &DatabaseConnection,
        country_id: i32,
    ) -> Result<Vec<cities::Model>, DbErr> {
        cities::Entity::find()
            .filter(cities::Column::CountryId.eq(country_id))
            .all(db)
            .await
    }

    /// Find all venues in a country
    ///
    /// # Arguments
    ///
    /// * `db` - Database connection
    /// * `country_id` - ID of the country to find venues for
    ///
    /// # Returns
    ///
    /// Vector of venues in the country
    pub async fn find_all_venues(
        db: &DatabaseConnection,
        country_id: i32,
    ) -> Result<Vec<venues::Model>, DbErr> {
        let city_ids = cities::Entity::find()
            .filter(cities::Column::CountryId.eq(country_id))
            .all(db)
            .await?
            .into_iter()
            .map(|city| city.id)
            .collect::<Vec<i32>>();

        venues::Entity::find()
            .filter(venues::Column::CityId.is_in(city_ids))
            .all(db)
            .await
    }

    /// Find all cities in a country with their related information
    ///
    /// # Arguments
    ///
    /// * `db` - Database connection
    /// * `country_id` - ID of the country to find cities for
    ///
    /// # Returns
    ///
    /// Vector of cities with their related information
    pub async fn find_all_cities_with_venues(
        db: &DatabaseConnection,
        country_id: i32,
    ) -> Result<Vec<(cities::Model, Vec<super::_entities::venues::Model>)>, DbErr> {
        cities::Entity::find()
            .filter(cities::Column::CountryId.eq(country_id))
            .find_with_related(super::_entities::venues::Entity)
            .all(db)
            .await
    }
}
