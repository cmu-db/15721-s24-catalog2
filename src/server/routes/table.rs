// use rocket::http::Status;
use rocket::response::{content, status};
use rocket::serde::json::Json;
use rocket::http::{Status, ContentType};
use rocket::serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};
use rocket::State;
use crate::catalog;
use crate::catalog::Catalog; 

// static mut MAP: Option<HashMap<String, String>> = None;

/// List all table identifiers underneath a given namespace
#[get("/namespaces/<namespace>/tables")]
pub fn get_table_by_namespace(namespace: &str) {
  todo!("get_table_by_namespace")
}

/// Create a table in the given namespace
#[post("/namespaces/<namespace>/tables")]
pub fn post_table_by_namespace(namespace: &str) {
  todo!("post_table_by_namespace")
}

/// Register a table in the given namespace using given metadata file location
#[post("/namespaces/<namespace>/register")]
pub fn register_table(namespace: &str) {
  todo!("register_table")
}


/// Load a table from the catalog
#[get("/namespaces/<namespace>/tables/<table>")]
pub fn get_table(namespace: &str, table: &str) -> (Status, (ContentType, &'static str)) {

  // MAP.insert(("Test key".to_string(), "Test Value".to_string()));

  // status::Custom(Status::Ok, ContentType::Json(response))
  (Status::ImATeapot, (ContentType::JSON, "{ \"identifiers\": []   }"))
}

/// Commit updates to a table
#[post("/namespaces/<namespace>/tables/<table>")]
pub fn post_table(catalog: &State<Catalog>, namespace: &str, table: &str) -> status::Custom<content::RawJson<&'static str>> {
  let catalog_instance = catalog.inner();
  let (success, message) = catalog_instance.post_table_func(namespace.to_string(), table.to_string());
  
  if success{
    status::Custom(Status::Ok, content::RawJson(""))
  } else {
    match &message[..3] {
      "409" => status::Custom(Status::Conflict, content::RawJson("409 Conflict; Table already exists in this namespace")),
      "400" => status::Custom(Status::BadRequest, content::RawJson("400 Bad Request")),
      "404" => status::Custom(Status::NotFound, content::RawJson("404 NotFound; Namespace not found")),
      _ => status::Custom(Status::InternalServerError, content::RawJson("5xx; Internal Server Error")),
    }
  }
  
  // let bad_request = true;
  // let post_success = false;
  // let namespace_found = false;
  // let table_exist_already = false;

  // if bad_request{
  //   status::Custom(Status::BadRequest, content::RawJson("{ \"Error 400 BadRequest\": \"Namespace or table name empty\" }")) 
  // } else {
  //   if post_success{
  //     status::Custom(Status::Ok, content::RawJson("")) // successful request 200
  //   } else {
  //     if !namespace_found{
  //       status::Custom(Status::NotFound, content::RawJson("{ \"Error 404 NotFound\": \"Namespace not found\" }"))
  //     } else {
  //       if table_exist_already{
  //         status::Custom(Status::Conflict, content::RawJson("{ \"Error 409 Conflict\": \"Table exist already\" }"))
  //       } else {
  //         status::Custom(Status::InternalServerError, content::RawJson("{ \"Error 5XX Others\": \"Server error\" }"))
  //       }
  //     }
  //   }
  // }
}

/// Drop a table from the catalog
#[delete("/namespaces/<namespace>/tables/<table>")]
pub fn delete_table(catalog: &State<Catalog>, namespace: &str, table: &str) -> status::Custom<content::RawJson<&'static str>> {
  
  let catalog_instance = catalog.inner();
  let (success, message) = catalog_instance.delete_table_func(namespace.to_string(), table.to_string());
  
  if success{
    status::Custom(Status::NoContent, content::RawJson(""))
  } else {
    match &message[..3] {
      "400" => status::Custom(Status::BadRequest, content::RawJson("400 Bad Request")),
      "404" => status::Custom(Status::NotFound, content::RawJson("404 NotFound; Table not found")),
      "402" => status::Custom(Status::NotFound, content::RawJson("404 NotFound; Namespace not found")),
      _ => status::Custom(Status::InternalServerError, content::RawJson("5xx; Internal Server Error")),
    }
  }
}

/// Check if a table exists
#[head("/namespaces/<namespace>/tables/<table>")]
pub fn head_table(catalog: &State<Catalog>, namespace: &str, table: &str) -> status::Custom<content::RawJson<&'static str>> {
  
  let catalog_instance = catalog.inner();
  let (success, message) = catalog_instance.head_table_func(namespace.to_string(), table.to_string());
  
  if success{
    status::Custom(Status::NoContent, content::RawJson(""))
  } else {
    match &message[..3] {
      "400" => status::Custom(Status::BadRequest, content::RawJson("400 Bad Request")),
      "404" => status::Custom(Status::NotFound, content::RawJson("404 NotFound; Table not found")),
      "402" => status::Custom(Status::NotFound, content::RawJson("404 NotFound; Namespace not found")),
      _ => status::Custom(Status::InternalServerError, content::RawJson("5xx; Internal Server Error")),
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

