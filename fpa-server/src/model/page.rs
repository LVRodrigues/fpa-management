
use serde::Serialize;
use serde_derive::Deserialize;
use utoipa::{IntoParams, ToResponse, ToSchema};

/* Previous declaration of structure that will be fed by the model fields */
#[derive(Debug, Serialize)]
pub struct Project;

#[derive(Debug, Serialize)]
pub struct User;

#[derive(Debug, Serialize)]
pub struct Version;

/// Page selected.
#[derive(Debug, Clone, Serialize, ToSchema, ToResponse)]
#[aliases(
    Projects = Page<Project>,
    Users = Page<User>,
    Versions = Page<Version>,
)]
pub struct Page<T> {
    /// Total of pages.
    pub pages: u64,
    /// Index of this page.
    pub index: u64,
    /// Records in this page.
    pub size: u64,
    /// Total of records.
    pub records: u64,
    /// List of records.
    pub items: Vec<T>,
}

impl<T> Page<T> {
    pub fn new() -> Self {
        Self { pages: 0, index: 0, size: 0, records: 0, items: Vec::<T>::new() }
    }
}

/// Page select params.
#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct PageParams {
    /// Index of page to select.
    #[param(minimum = 1, default = 1)]
    page: Option<u64>,
    /// Page's size (records).
    #[param(minimum = 1, maximum = 50, default = 10)]
    size: Option<u64>,
    /// Filter by name.
    #[param()]
    name: Option<String>,
}

impl Default for PageParams {
    fn default() -> Self {
        Self { page: Some(1), size: Some(10), name: Some(String::new()) }
    }
}

impl PageParams {
    pub fn page(&self) -> u64 {
        match self.page {
            Some(v) => v,
            None => Self::default().page.unwrap(),
        }
    }

    pub fn size(&self) -> u64 {
        match self.size {
            Some(v) => v,
            None => Self::default().size.unwrap(),
        }
    }

    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }
}
