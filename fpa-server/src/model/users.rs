//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.6

use sea_orm::entity::prelude::*;
use serde_derive::Serialize;
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, ToSchema)]
#[sea_orm(table_name = "users")]
#[schema(as=User)]
#[serde(rename = "User")] 
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub user: Uuid,
    #[serde(skip)]
    pub tenant: Uuid,
    pub name: String,
    pub email: String,
    #[schema(value_type = String, format = DateTime)]
    pub time: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::projects::Entity")]
    Projects,
    #[sea_orm(
        belongs_to = "super::tenants::Entity",
        from = "Column::Tenant",
        to = "super::tenants::Column::Tenant",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Tenants,
}

impl Related<super::projects::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Projects.def()
    }
}

impl Related<super::tenants::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tenants.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
