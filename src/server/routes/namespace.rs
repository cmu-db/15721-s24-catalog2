use crate::catalog::namespace::{Namespace, NamespaceIdent};
use crate::common::result::{self, EmptyResult, ErrorType, JsonResult, Location, Result};
use crate::err;

use rocket::request::FromParam;

use rocket::{
  serde::{
    json::{json, Json, Value},
    Deserialize,
  },
  State,
};

use crate::db::DB;

pub struct NamespaceParam(Vec<NamespaceIdent>);

/// Returns an instance of `PasteId` if the path segment is a valid ID.
/// Otherwise returns the invalid ID as the `Err` value.
impl<'r> FromParam<'r> for NamespaceParam {
  type Error = result::Error;
  fn from_param(param: &'r str) -> Result<Self> {
    NamespaceParam::try_from(param)
  }
}

impl TryFrom<&str> for NamespaceParam {
  type Error = result::Error;
  fn try_from(param: &str) -> Result<NamespaceParam> {
    let parts: Vec<_> = param.split('\u{001F}').collect();
    // check if all parts are valid
    if !parts
      .iter()
      .all(|p| p.chars().all(|c| c.is_ascii_alphanumeric()))
    {
      return err!(
        ErrorType::BadRequest,
        Location::Namespace,
        "Invalid parameter".to_owned()
      );
    }
    Ok(NamespaceParam(
      parts.into_iter().map(|x| x.to_string()).collect(),
    ))
  }
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
// Create Namespace Request
pub struct CreateNamespaceRequest {
  namespace: Vec<NamespaceIdent>,
  // Configured string to string map of properties for the namespace
  properties: Option<Value>,
}

/// List namespaces, optionally providing a parent namespace to list underneath
#[get("/namespaces?<parent..>")]
pub async fn get(parent: &str, db: &State<DB>) -> JsonResult {
  let conn = db.get_read_conn()?;
  let parent = NamespaceParam::try_from(parent)?.0;
  let res = Namespace::list(&conn, &parent);
  match res {
    None => err!(
      ErrorType::NotFound,
      Location::Namespace,
      format!("Namespace {} not found", parent.join("."))
    ),
    Some(v) => Ok(Json(json!( { "namespaces": v }))),
  }
}

/// Create a namespace
#[post("/namespaces", data = "<create_request>")]
pub async fn post(create_request: Json<CreateNamespaceRequest>, db: &State<DB>) -> JsonResult {
  let mut conn = db.get_write_conn()?;
  let created_namespace = Namespace::create(
    &mut conn,
    &create_request.namespace,
    create_request.properties.clone(), // FIXME: this is a clone, can it be avoided?
  )?;
  Ok(Json(json!({
    "namespace": create_request.namespace.clone(),
    "properties": created_namespace.properties,
  })))
}

/// Check if a namespace exists
#[head("/<namespace>")]
pub async fn head_by_name(namespace: NamespaceParam, db: &State<DB>) -> EmptyResult {
  let conn = db.get_read_conn()?;
  let exists = Namespace::exists(&conn, &namespace.0);
  match exists {
    true => Ok(()),
    false => err!(
      ErrorType::NotFound,
      Location::Namespace,
      format!("Namespace {} not found", namespace.0.join("."))
    ),
  }
}

/// Load the metadata properties for a namespace
#[get("/<namespace>")]
pub async fn get_by_name(namespace: NamespaceParam, db: &State<DB>) -> JsonResult {
  let conn = db.get_read_conn()?;
  let properties = Namespace::get_properties(&conn, &namespace.0)?;
  Ok(Json(json!({ "properties": properties })))
}

/// Drop a namespace from the catalog. Namespace must be empty.
#[delete("/<namespace>")]
pub fn delete_by_name(namespace: NamespaceParam, db: &State<DB>) {
  todo!("delete_namespace_by_name")
}

/// Set or remove properties on a namespace
#[post("/<namespace>/properties")]
pub fn post_properties(namespace: NamespaceParam, db: &State<DB>) {
  todo!("post_namespace_properties")
}

pub fn stage() -> rocket::fairing::AdHoc {
  rocket::fairing::AdHoc::on_ignite("namespace", |rocket| async {
    rocket
      .mount(
        "/v1/namespaces",
        routes![
          post,
          head_by_name,
          get_by_name,
          delete_by_name,
          post_properties
        ],
      )
      .mount("/v1", routes![get]) // for a query parameter
  })
}
