pub use super::_entities::songtitles::{self, ActiveModel, Entity, Model};
use loco_rs::prelude::*;
use sea_orm::entity::prelude::*;
pub type Songtitles = Entity;

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
    /// Finds a songtitle by the provided slug
    ///
    /// # Errors
    ///
    /// When could not find concert by the given slug or DB query error
    pub async fn find_by_slug(db: &DatabaseConnection, slug: &str) -> ModelResult<Self> {
        let title = Songtitles::find()
            .filter(
                model::query::condition()
                    .eq(songtitles::Column::Slug, slug)
                    .build(),
            )
            .one(db)
            .await?;
        title.ok_or_else(|| ModelError::EntityNotFound)
    }
}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {}
