pub use super::_entities::venues::{self, ActiveModel, Entity, Model};
use super::_entities::{cities, concerts, countries};
use loco_rs::prelude::*;
use sea_orm::entity::prelude::*;
use sea_orm::{QuerySelect, QueryTrait};
pub type Venues = Entity;

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
    /// Finds a venue by the provided name
    ///
    /// # Arguments
    ///
    /// * `db` - Database connection
    /// * `name` - Name of the venue to find
    ///
    /// # Returns
    ///
    /// The venue if found, or a ModelError if not found
    pub async fn find_by_name(db: &DatabaseConnection, name: &str) -> ModelResult<Self> {
        let venue = Venues::find()
            .filter(
                model::query::condition()
                    .eq(venues::Column::Name, name)
                    .build(),
            )
            .one(db)
            .await?;
        venue.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// Finds a venue by the provided slug
    ///
    /// # Arguments
    ///
    /// * `db` - Database connection
    /// * `slug` - Slug of the venue to find
    ///
    /// # Returns
    ///
    /// The venue if found, or a ModelError if not found
    pub async fn find_by_slug(db: &DatabaseConnection, slug: &str) -> ModelResult<Self> {
        let venue = Venues::find()
            .filter(
                model::query::condition()
                    .eq(venues::Column::Slug, slug)
                    .build(),
            )
            .one(db)
            .await?;
        venue.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// Finds a venue by its unique name
    ///
    /// # Arguments
    ///
    /// * `db` - Database connection
    /// * `unique_name` - Unique name of the venue to find
    ///
    /// # Returns
    ///
    /// The venue if found, or a ModelError if not found
    pub async fn find_by_unique_name(
        db: &DatabaseConnection,
        unique_name: &str,
    ) -> ModelResult<Self> {
        let venue = Venues::find()
            .filter(
                model::query::condition()
                    .eq(venues::Column::UniqueName, unique_name)
                    .build(),
            )
            .one(db)
            .await?;
        venue.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// Finds all venues in a city
    ///
    /// # Arguments
    ///
    /// * `db` - Database connection
    /// * `city_id` - ID of the city to find venues for
    ///
    /// # Returns
    ///
    /// Vector of venues in the city
    pub async fn find_by_city_id(
        db: &DatabaseConnection,
        city_id: i32,
    ) -> Result<Vec<Self>, DbErr> {
        Venues::find()
            .filter(venues::Column::CityId.eq(city_id))
            .all(db)
            .await
    }

    /// Finds venues by name pattern (case-insensitive partial match)
    ///
    /// # Arguments
    ///
    /// * `db` - Database connection
    /// * `pattern` - Pattern to match against venue names
    ///
    /// # Returns
    ///
    /// Vector of matching venues
    pub async fn find_by_name_pattern(
        db: &DatabaseConnection,
        pattern: &str,
    ) -> Result<Vec<Self>, DbErr> {
        Venues::find()
            .filter(venues::Column::Name.contains(pattern))
            .all(db)
            .await
    }

    /// Gets the city associated with this venue
    ///
    /// # Arguments
    ///
    /// * `db` - Database connection
    ///
    /// # Returns
    ///
    /// The city model for this venue
    ///
    /// # Errors
    ///
    /// When could not find the associated city or DB query error
    pub async fn city(&self, db: &DatabaseConnection) -> ModelResult<cities::Model> {
        cities::Entity::find_by_id(self.city_id)
            .one(db)
            .await?
            .ok_or_else(|| ModelError::EntityNotFound)
    }

    /// Gets the country associated with this venue
    ///
    /// # Arguments
    ///
    /// * `db` - Database connection
    ///
    /// # Returns
    ///
    /// The country model for this venue
    ///
    /// # Errors
    ///
    /// When could not find the associated country or DB query error
    pub async fn country(&self, db: &DatabaseConnection) -> ModelResult<countries::Model> {
        let city = self.city(db).await?;
        city.country(db).await
    }

    /// Gets all concerts at this venue
    ///
    /// # Arguments
    ///
    /// * `db` - Database connection
    ///
    /// # Returns
    ///
    /// Vector of concerts at this venue
    ///
    /// # Errors
    ///
    /// When there is a DB query error
    pub async fn concerts(&self, db: &DatabaseConnection) -> Result<Vec<concerts::Model>, DbErr> {
        concerts::Entity::find()
            .filter(concerts::Column::VenueId.eq(self.id))
            .all(db)
            .await
    }
}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {
    /// Find all venues with their related city information
    ///
    /// # Arguments
    ///
    /// * `db` - Database connection
    ///
    /// # Returns
    ///
    /// Vector of tuples containing (venue, city)
    pub async fn find_all_with_cities(
        db: &DatabaseConnection,
    ) -> Result<Vec<(Model, Option<cities::Model>)>, DbErr> {
        Self::find().find_also_related(cities::Entity).all(db).await
    }

    /// Find all venues with their related concerts
    ///
    /// # Arguments
    ///
    /// * `db` - Database connection
    ///
    /// # Returns
    ///
    /// Vector of tuples containing (venue, concert)
    pub async fn find_all_with_concerts(
        db: &DatabaseConnection,
    ) -> Result<Vec<(Model, Option<concerts::Model>)>, DbErr> {
        Self::find()
            .find_also_related(concerts::Entity)
            .all(db)
            .await
    }

    /// Find all venues with their related city and concerts
    ///
    /// # Arguments
    ///
    /// * `db` - Database connection
    ///
    /// # Returns
    ///
    /// Vector of tuples containing (venue, city, concerts)
    pub async fn find_all_with_cities_and_concerts(
        db: &DatabaseConnection,
    ) -> Result<Vec<(Model, Option<cities::Model>, Option<concerts::Model>)>, DbErr> {
        Self::find()
            .find_also_related(cities::Entity)
            .find_also_related(concerts::Entity)
            .all(db)
            .await
    }

    /// Find venues by city name
    ///
    /// # Arguments
    ///
    /// * `db` - Database connection
    /// * `city_name` - Name of the city to find venues for
    ///
    /// # Returns
    ///
    /// Vector of venues in the city
    pub async fn find_all_by_city_name(
        db: &DatabaseConnection,
        city_name: &str,
    ) -> Result<Vec<Model>, DbErr> {
        Self::find()
            .filter(
                venues::Column::CityId.in_subquery(
                    cities::Entity::find()
                        .filter(cities::Column::Name.eq(city_name))
                        .select_only()
                        .column(cities::Column::Id)
                        .into_query()
                        .to_owned(),
                ),
            )
            .all(db)
            .await
    }

    /// Find venues by city slug
    ///
    /// # Arguments
    ///
    /// * `db` - Database connection
    /// * `city_slug` - Slug of the city to find venues for
    ///
    /// # Returns
    ///
    /// Vector of venues in the city
    pub async fn find_all_by_city_slug(
        db: &DatabaseConnection,
        city_slug: &str,
    ) -> Result<Vec<Model>, DbErr> {
        Self::find()
            .filter(
                venues::Column::CityId.in_subquery(
                    cities::Entity::find()
                        .filter(cities::Column::Slug.eq(city_slug))
                        .select_only()
                        .column(cities::Column::Id)
                        .into_query()
                        .to_owned(),
                ),
            )
            .all(db)
            .await
    }

    /// Find venues that have concerts on a specific date
    ///
    /// # Arguments
    ///
    /// * `db` - Database connection
    /// * `date` - Date to find venues with concerts on
    ///
    /// # Returns
    ///
    /// Vector of venues with concerts on the specified date
    pub async fn find_all_with_concerts_on_date(
        db: &DatabaseConnection,
        date: chrono::NaiveDate,
    ) -> Result<Vec<Model>, DbErr> {
        Self::find()
            .filter(
                venues::Column::Id.in_subquery(
                    concerts::Entity::find()
                        .filter(concerts::Column::Date.eq(date))
                        .select_only()
                        .column(concerts::Column::VenueId)
                        .into_query()
                        .to_owned(),
                ),
            )
            .all(db)
            .await
    }

    /// Find venues that have concerts within a date range
    ///
    /// # Arguments
    ///
    /// * `db` - Database connection
    /// * `start_date` - Start of the date range
    /// * `end_date` - End of the date range
    ///
    /// # Returns
    ///
    /// Vector of venues with concerts within the date range
    pub async fn find_all_with_concerts_in_date_range(
        db: &DatabaseConnection,
        start_date: chrono::NaiveDate,
        end_date: chrono::NaiveDate,
    ) -> Result<Vec<Model>, DbErr> {
        Self::find()
            .filter(
                venues::Column::Id.in_subquery(
                    concerts::Entity::find()
                        .filter(concerts::Column::Date.between(start_date, end_date))
                        .select_only()
                        .column(concerts::Column::VenueId)
                        .into_query()
                        .to_owned(),
                ),
            )
            .all(db)
            .await
    }
}
