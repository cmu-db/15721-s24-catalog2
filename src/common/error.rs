use std::{fmt, result};

/// An enum that represents all types of errors that can occur when using calling catalog service.
pub enum ErrorType {
  /// Any error that cause the service to fail.
  Internal,
  /// An error when invalid arguments are passed in.
  InvalidArgument,
  /// An error when the requested resource is not found.
  NotFound,
  /// An error when the inserted resource already exists.
  AlreadyExists,
}

pub enum Code {
  Internal(String),
  InvalidArgument(String),
  NotFound(String),
  AlreadyExists(String),
}

// Error represents an application-specific error. Application errors can be
// unwrapped by the caller to extract out the code & message.
//
// Any non-application error (such as a disk error) should be reported as an
// Internal error and the human user should only see "Internal error" as the
// message. These low-level internal error details should only be logged and
// reported to the operator of the application (not the end user).

pub struct Error {
  // error code.
  code: Code,
}

// alias for Result with the error type.
pub type Result<T> = result::Result<T, Error>;

impl Error {
  pub fn new(code: Code) -> Error {
    Error { code }
  }

  /// Get the error type
  pub fn get_type(&self) -> ErrorType {
    match self.code {
      Code::Internal(_) => ErrorType::Internal,
      Code::InvalidArgument(_) => ErrorType::InvalidArgument,
      Code::NotFound(_) => ErrorType::NotFound,
      Code::AlreadyExists(_) => ErrorType::AlreadyExists,
    }
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self.code {
      Code::Internal(ref msg) => f.write_str(msg),
      Code::InvalidArgument(ref msg) => f.write_str(msg),
      Code::NotFound(ref msg) => f.write_str(msg),
      Code::AlreadyExists(ref msg) => f.write_str(msg),
    }
  }
}

impl fmt::Debug for Error {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.write_str(&format!(
      "Error {{ msg: {} }}",
      match self.code {
        Code::Internal(ref msg) => msg.to_string(),
        Code::InvalidArgument(ref msg) => msg.to_string(),
        Code::NotFound(ref msg) => msg.to_string(),
        Code::AlreadyExists(ref msg) => msg.to_string(),
      }
    ))
  }
}

impl std::error::Error for Error {}
