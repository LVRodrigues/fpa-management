use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, Uri},
    response::IntoResponse,
    Json,
};
use reqwest::StatusCode;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseTransaction, EntityTrait, PaginatorTrait,
    QueryFilter, Set,
};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::model::{alrs, ders, functions_datas, functions_transactions, prelude::*, rlrs};
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
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct DER {
    /// Unique Identifier of the DER.
    pub name: String,
    /// Description of the DER.
    pub description: Option<String>,
}

/// Record Layout Reference
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct RLR {
    /// Unique Identifier of the RLR.
    pub name: String,
    /// Description of the RLR.
    pub description: Option<String>,
    /// Set of Data Element Reference.
    pub ders: Vec<DER>,
}

/// Internal Logic File Function
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
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

/// Internal Logic File Function for create or update data.
#[derive(Debug, Deserialize, ToSchema, Clone)]
pub struct FunctionALIParam {
    /// Name of the Function.
    pub name: String,
    /// Description of the Function.
    pub description: Option<String>,
    /// Set of Record Layout Reference.
    pub rlrs: Vec<RLR>,
}

/// External Interface File Function
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
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

/// External Interface File Function for create or update data.
#[derive(Debug, Deserialize, ToSchema, Clone)]
pub struct FunctionAIEParam {
    /// Name of the Function.
    pub name: String,
    /// Description of the Function.
    pub description: Option<String>,
    /// Set of Record Layout Reference.
    pub rlrs: Vec<RLR>,
}

/// Type of the Function of Data Type.
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub enum FunctionData {
    /// Internal Logical File.
    ALI(FunctionALI),
    /// External Interface File.
    AIE(FunctionAIE),
}

/// Data Function for association with the Transaction Function.
#[derive(Debug, Deserialize, ToSchema, Clone)]
pub struct ALR {
    /// Unique Identifier of the Data Function.
    pub id: Uuid,
}

/// External Input Function.
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

/// External Input Function for create or update data.
#[derive(Debug, Deserialize, ToSchema)]
pub struct FunctionEEParam {
    /// Name of the Function.
    pub name: String,
    /// Description of the Function.
    pub description: Option<String>,
    /// Set of Data Functions (ALI and AIE).
    pub alrs: Vec<ALR>,
}

/// External Inquiry Function.
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

/// External Inquiry Function for create or update data.
#[derive(Debug, Deserialize, ToSchema)]
pub struct FunctionCEParam {
    /// Name of the Function.
    pub name: String,
    /// Description of the Function.
    pub description: Option<String>,
    /// Set of Data Functions (ALI and AIE).
    pub alrs: Vec<ALR>,
}

/// External Output Function.
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

/// External Output Function for create or update data.
#[derive(Debug, Deserialize, ToSchema)]
pub struct FunctionSEParam {
    /// Name of the Function.
    pub name: String,
    /// Description of the Function.
    pub description: Option<String>,
    /// Set of Data Functions (ALI and AIE).
    pub alrs: Vec<ALR>,
}

/// Type of the Function.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub enum Function {
    /// Internal Logic File Function.
    ALI(FunctionALI),
    /// External Interface File Function.
    AIE(FunctionAIE),
    /// External Input Function.
    EE(FunctionEE),
    /// External Inquiry Function.
    CE(FunctionCE),
    /// External Output Function.
    SE(FunctionSE),
}

