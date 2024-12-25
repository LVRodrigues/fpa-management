use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};
use sea_orm::{
    ColumnTrait, Condition, DatabaseTransaction, EntityTrait, PaginatorTrait, QueryFilter,
    QueryTrait,
};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::model::{alrs, ders, functions_datas, prelude::*, rlrs};
use crate::{
    ctx::Context,
    error::{Error, ErrorResponse},
    model::{
        functions::{self, Model},
        modules,
        page::Page,
        sea_orm_active_enums::FunctionType,
    },
    state::AppState,
};

/// Data Element Reference
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DER {
    /// Unique Identifier of the DER.
    pub name: String,
    /// Description of the DER.
    pub description: Option<String>,
}

/// Record Layout Reference
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RLR {
    /// Unique Identifier of the RLR.
    pub name: String,
    /// Description of the RLR.
    pub description: Option<String>,
    /// Set of Data Element Reference.
    pub ders: Vec<DER>,
}

/// Internal Logic File Function
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FunctionALI {
    /// Unique Identifier of the Function.
    pub id: Uuid,
    /// Name of the Function.
    pub name: String,
    /// Description of the Function.
    pub description: Option<String>,
    /// Set of Record Layout Reference.
    pub rlrs: Vec<RLR>,
}

/// External Interface File Function
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FunctionAIE {
    /// Unique Identifier of the Function.
    pub id: Uuid,
    /// Name of the Function.
    pub name: String,
    /// Description of the Function.
    pub description: Option<String>,
    /// Set of Record Layout Reference.
    pub rlrs: Vec<RLR>,
}

/// Type of the Function of Data Type.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub enum FunctionData {
    /// ALI
    ALI(FunctionALI),
    /// AIE
    AIE(FunctionAIE),
}

/// External Input Function
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FunctionEE {
    /// Unique Identifier of the Function.
    pub id: Uuid,
    /// Name of the Function.
    pub name: String,
    /// Description of the Function.
    pub description: Option<String>,
    /// Set of Data Functions (ALI and AIE).
    pub alrs: Vec<FunctionData>,
}

/// External Inquiry Function
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FunctionCE {
    /// Unique Identifier of the Function.
    pub id: Uuid,
    /// Name of the Function.
    pub name: String,
    /// Description of the Function.
    pub description: Option<String>,
    /// Set of Data Functions (ALI and AIE).
    pub alrs: Vec<FunctionData>,
}

/// External Output Function
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FunctionSE {
    /// Unique Identifier of the Function.
    pub id: Uuid,
    /// Name of the Function.
    pub name: String,
    /// Description of the Function.
    pub description: Option<String>,
    /// Set of Data Functions (ALI and AIE).
    pub alrs: Vec<FunctionData>,
}

/// Type of the Function.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub enum Function {
    /// ALI
    ALI(FunctionALI),
    /// AIE
    AIE(FunctionAIE),
    /// EE
    EE(FunctionEE),
    /// CE
    CE(FunctionCE),
    /// SE
    SE(FunctionSE),
}

/// Page select params.
#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct FunctionsParams {
    /// Index of page to select.
    #[param(minimum = 1, default = 1)]
    page: Option<u64>,
    /// Page's size (records).
    #[param(minimum = 1, maximum = 50, default = 10)]
    size: Option<u64>,
    /// Filter by name.
    #[param()]
    name: Option<String>,
    /// Filter by Function Type.
    r#type: Option<FunctionType>,
}

impl Default for FunctionsParams {
    fn default() -> Self {
        Self {
            page: Some(1),
            size: Some(10),
            name: Some(String::new()),
            r#type: None,
        }
    }
}

impl FunctionsParams {
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

    pub fn r#type(&self) -> Option<FunctionType> {
        self.r#type.clone()
    }
}

