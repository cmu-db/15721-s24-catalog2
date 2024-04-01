use rocket::http::Status;
use rocket::response::{content, status};
use rocket::serde::json::Json;
use crate::response::*;

use crate::common::result::{self, EmptyResult, ErrorType, JsonResult, Location, Result};
use crate::err;

// use rocket::serde::json::Json;
// use rocket::Error;


pub type JsonResultGeneric<T> = Result<Json<T>>;

/// List all table identifiers underneath a given namespace
#[get("/namespaces/<namespace>/tables")]
pub fn get_table_by_namespace(namespace: &str) -> JsonResultGeneric<ListTablesResponse> {
  // Initialize TableIdentifier instances
  let identifiers = vec![
    TableIdentifier {
      namespace: Namespace(vec!["accounting".to_string(), "tax".to_string()]),
      name: "paid".to_string(),
    },
    TableIdentifier {
      namespace: Namespace(vec!["accounting".to_string(), "tax".to_string()]),
      name: "owed".to_string(),
    },
  ];

  // Create and return ListTablesResponse
  let response = ListTablesResponse { identifiers };

  Ok(Json(response))
}

/// Create a table in the given namespace
#[post("/namespaces/<namespace>/tables")]
pub fn post_table_by_namespace(namespace: &str) -> JsonResultGeneric<CreateTableResponse> {
  // Generate metadata for the newly created table
  let metadata = TableMetadata {
    format_version: 1,
    table_uuid: "generated_uuid".to_string(),
    // Fill in other fields as needed
  };

  // Construct the response
  let response = CreateTableResponse { metadata };

  // Return the response as JSON
  Ok(Json(response))
}

/// Register a table in the given namespace using given metadata file location
#[post("/namespaces/<namespace>/register")]
pub fn register_table(namespace: &str) -> JsonResultGeneric<LoadTableResponse> {
  // Generate metadata for the newly created table
  let metadata = TableMetadata {
    format_version: 1,
    table_uuid: "generated_uuid".to_string(),
    // Fill in other fields as needed
  };

  // Construct the response
  let response = LoadTableResponse { metadata };

  // Return the response as JSON
  Ok(Json(response))
}

/// Load a table from the catalog
#[get("/namespaces/<namespace>/tables/<table>")]
pub fn get_table(namespace: &str, table: &str) -> JsonResultGeneric<LoadTableResponse> {
  // Generate metadata for the newly created table
  let metadata = TableMetadata {
    format_version: 1,
    table_uuid: "generated_uuid".to_string(),
    // Fill in other fields as needed
  };

  // Construct the response
  let response = LoadTableResponse { metadata };

  // Return the response as JSON
  Ok(Json(response))
}

/// Commit updates to a table
#[post("/namespaces/<namespace>/tables/<table>")]
pub fn post_table(namespace: &str, table: &str) -> status::Custom<content::RawJson<&'static str>> {
  let bad_request = namespace.is_empty() || table.is_empty();
  // let bad_request = true;
  let post_success = false;
  let namespace_found = false;
  let table_exist_already = false;

  if bad_request{
    status::Custom(Status::BadRequest, content::RawJson("{ \"Error 400 BadRequest\": \"Namespace or table name empty\" }")) 
  } else {
    if post_success{
      status::Custom(Status::Ok, content::RawJson("")) // successful request 200
    } else {
      if !namespace_found{
        status::Custom(Status::NotFound, content::RawJson("{ \"Error 404 NotFound\": \"Namespace not found\" }"))
      } else {
        if table_exist_already{
          status::Custom(Status::Conflict, content::RawJson("{ \"Error 409 Conflict\": \"Table exist already\" }"))
        } else {
          status::Custom(Status::InternalServerError, content::RawJson("{ \"Error 5XX Others\": \"Server error\" }"))
        }
      }
    }
  }
}

/// Drop a table from the catalog
#[delete("/namespaces/<namespace>/tables/<table>")]
pub fn delete_table(namespace: &str, table: &str) -> status::Custom<content::RawJson<&'static str>> {
  // let bad_request = namespace.is_empty() || table.is_empty();
  let bad_request = true;
  let delete_success = true;
  let table_found = false;

  if bad_request{
    status::Custom(Status::BadRequest, content::RawJson("{ \"Error 400 BadRequest\": \"Namespace or table name empty\" }")) 
  } else {
    if delete_success{
      status::Custom(Status::NoContent, content::RawJson("")) // 204 successful request
    } else {
      if !table_found{
        status::Custom(Status::NotFound, content::RawJson("{ \"Error 404 NotFound\": \"Table not found\" }"))
      } else {
        status::Custom(Status::InternalServerError, content::RawJson("{ \"Internal server error\": \"testing\" }"))
      }
    }
  }
}

/// Check if a table exists
#[head("/namespaces/<namespace>/tables/<table>")]
pub fn head_table(namespace: &str, table: &str) -> status::Custom<content::RawJson<&'static str>> {
  let bad_request = false;
  let table_found = true;
  let error_occur = true;

  if bad_request{
    status::Custom(Status::BadRequest, content::RawJson("{ \"Error 400 BadRequest\": \"Namespace or table name empty\" }")) 
  } else {
    if !error_occur{
      if table_found{
        status::Custom(Status::NoContent, content::RawJson("")) // 204 successful request
      } else {
        status::Custom(Status::NotFound, content::RawJson("{ \"Error 404 NotFound\": \"Table not found\" }"))
      }
    } else {
      status::Custom(Status::InternalServerError, content::RawJson("{ \"Internal server error\": \"testing\" }"))
    }
  }
}

/// Rename a table from its current name to a new name
#[post("/tables/rename")]
pub fn rename_table() -> status::Custom<content::RawJson<&'static str>> {
  let bad_request = true;
  let table_renamed = false;
  let namespace_found = false;
  let table_found = false;

  if bad_request{
    status::Custom(Status::BadRequest, content::RawJson("{ \"Error 400 BadRequest\": \"Namespace or table name empty\" }"))
  } else {
    if table_renamed {
      status::Custom(Status::NoContent, content::RawJson("")) // 204 successful request
    } else {
      if !namespace_found{
        status::Custom(Status::NotFound, content::RawJson("{ \"Error 404 NotFound\": \"Namespace not found\" }"))
      } else if !table_found{
        status::Custom(Status::NotFound, content::RawJson("{ \"Error 404 NotFound\": \"Table not found\" }"))
      } else {
        status::Custom(Status::InternalServerError, content::RawJson("{ \"Internal server error\": \"testing\" }"))
      }
    }
  }
}
