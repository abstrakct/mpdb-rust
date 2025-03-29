pub use super::_entities::cities::{self, ActiveModel, Entity, Model};
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
}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {}
