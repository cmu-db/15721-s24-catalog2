use std::io::Cursor;

use crate::common::error::{Error, ErrorType};
use rocket::{
  http::{ContentType, Status},
  response::{self, Responder},
  Request, Response,
};

// HTTP response builder for Error enum
impl<'r> Responder<'r, 'static> for Error {
  fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
    let status = match self.error_type {
      ErrorType::BadRequest => Status::BadRequest,
      ErrorType::NotFound => Status::NotFound,
      ErrorType::ServiceUnavailable => Status::ServiceUnavailable,
      ErrorType::AlreadyExists => Status::Conflict,
      ErrorType::Unprocessable => Status::UnprocessableEntity,
      ErrorType::InternalError => Status::InternalServerError,
    };

    // Serialize the error data structure into JSON.
    let body = "asd".to_owned();

    // Build and send the request.
    Response::build()
      .sized_body(body.len(), Cursor::new(body))
      .header(ContentType::new("application", "json"))
      .status(status)
      .ok()
  }
}
