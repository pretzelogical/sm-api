use serde::{Serialize, Deserialize};
use sea_orm::entity::prelude::*;


#[derive(Serialize, Deserialize, Debug, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "tag")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i64,
    pub post_id: i64,
    pub name: String
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
      belongs_to = "super::post::Entity",
      from = "Column::PostId",
      to = "super::post::Column::Id"
    )]
    Post
}

impl Related<super::post::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Post.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

