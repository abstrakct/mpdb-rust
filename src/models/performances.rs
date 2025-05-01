pub use super::_entities::performances::{self, ActiveModel, Entity, Model};
use super::_entities::songtitles;
use loco_rs::prelude::*;
use sea_orm::entity::prelude::*;
pub type Performances = Entity;

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
    // pub async fn songtitle(&self, db: &DatabaseConnection) -> ModelResult<songtitles::Model> {
    //     let title = songtitles::Entity::find_by_id(self.songtitle_id)
    //         .one(db)
    //         .await?;
    //     title.ok_or_else(|| ModelError::EntityNotFound)
    // }

    pub async fn songtitle_as_str(&self, db: &DatabaseConnection) -> ModelResult<Option<String>> {
        let item = songtitles::Entity::find()
            .filter(songtitles::Column::SongId.eq(self.song_id))
            .filter(songtitles::Column::IsDefault.eq(true))
            .one(db)
            .await?;

        match item {
            Some(item) => Ok(Some(item.title)),
            None => Ok(None),
        }
    }

    pub async fn performancetitle_as_str(
        &self,
        db: &DatabaseConnection,
    ) -> ModelResult<Option<String>> {
        let item = songtitles::Entity::find_by_id(self.songtitle_id)
            .one(db)
            .await?;

        match item {
            Some(item) => Ok(Some(item.title)),
            None => Ok(None),
        }
    }

    pub async fn performancetitle_slug(
        &self,
        db: &DatabaseConnection,
    ) -> ModelResult<Option<String>> {
        let item = songtitles::Entity::find_by_id(self.songtitle_id)
            .one(db)
            .await?;

        match item {
            Some(item) => Ok(Some(item.slug)),
            None => Ok(None),
        }
    }

    /// Finds the previous performance relative to this one.
    /// Returns None if this is the first performance in the set.
    ///
    /// # Errors
    ///
    /// If DB query fails.
    pub async fn prev(&self, db: &DatabaseConnection) -> ModelResult<Option<Self>> {
        if self.sort_order == 0 {
            return Ok(None);
        }

        let item = Entity::find()
            .filter(performances::Column::SetId.eq(self.set_id))
            .filter(performances::Column::SortOrder.eq(self.sort_order - 1))
            .one(db)
            .await?;

        Ok(item)
    }

    /// Finds the next performance relative to this one.
    /// Returns None if this is the last performance in the set.
    ///
    /// # Errors
    ///
    /// If DB query fails.
    pub async fn next(&self, db: &DatabaseConnection) -> ModelResult<Option<Self>> {
        let item = Entity::find()
            .filter(performances::Column::SetId.eq(self.set_id))
            .filter(performances::Column::SortOrder.eq(self.sort_order + 1))
            .one(db)
            .await?;

        Ok(item)
    }
}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {}
