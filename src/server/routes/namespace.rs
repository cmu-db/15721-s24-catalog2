use crate::catalog::namespace::{Namespace, NamespaceIdent};
use crate::common::result::{self, EmptyResult, ErrorType, JsonResult, Location, Result};
use crate::{err, ok_empty, ok_json};
use std::collections::HashSet;

use rocket::request::FromParam;

use rocket::{
  serde::{
    json::{Json, Value},
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

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
// Update Namespace Request
pub struct UpdateNamespaceRequest {
  pub removals: Option<Vec<String>>,
  pub updates: Option<Value>,
}

/// List namespaces, optionally providing a parent namespace to list underneath
#[get("/namespaces?<parent..>")]
pub async fn get(parent: Option<&str>, db: &State<DB>) -> JsonResult {
  let conn = db.get_read_conn()?;
  let parent = if let Some(p_str) = parent {
    NamespaceParam::try_from(p_str)?.0
  } else {
    vec![]
  };
  let res = Namespace::list(&conn, &parent);
  match res {
    None => err!(
      ErrorType::NotFound,
      Location::Namespace,
      format!("Namespace {} not found", parent.join("."))
    ),
    Some(v) => ok_json!( { "namespaces": v }),
  }
}

/// Create a namespace
#[post("/", data = "<create_request>")]
pub async fn post(create_request: Json<CreateNamespaceRequest>, db: &State<DB>) -> JsonResult {
  let mut conn = db.get_write_conn()?;
  let created_namespace = Namespace::create(
    &mut conn,
    &create_request.namespace,
    create_request.properties.clone(), // FIXME: this is a clone, can it be avoided?
  )?;
  ok_json!({
    "namespace": create_request.namespace.clone(),
    "properties": created_namespace.properties,
  })
}

/// Check if a namespace exists
#[head("/<namespace>")]
pub async fn head_by_name(namespace: NamespaceParam, db: &State<DB>) -> EmptyResult {
  let conn = db.get_read_conn()?;
  let exists = Namespace::exists(&conn, &namespace.0);
  match exists {
    true => ok_empty!(),
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
  ok_json!({ "properties": properties })
}

/// Drop a namespace from the catalog. Namespace must be empty.
#[delete("/<namespace>")]
pub async fn delete_by_name(namespace: NamespaceParam, db: &State<DB>) -> EmptyResult {
  let mut conn = db.get_write_conn()?;
  Namespace::delete(&mut conn, &namespace.0)?;
  ok_empty!()
}

/// Set or remove properties on a namespace
#[post("/<namespace>/properties", data = "<update_request>")]
pub fn post_properties(
  namespace: NamespaceParam,
  mut update_request: Json<UpdateNamespaceRequest>,
  db: &State<DB>,
) -> JsonResult {
  // we don't test the uniqueness of the keys in removals, it will be treated as a no-op.
  // we only test if a key is presented both in the removals and update.
  if update_request.updates.is_none() && update_request.removals.is_none() {
    return err!(
      ErrorType::BadRequest,
      Location::Request,
      "No updates or removals provided".to_owned()
    );
  }
  if let (Some(removals), Some(updates)) = (&update_request.removals, &update_request.updates) {
    let mut removed_key: HashSet<&str> = HashSet::new();
    for key in removals {
      removed_key.insert(key);
    }
    if let Some(updates) = updates.as_object() {
      for key in updates.keys() {
        if removed_key.contains(key.as_str()) {
          return err!(
            ErrorType::BadRequest,
            Location::Namespace,
            format!("Key {} is present in both removals and updates", key)
          );
        }
      }
    }
  }

  let mut conn = db.get_write_conn()?;
  let res = Namespace::update(
    &mut conn,
    &namespace.0,
    update_request.removals.take(),
    update_request.updates.take(),
  )?;
  ok_json!(res)
}

pub fn stage() -> rocket::fairing::AdHoc {
  rocket::fairing::AdHoc::on_ignite("namespace routes", |rocket| async {
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
