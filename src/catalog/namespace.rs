use std::rc::Rc;

use crate::{
  common::result::{ErrorType, Location, Result},
  err,
  util::time,
};
use rocket::serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::db::DBConnection;

pub type NamespaceIdent = String;
pub type NamespaceIdentRef<'a> = &'a str;

pub type NamespaceRef = Rc<Namespace>;

// we store the namespace as a string, the value should contains all the parent namespaces
// e.g. all the direct child to namespace A.B will be stored in the child field,
// and the ident field will be A::B
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Namespace {
  pub child: Vec<NamespaceIdent>,
  pub properties: Value,
}

fn hash<'a>(level: &Vec<NamespaceIdent>) -> String {
  if level.is_empty() {
    "root".to_string()
  } else {
    format!("root::{}", level.join("::"))
  }
}

impl Namespace {
  // exist will not return an error
  pub fn exists(conn: &DBConnection, level: &Vec<NamespaceIdent>) -> bool {
    let key = hash(level);
    conn.exists(&key)
  }

  // List all the child namespaces of the given parent namespace.
  pub fn list(
    conn: &DBConnection,
    parent: &Vec<NamespaceIdent>,
  ) -> Option<Vec<Vec<NamespaceIdent>>> {
    let key = hash(&parent);
    let res: Option<Namespace> = conn.get(&key);
    if res.is_none() {
      return None;
    }
    let val = res.unwrap();
    let parent: Vec<_> = parent.into_iter().map(|x| x.to_string()).collect();
    Some(
      val
        .child
        .into_iter()
        .map(|x| {
          let mut r = parent.clone();
          r.push(x);
          r
        })
        .collect(),
    )
  }

  // exist will not return an error
  pub fn create(
    conn: &mut DBConnection,
    level: &Vec<NamespaceIdent>,
    properties: Option<Value>,
  ) -> Result<Namespace> {
    let key = hash(level);
    if Namespace::exists(conn, level) {
      return err!(
        ErrorType::AlreadyExists,
        Location::Namespace,
        format!("Namespace {} already exists", key)
      );
    }

    let mut old_properties = properties.unwrap_or_else(|| json!({}));
    let new_properties = old_properties.as_object_mut().unwrap();
    new_properties.insert(
      "created_at".to_string(),
      Value::from(time::now().to_string()),
    );
    let namespace = Namespace {
      child: vec![],
      properties: Value::Object(new_properties.to_owned()),
    };
    conn.put(key.as_str(), &namespace)?;
    Ok(namespace)
  }

  // get will return an error if the namespace does not exist
  pub fn get_properties(conn: &DBConnection, level: &Vec<NamespaceIdent>) -> Result<Option<Value>> {
    let key = hash(level);
    let namespace: Option<Namespace> = conn.get(key.as_str());
    if namespace.is_none() {
      return err!(
        ErrorType::NotFound,
        Location::Namespace,
        format!("Namespace {} not found", key)
      );
    }
    Ok(Some(namespace.unwrap().properties))
  }

  // get will return an error if the namespace does not exist
  pub fn delete(conn: &DBConnection, level: &Vec<NamespaceIdent>) -> Result<Option<Value>> {
    let key = hash(level);
    let namespace: Option<Namespace> = conn.get(key.as_str());
    if namespace.is_none() {
      return err!(
        ErrorType::NotFound,
        Location::Namespace,
        format!("Namespace {} not found", key)
      );
    }
    Ok(Some(namespace.unwrap().properties))
  }
}
