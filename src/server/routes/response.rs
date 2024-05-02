use crate::server::routes::common::*;
use rocket::serde::Serialize;

// #[get("/namespaces/<namespace>/tables")] --> 200: ListTablesResponse
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ListTablesResponse {
  pub identifiers: Vec<TableIdentifier>,
}

// #[post("/namespaces/<namespace>/tables")] --> 200: CreateTableResponse
pub type CreateTableResponse = LoadTableResult;
// #[post("/namespaces/<namespace>/register")] --> 200: LoadTableResponse
// #[get("/namespaces/<namespace>/tables/<table>")] --> 200: LoadTableResponse
pub type LoadTableResponse = LoadTableResult;
// LoadTableResult
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct LoadTableResult {
  // pub metadata_location: Option<String>,
  pub metadata: TableMetadata,
  // pub config: HashMap<String, String>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CommitTableResponse {
  pub metadata_location: String,
  pub metadata: TableMetadata,
}

// 400: BadRequestErrorResponse
// 404: IcebergErrorResponse
// 409: TableAlreadyExistsError
// 503: ServiceUnavailableResponse
// 5XX: ServerErrorResponse
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct IcebergErrorResponse {
  pub error: ErrorModel,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ErrorModel {
  pub message: String,
  pub r#type: String, // Using r#type to avoid conflict with the type keyword
  pub code: i32,
  // pub stack: Option<Vec<String>>,
}
