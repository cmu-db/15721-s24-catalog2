use crate::request::*;
use crate::server::routes::common::*;
use crate::{err, ok_empty, response::*};
use rocket::http::Status;
use rocket::response::{content, status};
use rocket::serde::json::Json;

use crate::common::result::{EmptyResult, ErrorType, Location, Result};
use crate::catalog::table::Table;
use crate::catalog::namespace::Namespace;
use crate::server::routes::namespace::NamespaceParam;

use crate::DB;
use rocket::State;

pub type JsonResultGeneric<T> = Result<Json<T>>;


fn hash<'a>(level: &Vec<String>) -> String {
  if level.is_empty() {
    "root".to_string()
  } else {
    format!("root::{}", level.join("::"))
  }
}

/// List all table identifiers underneath a given namespace
#[get("/namespaces/<namespace>/tables")]
pub fn get_table_by_namespace(namespace: NamespaceParam, db: &State<DB>) -> JsonResultGeneric<ListTablesResponse> {
  let mut conn = db.get_read_conn()?;
  let copy = namespace.0.clone();
  let hash_key = hash(&namespace.0);
  let table_names = Table::list(
    &mut conn,
    hash_key.to_string(), 
  );
  let all_table_names = table_names.clone();
  
  let mut identifiers = Vec::new();
  for table_name in all_table_names.into_iter().flatten() {
      let identifier = TableIdentifier {
          namespace: NamespaceResponse(copy.clone() ), // Assuming namespace is a Vec<String>
          name: table_name.clone(),
      };
      identifiers.push(identifier);
  }

  if identifiers.is_empty() {
    return err!(
        ErrorType::NotFound,
        Location::Table,
        format!("No tables found for the specified namespace")
    );
  }

  // Create and return ListTablesResponse
  let response = ListTablesResponse { identifiers };

  Ok(Json(response))
}

/// Create a table in the given namespace
// TODO: check whether namespace exists first
#[post("/namespaces/<namespace>/tables", data = "<create_table_request>")]
pub fn post_table_by_namespace(namespace: &str, create_table_request: Json<CreateTableRequest>, db: &State<DB>) -> JsonResultGeneric<CreateTableResponse> {
  let mut conn = db.get_write_conn()?;
  let new_table = Table::create(
    &mut conn,
    namespace.to_string(),
    create_table_request.name.clone().to_string(), // FIXME: this is a clone, can it be avoided?
  )?;
  
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
#[post("/namespaces/<namespace>/register", data = "<register_table_request>")]
pub fn register_table(
  namespace: &str,
  register_table_request: Json<RegisterTableRequest>,
) -> JsonResultGeneric<LoadTableResponse> {
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
pub fn get_table(namespace: &str, table: &str, db: &State<DB>) -> JsonResultGeneric<LoadTableResponse> {
  let conn = db.get_read_conn()?;
  let table_data = Table::get(
    &conn,
    namespace.to_string(),
    table.to_string(), // FIXME: this is a clone, can it be avoided?
  );
  
  // TODO: update to real metadata
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
#[post("/namespaces/<namespace>/tables/<table>", data = "<commit_table_request>")]
pub fn post_table(namespace: &str, table: &str, commit_table_request: Json<CommitTableRequest>) -> JsonResultGeneric<CommitTableResponse> {
  // TODO: need to update metadata
  // Generate metadata for the newly created table
  let metadata = TableMetadata {
    format_version: 1,
    table_uuid: "generated_uuid".to_string(),
    // Fill in other fields as needed
  };

  // Construct the response
  let response = CommitTableResponse {
    metadata,
    metadata_location: "".to_string(),
  };

  // Return the response as JSON
  Ok(Json(response))
}

/// Drop a table from the catalog
#[delete("/namespaces/<namespace>/tables/<table>?<purge_requested..>")]
pub fn delete_table(namespace: &str, table: &str, purge_requested: PurgeRequested, db: &State<DB>) -> EmptyResult {
  let mut conn = db.get_write_conn()?;
  Table::delete(&mut conn, namespace.to_string(), table.to_string())?;
  ok_empty!()
}

/// Check if a table exists
#[head("/namespaces/<namespace>/tables/<table>")]
pub fn head_table(namespace: &str, table: &str, db: &State<DB>) -> EmptyResult {
  let conn = db.get_read_conn()?;
  let exists = Table::exists(&conn, namespace.to_string(), table.to_string());

  // let error = false;
  match exists {
    // true => Ok(()),
    true => ok_empty!(),
    false => err!(
      ErrorType::NotFound,
      Location::Table,
      format!("Table not found")
    ),
  }
}

/// Rename a table from its current name to a new name
#[post("/tables/rename", data = "<rename_table_request>")]
pub fn rename_table(rename_table_request: Json<RenameTableRequest>, db: &State<DB>) -> EmptyResult {
  let mut conn = db.get_write_conn()?;
  // Table::rename(&mut conn, namespace.to_string(), table.to_string())?;
  let tmp = &rename_table_request.source.namespace.0;
  let namespace_hash = hash(&tmp);

  // Table::rename(&mut conn, "a".to_string(), rename_table_request.source.name.clone(), rename_table_request.destination.name.clone());
  Table::rename(&mut conn, namespace_hash, rename_table_request.source.name.clone(), rename_table_request.destination.name.clone());
  ok_empty!()

  // let error = false;
  // match !error {
  //   // true => Ok(()),
  //   true => ok_empty!(),
  //   false => err!(
  //     ErrorType::NotFound,
  //     Location::Table,
  //     format!("Table not found")
  //   ),
  // }
}
