use crate::catalog::namespace::Namespace;
use crate::common::error::{ErrorType, Location, Result};
use crate::create_error;

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
  let conn = db.get_connection();
  // Namespace::list(conn, parent).await?;
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
  let exists = Namespace::exists(conn, namespace, None).await;
  match exists {
    true => Ok(()),
    false => create_error!(
      ErrorType::NotFound,
      Location::Namespace,
      format!("Namespace {} not found", namespace)
    ),
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
