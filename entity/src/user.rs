use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing, skip_serializing)]
    pub id: i64,
    pub name: String,
    pub pass: String,
    pub handle: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::post::Entity")]
    Post,
    #[sea_orm(has_many = "super::comment::Entity")]
    Comment,
    #[sea_orm(has_many = "super::like::Entity")]
    Like,
}

impl Related<super::post::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Post.def()
    }
}

impl Related<super::comment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Comment.def()
    }
}

impl Related<super::like::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Like.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
