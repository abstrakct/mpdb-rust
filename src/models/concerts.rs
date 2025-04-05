pub use super::_entities::concerts::{ActiveModel, Entity, Model};
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
}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {}
