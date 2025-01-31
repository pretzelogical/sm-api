use crate::post_tag::Relation;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "tag")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing, skip_serializing)]
    pub id: i64,
    pub name: String,
}

impl Related<super::post::Entity> for Entity {
    fn to() -> RelationDef {
        super::post_tag::Relation::Post.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::post_tag::Relation::Tag.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
