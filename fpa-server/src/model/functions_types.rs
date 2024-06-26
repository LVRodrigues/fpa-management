//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.6

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "functions_types")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub r#type: i32,
    pub description: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::functions::Entity")]
    Functions,
}

impl Related<super::functions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Functions.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
