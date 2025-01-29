use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "post_tag")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i64,
    pub post_id: i64,
    pub tag_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::post::Entity",
        from = "Column::PostId",
        to = "super::post::Column::Id"
    )]
    Post,
    #[sea_orm(
        belongs_to = "super::tag::Entity",
        from = "Column::TagId",
        to = "super::tag::Column::Id"
    )]
    Tag,
}

// impl Related<super::user::Entity> for Entity {
//     fn to() -> RelationDef {
//         Relation::User.def()
//     }
// }
//
// impl Related<super::comment::Entity> for Entity {
//     fn to() -> RelationDef {
//         Relation::Comment.def()
//     }
// }
//
// impl Related<super::tag::Entity> for Entity {
//     fn to() -> RelationDef {
//         Relation::Tag.def()
//     }
// }
//
impl ActiveModelBehavior for ActiveModel {}
