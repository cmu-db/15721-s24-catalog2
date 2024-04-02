use std::rc::Rc;

use crate::{
  common::result::{ErrorType, Location, Result},
  err,
  util::time,
};
use rocket::serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::db::DBConnection;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Table {
  pub properties: Value,
}

