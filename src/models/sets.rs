pub use super::_entities::sets::{ActiveModel, Entity, Model};
use sea_orm::entity::prelude::*;
pub type Sets = Entity;

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
    pub async fn performances(
        &self,
        db: &DatabaseConnection,
    ) -> Result<Vec<super::performances::Model>, DbErr> {
        self.find_related(super::performances::Entity).all(db).await
    }

    pub async fn songs(&self, db: &DatabaseConnection) -> Result<Vec<super::songs::Model>, DbErr> {
        let performances = self.performances(db).await?;
        let mut songs = Vec::new();
        for performance in performances {
            if let Some(song) = performance
                .find_related(super::songs::Entity)
                .one(db)
                .await?
            {
                songs.push(song);
            }
        }
        Ok(songs)
    }
}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {}
