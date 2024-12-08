//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.2


use super::sea_orm_active_enums::FunctionType;
use axum::async_trait;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "functions_transactions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub function: Uuid,
    pub module: Uuid,
    pub tenant: Uuid,
    pub r#type: FunctionType,
    pub name: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::alrs::Entity")]
    Alrs,
    #[sea_orm(
        belongs_to = "super::modules::Entity",
        from = "Column::Module",
        to = "super::modules::Column::Module",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Modules,
    #[sea_orm(
        belongs_to = "super::tenants::Entity",
        from = "Column::Tenant",
        to = "super::tenants::Column::Tenant",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Tenants,
}

impl Related<super::rlrs::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Alrs.def()
    }
}

impl Related<super::modules::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Modules.def()
    }
}

impl Related<super::tenants::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tenants.def()
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, _insert: bool) -> Result<Self, DbErr> where C: ConnectionTrait {
        let test = self.r#type.as_ref();
        match test {
            FunctionType::ALI | FunctionType::AIE => Err(DbErr::Custom(String::from("Invalid Type for FunctionTransaction"))),
            _ => Ok(self)
        }
    }
}
