use rocket::http::Status;
use rocket::response::{content, status};


struct ErrorResponse {
    error: String,
}


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
pub fn get_table(namespace: &str, table: &str) {
  todo!("post_namespace_table")
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
