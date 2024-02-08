use serde::Serialize;
use utoipa::{ToResponse, ToSchema};

/// Page selected.
#[derive(Debug, Clone, Serialize, ToSchema, ToResponse)]
#[aliases(Page = Pages<String>)]
pub struct Pages<T> {
    /// Total of pages.
    pub total: u16,
    /// Index of this page.
    pub index: u16,
    /// Records in this page.
    pub size: u16,
    /// Total of records.
    pub records: u64,
    /// List of records.
    pub items: Vec<T>,
}