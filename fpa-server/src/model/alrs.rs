//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.6

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "alrs")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub function: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub alr: Uuid,
    pub tenant: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::tenants::Entity",
        from = "Column::Tenant",
        to = "super::tenants::Column::Tenant",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Tenants,
}

impl Related<super::tenants::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tenants.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