/// Type of the Function for create or update data.
#[derive(Debug, Deserialize, ToSchema)]
pub enum FunctionParam {
    /// Internal Logic File Function.
    ALI(FunctionALIParam),
    /// External Interface File Function.
    AIE(FunctionAIEParam),
    /// External Input Function.
    EE(FunctionEEParam),
    /// External Inquiry Function.
    CE(FunctionCEParam),
    /// External Output Function.
    SE(FunctionSEParam),
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

/// Search for a set of Functions for a selected Project and Module.
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

/// Search for a especific Function for a selected Project and Module.
#[utoipa::path(
    tag = "Functions",
    get,
    path = "/api/projects/{project}/modules/{module}/functions/{function}",
    responses(
        (status = OK, description = "Success", body = Function),
        (status = UNAUTHORIZED, description = "User not authorized.", body = ErrorResponse),
        (status = NOT_FOUND, description = "Project not founded.", body = ErrorResponse),
        (status = SERVICE_UNAVAILABLE, description = "FPA Management service unavailable.", body = ErrorResponse)
    ),
    params(
        ("project" = Uuid, description = "Project Unique ID."),
        ("module" = Uuid, Path, description = "Module Unique ID."),
        ("function" = Uuid, Path, description = "Function Unique ID."),
    ),
    security(("fpa-security" = []))
)]
pub async fn by_id(
    Path((project, module, function)): Path<(Uuid, Uuid, Uuid)>,
    context: Option<Context>,
    state: State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
    println!("==> {:<12} - /by_id", "FUNCTIONS");

    let mut conditions = Condition::all();
    conditions = conditions.add(modules::Column::Project.eq(project));
    conditions = conditions.add(functions::Column::Module.eq(module));
    conditions = conditions.add(functions::Column::Function.eq(function));

    let ctx = context.unwrap();
    let db = state.connection(ctx.tenant()).await?;
    let data = Functions::find()
        .inner_join(modules::Entity)
        .filter(conditions)
        .one(&db)
        .await?;

    match data {
        Some(v) => Ok((StatusCode::OK, Json(translate(v, &db).await?))),
        None => return Err(Error::NotFound),
    }
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

/// Create a new Function for a selected Project and Module.
#[utoipa::path(
    tag = "Functions",
    post,
    path = "/api/projects/{project}/modules/{module}/functions",
    responses(
        (status = CREATED, description = "Success.", body = Function, headers(("Location", description = "New function address."))),
        (status = UNAUTHORIZED, description = "User not authorized.", body = ErrorResponse),
        (status = NOT_FOUND, description = "Project or Module not founded.", body = ErrorResponse),
        (status = NOT_ACCEPTABLE, description = "Function Type incorrect.", body = ErrorResponse),
        (status = SERVICE_UNAVAILABLE, description = "FPA Management service unavailable.", body = ErrorResponse)
    ),
    params(
        ("project" = Uuid, Path, description = "Project Unique ID."),
        ("module" = Uuid, Path, description = "Module Unique ID."),
    ),
    security(("fpa-security" = []))
)]
pub async fn create(
    Path((project, module)): Path<(Uuid, Uuid)>,
    context: Option<Context>,
    state: State<Arc<AppState>>,
    Json(data): Json<FunctionParam>,
) -> Result<impl IntoResponse, Error> {
    println!("==> {:<12} - /create (Data: {:?})", "FUNCTIONS", data);

    let ctx = context.unwrap();
    let db = state.connection(ctx.tenant()).await?;
    let config = state.configuration();

    // Module must belong to the Project.
    match Modules::find()
        .filter(
            Condition::all()
                .add(modules::Column::Project.eq(project))
                .add(modules::Column::Module.eq(module)),
        )
        .one(&db)
        .await
    {
        Ok(_) => (),
        Err(_) => return Err(Error::NotFound),
    };

    let (id, function) = match data {
        FunctionParam::ALI(_) | FunctionParam::AIE(_) => {
            insert_function_data(data, module, &db, &ctx).await?
        }
        FunctionParam::EE(_) | FunctionParam::CE(_) | FunctionParam::SE(_) => {
            insert_function_transaction(data, module, &db, &ctx).await?
        }
    };

    match db.commit().await {
        Ok(it) => it,
        Err(_) => return Err(Error::DatabaseTransaction),
    };

    let location = Uri::builder()
        .scheme(config.scheme.clone())
        .authority(format!(
            "{}:{}",
            config.authority.clone(),
            config.port.clone()
        ))
        .path_and_query(format!(
            "/api/projects/{}/modules/{}/functions/{}",
            project, module, id
        ))
        .build()
        .unwrap()
        .to_string()
        .parse()
        .unwrap();
    let mut header = HeaderMap::new();
    header.insert("Location", location);

    Ok((StatusCode::CREATED, header, Json(function)))
}

async fn insert_function_transaction(
    data: FunctionParam,
    module: Uuid,
    db: &DatabaseTransaction,
    ctx: &Context,
) -> Result<(Uuid, Function), Error> {
    let mut function = functions_transactions::ActiveModel {
        function: Set(Uuid::now_v7()),
        module: Set(module),
        tenant: Set(ctx.tenant().clone()),
        ..Default::default()
    };

    let mut alrs = Vec::<ALR>::new();
    match data {
        FunctionParam::EE(data) => {
            function.r#type = Set(FunctionType::EE);
            function.name = Set(data.name.to_owned());
            function.description = Set(data.description.to_owned());
            alrs = data.alrs;
        }
        FunctionParam::CE(data) => {
            function.r#type = Set(FunctionType::CE);
            function.name = Set(data.name.to_owned());
            function.description = Set(data.description.to_owned());
            alrs = data.alrs;
        }
        FunctionParam::SE(data) => {
            function.r#type = Set(FunctionType::SE);
            function.name = Set(data.name.to_owned());
            function.description = Set(data.description.to_owned());
            alrs = data.alrs;
        }
        _ => return Err(Error::NotFunctionTransaction),
    };
    let function = function.insert(db).await?;

    for alr in alrs {
        let item = alrs::ActiveModel {
            function: Set(function.function),
            tenant: Set(ctx.tenant().clone()),
            alr: Set(alr.id),
        };
        item.insert(db).await?;
    }

    let result = match function.r#type {
        FunctionType::EE => Function::EE(FunctionEE {
            id: function.function,
            name: function.name,
            description: function.description,
            alrs: load_arls(function.function, &db).await?,
        }),
        FunctionType::CE => Function::CE(FunctionCE {
            id: function.function,
            name: function.name,
            description: function.description,
            alrs: load_arls(function.function, &db).await?,
        }),
        FunctionType::SE => Function::SE(FunctionSE {
            id: function.function,
            name: function.name,
            description: function.description,
            alrs: load_arls(function.function, &db).await?,
        }),
        _ => return Err(Error::FunctionCreate),
    };

    Ok((function.function, result))
}

