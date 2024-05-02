use std::{fmt, result};

use derive_builder::Builder;
use rocket::serde::{json::Json, Serialize};
use serde_json::Value;

/// An enum that represents all types of errors that can occur when using calling catalog service.
#[derive(Clone, Serialize)]
#[serde(crate = "rocket::serde")]
#[allow(dead_code)]
pub enum ErrorType {
  BadRequest,
  NotFound,
  ServiceUnavailable,
  AlreadyExists,
  Unprocessable,
  InternalError,
}

#[derive(Clone, Serialize)]
#[serde(crate = "rocket::serde")]
pub enum Location {
  DB,
  Namespace,
  Request,
  Table,
}

#[derive(Builder, Serialize)]
#[builder(setter(into))]
#[serde(crate = "rocket::serde")]
pub struct Error {
  // error type.
  pub error_type: ErrorType,
  // place where the error occurred.
  pub location: Location,
  // error message.
  pub message: String,
}

pub struct Empty {}

// alias for Result with the error type.
pub type Result<T> = result::Result<T, Error>;
pub type JsonResult = Result<Json<Value>>;
pub type EmptyResult = Result<Empty>;

impl From<()> for Empty {
  fn from(_: ()) -> Empty {
    Empty {}
  }
}

impl fmt::Display for Location {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Location::Namespace => write!(f, "namespace"),
      Location::Table => write!(f, "table"),
      Location::DB => write!(f, "DB"),
      Location::Request => write!(f, "request"),
    }
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self.error_type {
      ErrorType::BadRequest => write!(f, "[{}] Bad Request: {}", self.location, self.message),
      ErrorType::NotFound => write!(f, "[{}] Not Found: {}", self.location, self.message),
      ErrorType::ServiceUnavailable => {
        write!(f, "[{}] Unavailable: {}", self.location, self.message)
      }
      ErrorType::AlreadyExists => write!(f, "[{}] Already Exists: {}", self.location, self.message),
      ErrorType::Unprocessable => write!(f, "[{}] Unprocessable: {}", self.location, self.message),
      ErrorType::InternalError => write!(f, "[{}] Internal Error: {}", self.location, self.message),
    }
  }
}

impl fmt::Debug for Error {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.write_str(&format!("Error {{ msg: {} }}", self.message))
  }
}

impl std::error::Error for Error {}
