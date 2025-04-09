pub use super::_entities::songs::{ActiveModel, Entity, Model};
use super::_entities::songtitles;
use sea_orm::entity::prelude::*;
pub type Songs = Entity;

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
    pub async fn titles(
        &self,
        db: &DatabaseConnection,
    ) -> Result<Vec<super::songtitles::Model>, DbErr> {
        self.find_related(super::songtitles::Entity).all(db).await
    }

    pub async fn title(&self, db: &DatabaseConnection) -> Result<Option<String>, DbErr> {
        let item = self
            .find_related(super::songtitles::Entity)
            .filter(songtitles::Column::IsDefault.eq(true))
            .one(db)
            .await?;

        match item {
            Some(item) => Ok(Some(item.title)),
            None => Ok(None),
        }
    }
}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {}
