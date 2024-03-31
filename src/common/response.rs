use std::io::Cursor;

use crate::common::result::{Error, ErrorType};
use rocket::{
  http::{ContentType, Status},
  response::{self, Responder},
  serde::json::json,
  Request, Response,
};

impl ErrorType {
  fn to_status(&self) -> Status {
    match self {
      ErrorType::BadRequest => Status::BadRequest,
      ErrorType::NotFound => Status::NotFound,
      ErrorType::ServiceUnavailable => Status::ServiceUnavailable,
      ErrorType::AlreadyExists => Status::Conflict,
      ErrorType::Unprocessable => Status::UnprocessableEntity,
      ErrorType::InternalError => Status::InternalServerError,
    }
  }
}

// HTTP response builder for Error enum
impl<'r> Responder<'r, 'static> for Error {
  fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
    let status = self.error_type.to_status();
    let body: String = json!({ "error": {
      "code": status.code,
      "type": self.error_type,
      "message": self.message,
    } })
    .to_string();

    // Build and send the request.
    Response::build()
      .sized_body(body.len(), Cursor::new(body))
      .header(ContentType::new("application", "json"))
      .status(status)
      .ok()
  }
}

#[macro_export]
macro_rules! err {
  ($error_type:expr, $location:expr, $message:expr) => {
    Err(crate::common::result::Error {
      error_type: $error_type,
      location: $location,
      message: $message,
    })
  };
}
