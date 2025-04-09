pub use super::_entities::concerts::{self, ActiveModel, Entity, Model};
use super::_entities::{performances, sets};
use loco_rs::prelude::*;
use sea_orm::entity::prelude::*;
pub type Concerts = Entity;

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
    /// Finds a concert by the provided slug
    ///
    /// # Errors
    ///
    /// When could not find concert by the given slug or DB query error
    pub async fn find_by_slug(db: &DatabaseConnection, slug: &str) -> ModelResult<Self> {
        let concert = Concerts::find()
            .filter(
                model::query::condition()
                    .eq(concerts::Column::Slug, slug)
                    .build(),
            )
            .one(db)
            .await?;
        concert.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// Find all concerts with their associated venue and artist information
    ///
    /// # Returns
    /// Vector of tuples containing (concert, venue, artist)
    pub async fn find_all_with_venue_and_artist(
        db: &DatabaseConnection,
    ) -> Result<
        Vec<(
            Model,
            Option<super::_entities::venues::Model>,
            Option<super::_entities::artists::Model>,
        )>,
        DbErr,
    > {
        Entity::find()
            .find_also_related(super::_entities::venues::Entity)
            .find_also_related(super::_entities::artists::Entity)
            .all(db)
            .await
    }

    pub async fn sets(&self, db: &DatabaseConnection) -> Result<Vec<sets::Model>, DbErr> {
        sets::Entity::find()
            .filter(sets::Column::ConcertId.eq(self.id))
            .all(db)
            .await
    }
}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {}