async fn insert_function_data(
    data: FunctionParam,
    module: Uuid,
    db: &DatabaseTransaction,
    ctx: &Context,
) -> Result<(Uuid, Function), Error> {
    let mut function = functions_datas::ActiveModel {
        function: Set(Uuid::now_v7()),
        module: Set(module),
        tenant: Set(ctx.tenant().clone()),
        ..Default::default()
    };

    let mut rlrs = Vec::<RLR>::new();
    match data {
        FunctionParam::ALI(data) => {
            function.r#type = Set(FunctionType::ALI);
            function.name = Set(data.name.to_owned());
            function.description = Set(data.description.to_owned());
            rlrs = data.rlrs;
        }
        FunctionParam::AIE(data) => {
            function.r#type = Set(FunctionType::AIE);
            function.name = Set(data.name.to_owned());
            function.description = Set(data.description.to_owned());
            rlrs = data.rlrs;
        }
        _ => return Err(Error::NotFunctionData),
    };
    let function = function.insert(db).await?;

    for rlr in rlrs.to_vec() {
        let item = rlrs::ActiveModel {
            function: Set(function.function),
            name: Set(rlr.name.clone()),
            tenant: Set(ctx.tenant().clone()),
            description: Set(rlr.description),
        };
        item.insert(db).await?;

        for der in rlr.ders {
            let item = ders::ActiveModel {
                function: Set(function.function),
                rlr: Set(rlr.name.clone()),
                name: Set(der.name),
                tenant: Set(ctx.tenant().clone()),
                description: Set(der.description),
            };
            item.insert(db).await?;
        }
    }

    let result = match function.r#type {
        FunctionType::ALI => Function::ALI(FunctionALI {
            id: function.function,
            name: function.name,
            description: function.description,
            rlrs: rlrs,
        }),
        FunctionType::AIE => Function::AIE(FunctionAIE {
            id: function.function,
            name: function.name,
            description: function.description,
            rlrs: rlrs,
        }),
        _ => return Err(Error::FunctionCreate),
    };

    Ok((function.function, result))
}

/// Update a existing Function.
#[utoipa::path(
    tag = "Functions",
    put,
    path = "/api/projects/{project}/modules/{module}/functions/{function}",
    responses(
        (status = OK, description = "Success.", body = Function),
        (status = UNAUTHORIZED, description = "User not authorized.", body = ErrorResponse),
        (status = NOT_FOUND, description = "Project or Module not founded.", body = ErrorResponse),
        (status = NOT_ACCEPTABLE, description = "The Function Type cannot be updated.", body = ErrorResponse),
        (status = CONFLICT, description = "The name must be unique for this scope.", body = ErrorResponse),
        (status = SERVICE_UNAVAILABLE, description = "FPA Management service unavailable.", body = ErrorResponse)
    ),
    params(
        ("project" = Uuid, Path, description = "Project Unique ID."),
        ("module" = Uuid, Path, description = "Module Unique ID."),
        ("function" = Uuid, Path, description = "Functions Unique ID."),
    ),
    security(("fpa-security" = []))
)]
pub async fn update(
    Path((project, module, function)): Path<(Uuid, Uuid, Uuid)>,
    context: Option<Context>,
    state: State<Arc<AppState>>,
    Json(params): Json<FunctionParam>,
) -> Result<impl IntoResponse, Error> {
    println!(
        "==> {:<12} - /{project}/update/{module}/functions/{function} {:?}",
        "MODULES", params
    );

    let mut conditions = Condition::all();
    conditions = conditions.add(modules::Column::Project.eq(project));
    conditions = conditions.add(functions::Column::Module.eq(module));
    conditions = conditions.add(functions::Column::Function.eq(function));

    let ctx = context.unwrap();
    let db = state.connection(ctx.tenant()).await?;
    let data = match Functions::find()
        .inner_join(modules::Entity)
        .filter(conditions)
        .one(&db)
        .await?
    {
        Some(v) => v,
        None => return Err(Error::NotFound),
    };

    let data = match params {
        FunctionParam::ALI(value) => {
            if data.r#type != FunctionType::ALI {
                return Err(Error::FunctionTypeUpdateError);
            }
            update_function_data(function, value.name, value.description, value.rlrs, &db).await?
        }
        FunctionParam::AIE(value) => {
            if data.r#type != FunctionType::AIE {
                return Err(Error::FunctionTypeUpdateError);
            }
            update_function_data(function, value.name, value.description, value.rlrs, &db).await?
        }
        FunctionParam::EE(value) => {
            if data.r#type != FunctionType::EE {
                return Err(Error::FunctionTypeUpdateError);
            }
            update_function_transaction(function, value.name, value.description, value.alrs, &db).await?
        }
        FunctionParam::CE(value) => {
            if data.r#type != FunctionType::CE {
                return Err(Error::FunctionTypeUpdateError);
            }
            update_function_transaction(function, value.name, value.description, value.alrs, &db).await?
        }
        FunctionParam::SE(value) => {
            if data.r#type != FunctionType::SE {
                return Err(Error::FunctionTypeUpdateError);
            }
            update_function_transaction(function, value.name, value.description, value.alrs, &db).await?
        }
    };

    match db.commit().await {
        Ok(it) => it,
        Err(_) => return Err(Error::DatabaseTransaction),
    };

    Ok(Json(data))
}

