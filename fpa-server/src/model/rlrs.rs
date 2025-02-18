//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "rlrs")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub function: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub name: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    pub tenant: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::ders::Entity")]
    Ders,
    #[sea_orm(
        belongs_to = "super::tenants::Entity",
        from = "Column::Tenant",
        to = "super::tenants::Column::Tenant",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Tenants,
    #[sea_orm(
        belongs_to = "super::functions_datas::Entity",
        from = "Column::Function",
        to = "super::functions_datas::Column::Function",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    FunctionsDatas,
}

impl Related<super::ders::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Ders.def()
    }
}

impl Related<super::tenants::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tenants.def()
    }
}

impl Related<super::functions_datas::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FunctionsDatas.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
