use crate::catalog::namespace::Namespace;
use crate::common::error::{Error, ErrorType, Location, Result};

use rocket::{
  serde::{
    json::{json, Json, Value},
    Deserialize, Serialize,
  },
  State,
};

use crate::db::DB;

/// List namespaces, optionally providing a parent namespace to list underneath
#[get("/?<parent..>")]
pub async fn get(parent: Option<String>, db: &State<DB>) -> Result<()> {
  // let conn = db.get_connection();
  // Namespace::
  // An optional namespace, underneath which to list namespaces.
  // If not provided or empty, all top-level namespaces should be listed.
  // If parent is a multipart namespace, the parts must be separated by the unit separator (`0x1F`) byte.
  todo!("get_namespace")
}

/// Create a namespace
#[post("/")]
pub fn post() {
  todo!("post_namespace")
}

/// Check if a namespace exists
#[head("/<namespace>")]
pub async fn head_by_name(namespace: &str, db: &State<DB>) -> Result<()> {
  let conn = db.get_connection();
  let res = Namespace::exists(conn, namespace, None).await;
  match res {
    Ok(exists) => {
      if exists {
        Ok(())
      } else {
        Err(Error {
          error_type: ErrorType::NotFound,
          location: Location::Namespace,
          message: format!("Namespace {} not found", namespace),
        })
      }
    }
    Err(e) => Err(e),
  }
}

/// Load the metadata properties for a namespace
#[get("/<namespace>")]
pub fn get_by_name(namespace: &str) {
  todo!("get_namespace_by_name")
}

/// Drop a namespace from the catalog. Namespace must be empty.
#[delete("/<namespace>")]
pub fn delete_by_name(namespace: &str) {
  todo!("delete_namespace_by_name")
}

/// Set or remove properties on a namespace
#[post("/<namespace>/properties")]
pub fn post_properties(namespace: &str) {
  todo!("post_namespace_properties")
}

pub fn stage() -> rocket::fairing::AdHoc {
  rocket::fairing::AdHoc::on_ignite("namespace", |rocket| async {
    rocket.mount(
      "/v1/namespace",
      routes![
        get,
        post,
        head_by_name,
        get_by_name,
        delete_by_name,
        post_properties
      ],
    )
  })
}