async fn update_function_data(
    function: Uuid,
    name: String,
    description: Option<String>,
    rlrs: Vec<RLR>,
    db: &DatabaseTransaction,
) -> Result<Function, Error> {

    let data = FunctionsDatas::find()
        .filter(functions_datas::Column::Function.eq(function))
        .one(db)
        .await?
        .unwrap();

    let mut data: functions_datas::ActiveModel = data.into();
    data.name = Set(name);
    data.description = Set(description);

    let data: functions_datas::Model = match data.update(db).await {
        Ok(v) => v,
        Err(e) => {
            match e.sql_err().unwrap() {
                sea_orm::SqlErr::UniqueConstraintViolation(_) => {
                    return Err(Error::FunctionNameDuplicated)
                }
                _ => return Err(Error::FunctionUpdate),
            };
        }
    };

    delete_related_rlrs(function, db).await?;

    for rlr in rlrs {
        let item = rlrs::ActiveModel {
            function: Set(function),
            name: Set(rlr.name.clone()),
            tenant: Set(data.tenant.clone()),
            description: Set(rlr.description),
        };
        item.insert(db).await?;

        for der in rlr.ders {
            let item = ders::ActiveModel {
                function: Set(function),
                rlr: Set(rlr.name.clone()),
                name: Set(der.name),
                tenant: Set(data.tenant.clone()),
                description: Set(der.description),
            };
            item.insert(db).await?;
        }
    }

    let data = translate(data.into(), db).await?;
    Ok(data)
}

async fn delete_related_rlrs(function: Uuid, db: &DatabaseTransaction) -> Result<(), Error> {
    Rlrs::delete_many()
        .filter(rlrs::Column::Function.eq(function))
        .exec(db)
        .await?;
    Ok(())
}

async fn update_function_transaction(
    function: Uuid,
    name: String,
    description: Option<String>,
    alrs: Vec<ALR>,
    db: &DatabaseTransaction,
) -> Result<Function, Error> {
    
    let data: functions_transactions::Model = FunctionsTransactions::find()
        .filter(functions_transactions::Column::Function.eq(function))
        .one(db)
        .await?
        .unwrap();

    let mut data: functions_transactions::ActiveModel = data.into();
    data.name = Set(name);
    data.description = Set(description);

    let data: functions_transactions::Model = match data.update(db).await {
        Ok(v) => v,
        Err(e) => {
            match e.sql_err().unwrap() {
                sea_orm::SqlErr::UniqueConstraintViolation(_) => {
                    return Err(Error::FunctionNameDuplicated)
                }
                _ => return Err(Error::FunctionUpdate),
            };
        }
    };

    delete_related_alrs(function, db).await?;

    for alr in alrs {
        let item = alrs::ActiveModel {
            function: Set(function),
            tenant: Set(data.tenant.clone()),
            alr: Set(alr.id),
        };
        item.insert(db).await?;
    }

    let data = translate(data.into(), db).await?;
    Ok(data)

}

async fn delete_related_alrs(function: Uuid, db: &DatabaseTransaction) -> Result<(), Error> {
    Alrs::delete_many()
        .filter(rlrs::Column::Function.eq(function))
        .exec(db)
        .await?;
    Ok(())
}