#[utoipa::path(
    tag = "Functions",
    get,
    path = "/api/projects/{project}/modules/{module}/functions",
    responses(
        (status = OK, description = "Success", body = Page<Function>),
        (status = UNAUTHORIZED, description = "User not authorized.", body = ErrorResponse),
        (status = NOT_FOUND, description = "Project not founded.", body = ErrorResponse),
        (status = SERVICE_UNAVAILABLE, description = "FPA Management service unavailable.", body = ErrorResponse)
    ),
    params(
        ("project" = Uuid, description = "Project Unique ID."),
        ("module" = Uuid, Path, description = "Module Unique ID."),
        FunctionsParams,
    ),
    security(("fpa-security" = []))
)]
pub async fn list(
    Path((project, module)): Path<(Uuid, Uuid)>,
    params: Query<FunctionsParams>,
    context: Option<Context>,
    state: State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
    println!("==> {:<12} - /list (Params: {:?})", "FUNCTIONS", params);

    let mut conditions = Condition::all();
    conditions = conditions.add(modules::Column::Project.eq(project));
    conditions = conditions.add(functions::Column::Module.eq(module));
    if let Some(name) = params.name() {
        conditions = conditions.add(functions::Column::Name.contains(name));
    }
    if let Some(r#type) = params.r#type() {
        conditions = conditions.add(functions::Column::Type.eq(r#type));
    }

    let ctx = context.unwrap();
    let db = state.connection(ctx.tenant()).await?;
    let paginator = Functions::find()
        .inner_join(modules::Entity)
        .filter(conditions)
        .paginate(&db, params.size());

    let items = paginator.fetch_page(params.page() - 1).await?;
    let mut page = Page::<Function>::new();
    page.pages = paginator.num_pages().await?;
    page.index = params.page();
    page.size = items.len() as u64;
    page.records = paginator.num_items().await?;
    for item in items {
        page.items.push(translate(item, &db).await?);
    }

    Ok(Json(page))
}

async fn translate(func: Model, db: &DatabaseTransaction) -> Result<Function, Error> {
    let result = match func.r#type {
        FunctionType::ALI => Function::ALI(FunctionALI {
            id: func.function,
            name: func.name,
            description: func.description,
            rlrs: load_rlrs(func.function, &db).await?,
        }),
        FunctionType::AIE => Function::AIE(FunctionAIE {
            id: func.function,
            name: func.name,
            description: func.description,
            rlrs: load_rlrs(func.function, &db).await?,
        }),
        FunctionType::EE => Function::EE(FunctionEE {
            id: func.function,
            name: func.name,
            description: func.description,
            alrs: load_arls(func.function, &db).await?,
        }),
        FunctionType::CE => Function::CE(FunctionCE {
            id: func.function,
            name: func.name,
            description: func.description,
            alrs: load_arls(func.function, &db).await?,
        }),
        FunctionType::SE => Function::SE(FunctionSE {
            id: func.function,
            name: func.name,
            description: func.description,
            alrs: load_arls(func.function, &db).await?,
        }),
    };

    Ok(result)
}

async fn load_arls(function: Uuid, db: &DatabaseTransaction) -> Result<Vec<FunctionData>, Error> {
    let mut result = Vec::<FunctionData>::new();

    let alrs: Vec<functions_datas::Model> = FunctionsDatas::find()
        .inner_join(alrs::Entity)
        .filter(Condition::all().add(alrs::Column::Function.eq(function)))
        .all(db)
        .await?;

    for alr in alrs {
        let data = match alr.r#type {
            FunctionType::ALI => FunctionData::ALI(FunctionALI {
                id: alr.function,
                name: alr.name,
                description: alr.description,
                rlrs: load_rlrs(alr.function, &db).await?,
            }),
            FunctionType::AIE => FunctionData::AIE(FunctionAIE {
                id: alr.function,
                name: alr.name,
                description: alr.description,
                rlrs: load_rlrs(alr.function, &db).await?,
            }),
            _ => return Err(Error::NotFunctionData),
        };

        result.push(data);
    }

    Ok(result)
}

async fn load_rlrs(function: Uuid, db: &DatabaseTransaction) -> Result<Vec<RLR>, Error> {
    let mut result = Vec::<RLR>::new();

    let rlrs = Rlrs::find()
        .filter(Condition::all().add(rlrs::Column::Function.eq(function)))
        .all(db)
        .await?;

    for rlr in rlrs {
        let mut ders = Vec::<DER>::new();
        let der = Ders::find()
            .filter(
                Condition::all()
                    .add(ders::Column::Function.eq(rlr.function))
                    .add(ders::Column::Rlr.eq(rlr.name.clone())),
            )
            .all(db)
            .await?;
        for d in der {
            ders.push(DER {
                name: d.name,
                description: d.description,
            });
        }

        result.push(RLR {
            name: rlr.name,
            description: rlr.description,
            ders,
        });
    }

    Ok(result)
}
