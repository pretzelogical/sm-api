use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "post")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i64,
    pub author_id: i64,
    pub title: String,
    pub content: String,
    pub img: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::AuthorId",
        to = "super::user::Column::Id"
    )]
    User,
    #[sea_orm(has_many = "super::comment::Entity")]
    Comment,
    #[sea_orm(has_many = "super::tag::Entity")]
    Tag,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::comment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Comment.def()
    }
}

impl Related<super::tag::Entity> for Entity {
    fn to() -> RelationDef {
        super::post_tag::Relation::Tag.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::post_tag::Relation::Post.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
