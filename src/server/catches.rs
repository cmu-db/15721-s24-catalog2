use crate::common::result::{EmptyResult, ErrorType, Location};
use crate::err;

#[catch(404)]
fn general_not_found() -> EmptyResult {
  err!(
    ErrorType::NotFound,
    Location::Request,
    "Resource not found, please check the URL".to_string()
  )
}

#[catch(400)]
fn general_bad_request() -> EmptyResult {
  err!(
    ErrorType::BadRequest,
    Location::Request,
    "Bad Request".to_string()
  )
}

#[catch(422)]
fn general_unprocessable_request() -> EmptyResult {
  err!(
    ErrorType::Unprocessable,
    Location::Request,
    "Unprocessable request".to_string()
  )
}

#[catch(500)]
fn general_internal_error() -> EmptyResult {
  err!(
    ErrorType::InternalError,
    Location::Request,
    "Internal server error".to_string()
  )
}

pub fn stage() -> rocket::fairing::AdHoc {
  rocket::fairing::AdHoc::on_ignite("error response guard", |rocket| async {
    rocket.register(
      "/",
      catchers![
        general_not_found,
        general_bad_request,
        general_internal_error,
        general_unprocessable_request
      ],
    )
  })
}
