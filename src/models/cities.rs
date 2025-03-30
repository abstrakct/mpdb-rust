pub use super::_entities::cities::{self, ActiveModel, Entity, Model};
use super::_entities::countries;
use super::_entities::venues;
use loco_rs::prelude::*;
use sea_orm::entity::prelude::*;
pub type Cities = Entity;

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
    /// Finds a city by the provided name
    ///
    /// # Errors
    ///
    /// When could not find city by the given name or DB query error
    pub async fn find_by_name(db: &DatabaseConnection, name: &str) -> ModelResult<Self> {
        let city = Cities::find()
            .filter(
                model::query::condition()
                    .eq(cities::Column::Name, name)
                    .build(),
            )
            .one(db)
            .await?;
        city.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// Finds a city by the provided slug
    ///
    /// # Errors
    ///
    /// When could not find city by the given slug or DB query error
    pub async fn find_by_slug(db: &DatabaseConnection, slug: &str) -> ModelResult<Self> {
        let city = Cities::find()
            .filter(
                model::query::condition()
                    .eq(cities::Column::Slug, slug)
                    .build(),
            )
            .one(db)
            .await?;
        city.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// Gets the country associated with this city
    ///
    /// # Arguments
    ///
    /// * `db` - Database connection
    ///
    /// # Returns
    ///
    /// The country model for this city
    ///
    /// # Errors
    ///
    /// When could not find the associated country or DB query error
    pub async fn country(&self, db: &DatabaseConnection) -> ModelResult<countries::Model> {
        countries::Entity::find_by_id(self.country_id)
            .one(db)
            .await?
            .ok_or_else(|| ModelError::EntityNotFound)
    }
}

impl ActiveModel {
    // implement your write-oriented logic here
}

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
    pub async fn find_all_by_country_id(
        db: &DatabaseConnection,
        country_id: i32,
    ) -> Result<Vec<Model>, DbErr> {
        Self::find()
            .filter(cities::Column::CountryId.eq(country_id))
            .all(db)
            .await
    }

    /// Find all cities with their related country information
    ///
    /// # Arguments
    ///
    /// * `db` - Database connection
    ///
    /// # Returns
    ///
    /// Vector of tuples containing (city, country)
    pub async fn find_all_with_countries(
        db: &DatabaseConnection,
    ) -> Result<Vec<(Model, Option<countries::Model>)>, DbErr> {
        Self::find()
            .find_also_related(countries::Entity)
            .all(db)
            .await
    }

    /// Find all cities with their related venues
    ///
    /// # Arguments
    ///
    /// * `db` - Database connection
    ///
    /// # Returns
    ///
    /// Vector of tuples containing (city, venue)
    pub async fn find_all_with_venues(
        db: &DatabaseConnection,
    ) -> Result<Vec<(Model, Option<venues::Model>)>, DbErr> {
        Self::find().find_also_related(venues::Entity).all(db).await
    }

    /// Find all cities with their related country and venues
    ///
    /// # Arguments
    ///
    /// * `db` - Database connection
    ///
    /// # Returns
    ///
    /// Vector of tuples containing (city, country, venue)
    pub async fn find_all_with_countries_and_venues(
        db: &DatabaseConnection,
    ) -> Result<Vec<(Model, Option<countries::Model>, Option<venues::Model>)>, DbErr> {
        Self::find()
            .find_also_related(countries::Entity)
            .find_also_related(venues::Entity)
            .all(db)
            .await
    }

    /// Find cities by name pattern (case-insensitive partial match)
    ///
    /// # Arguments
    ///
    /// * `db` - Database connection
    /// * `pattern` - Pattern to match against city names
    ///
    /// # Returns
    ///
    /// Vector of matching cities
    pub async fn find_all_by_name_pattern(
        db: &DatabaseConnection,
        pattern: &str,
    ) -> Result<Vec<Model>, DbErr> {
        Self::find()
            .filter(cities::Column::Name.contains(pattern))
            .all(db)
            .await
    }
}
