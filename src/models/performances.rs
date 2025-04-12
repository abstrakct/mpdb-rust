pub use super::_entities::performances::{ActiveModel, Entity, Model};
use super::_entities::songtitles;
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

    pub async fn songtitle_as_str(&self, db: &DatabaseConnection) -> Result<Option<String>, DbErr> {
        let item = songtitles::Entity::find()
            .filter(songtitles::Column::SongId.eq(self.song_id))
            .filter(songtitles::Column::IsDefault.eq(true))
            .one(db)
            .await?;

        match item {
            Some(item) => Ok(Some(item.title)),
            None => Ok(None),
        }
        // title.ok_or_else(|| ModelError::EntityNotFound)
    }

    pub async fn performancetitle_as_str(
        &self,
        db: &DatabaseConnection,
    ) -> Result<Option<String>, DbErr> {
        let item = songtitles::Entity::find_by_id(self.songtitle_id)
            .one(db)
            .await?;

        match item {
            Some(item) => Ok(Some(item.title)),
            None => Ok(None),
        }
        // title.ok_or_else(|| ModelError::EntityNotFound)
    }

    pub async fn performancetitle_slug(
        &self,
        db: &DatabaseConnection,
    ) -> Result<Option<String>, DbErr> {
        let item = songtitles::Entity::find_by_id(self.songtitle_id)
            .one(db)
            .await?;

        match item {
            Some(item) => Ok(Some(item.slug)),
            None => Ok(None),
        }
        // title.ok_or_else(|| ModelError::EntityNotFound)
    }
}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {}
