use crate::request::*;
use crate::server::catches;
use crate::server::routes::common::*;
use crate::{err, ok_empty, response::*};
use rocket::futures::io::Empty;
use rocket::http::Status;
use rocket::response::{content, status};
use rocket::serde::json::Json;

// use crate::catalog::namespace::Namespace;
use crate::catalog::namespace::{Namespace, NamespaceIdent};
use crate::catalog::table::Table;
use crate::common::result::{EmptyResult, ErrorType, Location, Result};
use crate::server::routes::namespace::CreateNamespaceRequest;
use crate::server::routes::namespace::NamespaceParam;

use crate::DB;
use rocket::State;

use super::*;
use rocket::http::ContentType;
use rocket::local::asynchronous::Client;

use std::path::PathBuf;
use tempfile::tempfile;

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
pub fn get_table_by_namespace(
  namespace: NamespaceParam,
  db: &State<DB>,
) -> JsonResultGeneric<ListTablesResponse> {
  let mut conn = db.get_read_conn()?;
  let copy = namespace.0.clone();
  let hash_key = hash(&namespace.0);
  let table_names = Table::list(&mut conn, hash_key.to_string());
  let all_table_names = table_names.clone();

  let mut identifiers = Vec::new();
  for table_name in all_table_names.into_iter().flatten() {
    let identifier = TableIdentifier {
      namespace: NamespaceResponse(copy.clone()), // Assuming namespace is a Vec<String>
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
pub fn post_table_by_namespace(
  namespace: NamespaceParam,
  create_table_request: Json<CreateTableRequest>,
  db: &State<DB>,
  table_metedata_generator: &State<TableMetadataAtomicIncr>,
) -> JsonResultGeneric<CreateTableResponse> {
  let mut conn = db.get_write_conn()?;
  let hash_key = hash(&namespace.0);
  let new_table = Table::create(
    &mut conn,
    hash_key.to_string(),
    create_table_request.name.clone().to_string(),
    table_metedata_generator,
  )?;

  // Generate metadata for the newly created table
  let metadata = TableMetadata {
    format_version: new_table.metadata.format_version,
    table_uuid: new_table.metadata.table_uuid,
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
pub fn get_table(
  namespace: NamespaceParam,
  table: &str,
  db: &State<DB>,
) -> JsonResultGeneric<LoadTableResponse> {
  let conn = db.get_read_conn()?;
  let hash_key = hash(&namespace.0);
  let table_data_option = Table::get(
    &conn,
    hash_key.to_string(),
    table.to_string(), // FIXME: this is a clone, can it be avoided?
  );

  // TODO: update to real metadata
  let table_data = table_data_option.unwrap();
  // Generate metadata for the newly created table
  let metadata = TableMetadata {
    format_version: table_data.metadata.format_version,
    table_uuid: table_data.metadata.table_uuid,
    // Fill in other fields as needed
  };

  // Construct the response
  let response = LoadTableResponse { metadata };

  // Return the response as JSON
  Ok(Json(response))
}

/// Commit updates to a table
#[post(
  "/namespaces/<namespace>/tables/<table>",
  data = "<commit_table_request>"
)]
pub fn post_table(
  namespace: &str,
  table: &str,
  commit_table_request: Json<CommitTableRequest>,
) -> JsonResultGeneric<CommitTableResponse> {
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
pub fn delete_table(
  namespace: NamespaceParam,
  table: &str,
  purge_requested: PurgeRequested,
  db: &State<DB>,
) -> EmptyResult {
  let mut conn = db.get_write_conn()?;
  let hash_key = hash(&namespace.0);
  Table::delete(&mut conn, hash_key.to_string(), table.to_string())?;
  ok_empty!()
}

/// Check if a table exists
#[head("/namespaces/<namespace>/tables/<table>")]
pub fn head_table(namespace: NamespaceParam, table: &str, db: &State<DB>) -> EmptyResult {
  let conn = db.get_read_conn()?;
  let hash_key = hash(&namespace.0);
  let exists = Table::exists(&conn, hash_key.to_string(), table.to_string());

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
  Table::rename(
    &mut conn,
    namespace_hash,
    rename_table_request.source.name.clone(),
    rename_table_request.destination.name.clone(),
  )?;
  ok_empty!()
}

#[rocket::async_test]
async fn test_get_table_by_namespace_empty_result() {
  let temp_dir = tempfile::tempdir().expect("failed to create a temporary directory");
  let client = create_mock_client(temp_dir.path().to_path_buf()).await;

  let namespace_name = "namespacenametest";
  let url = format!("/v1/namespaces/{}/tables", namespace_name);
  let response = client.get(&url).dispatch().await;

  assert_eq!(response.status(), Status::NotFound);
}

#[rocket::async_test]
async fn test_get_table_by_namespace_result_found() {
  let temp_dir = tempfile::tempdir().expect("failed to create a temporary directory");
  let client = create_mock_client(temp_dir.path().to_path_buf()).await;

  let namespace_name = "namespacenametest";
  let create_namespace_request = CreateNamespaceRequest {
    namespace: vec![namespace_name.to_string()], // Use String directly
    properties: None,                            // Adjust as needed
  };
  let create_namespace_request_json = Json(create_namespace_request);
  let create_namespace_request_json_bytes =
    serde_json::to_vec(&create_namespace_request_json.into_inner()).unwrap();

  let response = client
    .post("/v1/namespaces")
    .header(ContentType::JSON)
    .body(create_namespace_request_json_bytes)
    .dispatch()
    .await;

  assert_eq!(response.status(), Status::Ok);

  let url = format!("/v1/namespaces/{}/tables", namespace_name);
  let get_response = client.get(&url).dispatch().await;
  assert_eq!(get_response.status(), Status::NotFound);

  let create_table_request = CreateTableRequest {
    name: "tablenametest".to_string(),
  };
  let create_table_request_json = Json(create_table_request);
  let json_bytes = serde_json::to_vec(&create_table_request_json.into_inner()).unwrap();

  let post_response = client
    .post(format!("/v1/namespaces/{}/tables", namespace_name))
    .header(ContentType::JSON)
    .body(json_bytes)
    .dispatch()
    .await;

  assert_eq!(post_response.status(), Status::Ok);

  let get_response_2 = client.get(&url).dispatch().await;
  assert_eq!(get_response_2.status(), Status::Ok);
}

#[rocket::async_test]
async fn test_post_table_by_namespace_new_table() {
  let temp_dir = tempfile::tempdir().expect("failed to create a temporary directory");
  let client = create_mock_client(temp_dir.path().to_path_buf()).await;

  let namespace_name = "testnamespacename";
  let create_namespace_request = CreateNamespaceRequest {
    namespace: vec![namespace_name.to_string()],
    properties: None,
  };
  let create_namespace_request_json = Json(create_namespace_request);
  let create_namespace_request_json_bytes =
    serde_json::to_vec(&create_namespace_request_json.into_inner()).unwrap();

  let response = client
    .post("/v1/namespaces")
    .header(ContentType::JSON)
    .body(create_namespace_request_json_bytes)
    .dispatch()
    .await;

  assert_eq!(response.status(), Status::Ok);
}

#[rocket::async_test]
async fn test_post_table_by_namespace_conflict() {
  let temp_dir = tempfile::tempdir().expect("failed to create a temporary directory");
  let client = create_mock_client(temp_dir.path().to_path_buf()).await;

  let namespace_name = "testnamespacename";
  let create_namespace_request = CreateNamespaceRequest {
    namespace: vec![namespace_name.to_string()], // Use String directly
    properties: None,                            // Adjust as needed
  };
  let create_namespace_request_json = Json(create_namespace_request);
  let create_namespace_request_json_bytes =
    serde_json::to_vec(&create_namespace_request_json.into_inner()).unwrap();

  let response = client
    .post("/v1/namespaces")
    .header(ContentType::JSON)
    .body(create_namespace_request_json_bytes.clone())
    .dispatch()
    .await;

  assert_eq!(response.status(), Status::Ok);

  let second_response = client
    .post("/v1/namespaces")
    .header(ContentType::JSON)
    .body(create_namespace_request_json_bytes)
    .dispatch()
    .await;

  assert_eq!(second_response.status(), Status::Conflict);
}

#[rocket::async_test]
async fn test_delete_table_that_exists() {
  let temp_dir = tempfile::tempdir().expect("failed to create a temporary directory");
  let client = create_mock_client(temp_dir.path().to_path_buf()).await;

  let namespace_name = "namespacenametest";
  let create_namespace_request = CreateNamespaceRequest {
    namespace: vec![namespace_name.to_string()], // Use String directly
    properties: None,                            // Adjust as needed
  };
  let create_namespace_request_json = Json(create_namespace_request);
  let create_namespace_request_json_bytes =
    serde_json::to_vec(&create_namespace_request_json.into_inner()).unwrap();

  let response = client
    .post("/v1/namespaces")
    .header(ContentType::JSON)
    .body(create_namespace_request_json_bytes)
    .dispatch()
    .await;

  assert_eq!(response.status(), Status::Ok);

  let url = format!("/v1/namespaces/{}/tables", namespace_name);
  let get_response = client.get(&url).dispatch().await;
  assert_eq!(get_response.status(), Status::NotFound);

  let table_name = "tablenametest";
  let create_table_request = CreateTableRequest {
    name: table_name.to_string(),
  };
  let create_table_request_json = Json(create_table_request);
  let json_bytes = serde_json::to_vec(&create_table_request_json.into_inner()).unwrap();

  let post_response = client
    .post(format!("/v1/namespaces/{}/tables", namespace_name))
    .header(ContentType::JSON)
    .body(json_bytes)
    .dispatch()
    .await;

  assert_eq!(post_response.status(), Status::Ok);

  let get_response_2 = client.get(&url).dispatch().await;
  assert_eq!(get_response_2.status(), Status::Ok);

  let delete_url = format!("/v1/namespaces/{}/tables/{}", namespace_name, table_name);
  let delete_response = client.delete(&delete_url).dispatch().await;
  assert_eq!(delete_response.status(), Status::NoContent);
}

#[rocket::async_test]
async fn test_delete_table_that_not_exists() {
  let temp_dir = tempfile::tempdir().expect("failed to create a temporary directory");
  let client = create_mock_client(temp_dir.path().to_path_buf()).await;

  let namespace_name = "namespacenametest";
  let create_namespace_request = CreateNamespaceRequest {
    namespace: vec![namespace_name.to_string()], // Use String directly
    properties: None,                            // Adjust as needed
  };
  let create_namespace_request_json = Json(create_namespace_request);
  let create_namespace_request_json_bytes =
    serde_json::to_vec(&create_namespace_request_json.into_inner()).unwrap();

  let response = client
    .post("/v1/namespaces")
    .header(ContentType::JSON)
    .body(create_namespace_request_json_bytes)
    .dispatch()
    .await;

  assert_eq!(response.status(), Status::Ok);

  let url = format!("/v1/namespaces/{}/tables", namespace_name);
  let get_response = client.get(&url).dispatch().await;
  assert_eq!(get_response.status(), Status::NotFound);

  let table_name = "tablenametest";
  let create_table_request = CreateTableRequest {
    name: table_name.to_string(),
  };
  let create_table_request_json = Json(create_table_request);
  let json_bytes = serde_json::to_vec(&create_table_request_json.into_inner()).unwrap();

  let post_response = client
    .post(format!("/v1/namespaces/{}/tables", namespace_name))
    .header(ContentType::JSON)
    .body(json_bytes)
    .dispatch()
    .await;

  assert_eq!(post_response.status(), Status::Ok);

  let get_response_2 = client.get(&url).dispatch().await;
  assert_eq!(get_response_2.status(), Status::Ok);

  let table_name_not_exists = "tablenamenotexist";
  let delete_url = format!(
    "/v1/namespaces/{}/tables/{}",
    namespace_name, table_name_not_exists
  );
  let delete_response = client.delete(&delete_url).dispatch().await;
  assert_eq!(delete_response.status(), Status::NotFound);
}

#[rocket::async_test]
async fn test_head_table() {
  let temp_dir = tempfile::tempdir().expect("failed to create a temporary directory");
  let client = create_mock_client(temp_dir.path().to_path_buf()).await;

  let namespace_name = "namespacenametest";
  let create_namespace_request = CreateNamespaceRequest {
    namespace: vec![namespace_name.to_string()], // Use String directly
    properties: None,                            // Adjust as needed
  };
  let create_namespace_request_json = Json(create_namespace_request);
  let create_namespace_request_json_bytes =
    serde_json::to_vec(&create_namespace_request_json.into_inner()).unwrap();

  let response = client
    .post("/v1/namespaces")
    .header(ContentType::JSON)
    .body(create_namespace_request_json_bytes)
    .dispatch()
    .await;

  assert_eq!(response.status(), Status::Ok);

  let table_name = "tablenametest";

  let url = format!("/v1/namespaces/{}/tables/{}", namespace_name, table_name);
  let head_response = client.head(&url).dispatch().await;
  assert_eq!(head_response.status(), Status::NotFound);

  let create_table_request = CreateTableRequest {
    name: table_name.to_string(),
  };
  let create_table_request_json = Json(create_table_request);
  let json_bytes = serde_json::to_vec(&create_table_request_json.into_inner()).unwrap();

  let post_response = client
    .post(format!("/v1/namespaces/{}/tables", namespace_name))
    .header(ContentType::JSON)
    .body(json_bytes)
    .dispatch()
    .await;

  assert_eq!(post_response.status(), Status::Ok);

  let head_response_2 = client.head(&url).dispatch().await;
  assert_eq!(head_response_2.status(), Status::NoContent);
}

#[rocket::async_test]
async fn test_rename_table() {
  let temp_dir = tempfile::tempdir().expect("failed to create a temporary directory");
  let client = create_mock_client(temp_dir.path().to_path_buf()).await;

  let namespace_name = "namespacenametest";
  let create_namespace_request = CreateNamespaceRequest {
    namespace: vec![namespace_name.to_string()], // Use String directly
    properties: None,                            // Adjust as needed
  };
  let create_namespace_request_json = Json(create_namespace_request);
  let create_namespace_request_json_bytes =
    serde_json::to_vec(&create_namespace_request_json.into_inner()).unwrap();

  let response = client
    .post("/v1/namespaces")
    .header(ContentType::JSON)
    .body(create_namespace_request_json_bytes)
    .dispatch()
    .await;

  assert_eq!(response.status(), Status::Ok);

  let table_name = "tablenametest";
  let new_table_name = "renamedtable";

  let url = format!("/v1/namespaces/{}/tables/{}", namespace_name, table_name);
  let rename_url = format!(
    "/v1/namespaces/{}/tables/{}",
    namespace_name, new_table_name
  );
  let head_response = client.head(&url).dispatch().await;
  assert_eq!(head_response.status(), Status::NotFound);

  let create_table_request = CreateTableRequest {
    name: table_name.to_string(),
  };
  let create_table_request_json = Json(create_table_request);
  let json_bytes = serde_json::to_vec(&create_table_request_json.into_inner()).unwrap();

  let post_response = client
    .post(format!("/v1/namespaces/{}/tables", namespace_name))
    .header(ContentType::JSON)
    .body(json_bytes)
    .dispatch()
    .await;

  assert_eq!(post_response.status(), Status::Ok);

  let head_response_2 = client.head(&url).dispatch().await;
  assert_eq!(head_response_2.status(), Status::NoContent);

  // rename table
  // let rename_table_request = RenameTableRequest {
  //   source:{namespace: vec![namespace_name.to_string()], name: table_name},
  //   destination: {namespace: vec![namespace_name.to_string()], name: new_table_name},
  // };
  let rename_table_request = RenameTableRequest {
    source: TableIdentifier {
      namespace: NamespaceResponse(vec![namespace_name.to_string()]),
      name: table_name.to_string(),
    },
    destination: TableIdentifier {
      namespace: NamespaceResponse(vec![namespace_name.to_string()]),
      name: new_table_name.to_string(),
    },
  };
  let rename_table_request_json = Json(rename_table_request);
  let rename_json_bytes = serde_json::to_vec(&rename_table_request_json.into_inner()).unwrap();

  let rename_response = client
    .post("/v1/tables/rename")
    .header(ContentType::JSON)
    .body(rename_json_bytes)
    .dispatch()
    .await;
  assert_eq!(rename_response.status(), Status::NoContent);

  let head_response_3 = client.head(&url).dispatch().await;
  assert_eq!(head_response_3.status(), Status::NotFound);

  let head_response_4 = client.head(&rename_url).dispatch().await;
  assert_eq!(head_response_4.status(), Status::NoContent);
}

pub async fn create_mock_client(temp_dir: PathBuf) -> Client {
  let db_test = DB::new(temp_dir).expect("failed to create a db");

  let table_metadata_generator = TableMetadataAtomicIncr::new();
  let mut rocket = rocket::build();
  rocket = rocket
    .manage(db_test)
    .manage(table_metadata_generator)
    .attach(namespace::stage())
    .attach(catches::stage())
    .mount(
      "/v1",
      routes![
        table::get_table_by_namespace,
        table::post_table_by_namespace,
        table::register_table,
        table::get_table,
        table::post_table,
        table::delete_table,
        table::head_table,
        table::rename_table,
        metric::post_metrics,
        config::get_config,
      ],
    );

  Client::tracked(rocket)
    .await
    .expect("valid rocket instance")
}
