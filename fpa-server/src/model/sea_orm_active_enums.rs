//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Empirical adjustment for the Project.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    EnumIter,
    DeriveActiveEnum,
    utoipa::ToSchema,
    Serialize,
    Deserialize,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "empirical_type")]
pub enum EmpiricalType {
    #[sea_orm(string_value = "COORDINATION")]
    Coordination,
    #[sea_orm(string_value = "DEPLOYMENT")]
    Deployment,
    #[sea_orm(string_value = "PLANNING")]
    Planning,
    #[sea_orm(string_value = "PRODUCTIVITY")]
    Productivity,
    #[sea_orm(string_value = "TESTING")]
    Testing,
}

/// Adjustment factor for the Project.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    EnumIter,
    DeriveActiveEnum,
    utoipa::ToSchema,
    Serialize,
    Deserialize,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "factor_type")]
pub enum FactorType {
    #[sea_orm(string_value = "COMPLEX_PROCESSING")]
    ComplexProcessing,
    #[sea_orm(string_value = "DATA_COMMUNICATIONS")]
    DataCommunications,
    #[sea_orm(string_value = "DISTRIBUTED_DATA_PROCESSING")]
    DistributedDataProcessing,
    #[sea_orm(string_value = "END_USER_EFFICIENCY")]
    EndUserEfficiency,
    #[sea_orm(string_value = "FACILITTE_CHANGE")]
    FacilitteChange,
    #[sea_orm(string_value = "HEAVILY_USED_CONFIGURATION")]
    HeavilyUsedConfiguration,
    #[sea_orm(string_value = "INSTALLATION_EASE")]
    InstallationEase,
    #[sea_orm(string_value = "MULTIPLE_SITES")]
    MultipleSites,
    #[sea_orm(string_value = "ONLINE_DATA_ENTRY")]
    OnlineDataEntry,
    #[sea_orm(string_value = "ONLINE_UPDATE")]
    OnlineUpdate,
    #[sea_orm(string_value = "OPERATIONAL_EASE")]
    OperationalEase,
    #[sea_orm(string_value = "PERFORMANCE")]
    Performance,
    #[sea_orm(string_value = "REUSABILITY")]
    Reusability,
    #[sea_orm(string_value = "TRANSACTION_RATE")]
    TransactionRate,
}

/// Influence value for the adjustment factor.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    EnumIter,
    DeriveActiveEnum,
    utoipa::ToSchema,
    Serialize,
    Deserialize,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "influence_type")]
pub enum InfluenceType {
    #[sea_orm(string_value = "ABSENT")]
    Absent,
    #[sea_orm(string_value = "AVERAGE")]
    Average,
    #[sea_orm(string_value = "MINIMUM")]
    Minimum,
    #[sea_orm(string_value = "MODERATE")]
    Moderate,
    #[sea_orm(string_value = "SIGNIFICANT")]
    Significant,
    #[sea_orm(string_value = "STRONG")]
    Strong,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "tenant_status")]
pub enum TenantStatus {
    #[sea_orm(string_value = "ACTIVE")]
    Active,
    #[sea_orm(string_value = "DISABLED")]
    Disabled,
    #[sea_orm(string_value = "SUSPENDED")]
    Suspended,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "tenant_tier")]
pub enum TenantTier {
    #[sea_orm(string_value = "BRONZE")]
    Bronze,
    #[sea_orm(string_value = "GOLD")]
    Gold,
    #[sea_orm(string_value = "SILVER")]
    Silver,
}

/// Type of Function.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    EnumIter,
    DeriveActiveEnum,
    utoipa::ToSchema,
    Serialize,
    Deserialize,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "function_type")]
pub enum FunctionType {
    /// Internal Logic File Function
    #[sea_orm(string_value = "ALI")]
    ALI,
    /// External Interface File Function
    #[sea_orm(string_value = "AIE")]
    AIE,
    /// External Inquiry Function
    #[sea_orm(string_value = "CE")]
    CE,
    /// External Input Function
    #[sea_orm(string_value = "EE")]
    EE,
    /// External Output Function
    #[sea_orm(string_value = "SE")]
    SE,
}
