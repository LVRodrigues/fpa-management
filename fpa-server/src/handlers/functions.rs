use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    response::IntoResponse, Json,
};
use sea_orm::{ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    ctx::Context,
    error::{Error, ErrorResponse},
    model::{functions::{self, ActiveModel, Model}, modules, page::{Page, PageParams}, sea_orm_active_enums::FunctionType},
    state::AppState,
};

/// Internal Logic File Function
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FunctionALI {
    /// Unique Identifier of the Function.
    pub id: Uuid,
    /// Name of the Function.
    pub name: String,
    /// Description of the Function.
    pub description: Option<String>,
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
}

/// Type of the Function.
#[derive(Debug, Serialize, ToSchema)]
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
        PageParams,
        ("ftype" = Option<FunctionType>, Query, description = "Type of the Function."),
    ),
    security(("fpa-security" = []))
)]
pub async fn list(
    Path((project, module)): Path<(Uuid, Uuid)>,
    Query((params, ftype)): Query<(PageParams, Option<FunctionType>)>,
    context: Option<Context>,
    state: State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
    println!("==> {:<12} - /list (Params: {:?} - FunctionType: {:?})", "FUNCTIONS", params, ftype);

    let mut conditions = Condition::all();
    conditions = conditions.add(modules::Column::Project.eq(project));
    conditions = conditions.add(functions::Column::Module.eq(module));
    if let Some(name) = params.name() {
        conditions = conditions.add(functions::Column::Name.contains(name));
    }
    if let Some(ftype) = ftype {
        conditions = conditions.add(functions::Column::Type.eq(ftype));
    }

    let ctx = context.unwrap();
    let db = state.connection(ctx.tenant()).await?;
    let paginator = functions::Entity::find()
        .filter(conditions)
        .paginate(&db, params.size());

    let records = paginator.fetch_page(params.page()).await?;
    let mut page: Page<Function> = Page::new();
    page.pages = paginator.num_pages().await?;
    page.index = params.page();
    page.size = records.len() as u64;
    page.records = paginator.num_items().await?;
    for record in records {
        page.items.push(translate(record));
    }
    
    Ok(Json(page))
}

// TODO - Add rlrs and ders do functions ALI and AIE
// TODO - Add alrs to functions EE, CE and SE
fn translate(func: Model) -> Function {
    match func.r#type {
        FunctionType::ALI => Function::ALI(FunctionALI {
            id: func.function,
            name: func.name,
            description: func.description,
        }),
        FunctionType::AIE => Function::AIE(FunctionAIE {
            id: func.function,
            name: func.name,
            description: func.description,
        }),
        FunctionType::EE => Function::EE(FunctionEE {
            id: func.function,
            name: func.name,
            description: func.description,
        }),
        FunctionType::CE => Function::CE(FunctionCE {
            id: func.function,
            name: func.name,
            description: func.description,
        }),
        FunctionType::SE => Function::SE(FunctionSE {
            id: func.function,
            name: func.name,
            description: func.description,
        }),
    }
}
