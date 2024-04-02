use crate::{
  catalog::namespace::Namespace,
  common::result::{Error, ErrorType, Location, Result},
  err,
};
use pickledb::PickleDb;

use rocket::serde::Serialize;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::{fs, path::PathBuf};

pub struct DB {
  conn: RwLock<DBConnection>, // simple rw lock
}

impl DB {
  pub fn get_read_conn(&self) -> Result<RwLockReadGuard<DBConnection>> {
    let read_guard = self.conn.read();
    if read_guard.is_err() {
      return err!(
        ErrorType::InternalError,
        Location::DB,
        "Failed to get read connection".to_owned()
      );
    }

    Ok(read_guard.unwrap())
  }

  pub fn get_write_conn(&self) -> Result<RwLockWriteGuard<DBConnection>> {
    let write_guard = self.conn.write();
    if write_guard.is_err() {
      return err!(
        ErrorType::InternalError,
        Location::DB,
        "Failed to get write connection".to_owned()
      );
    }

    Ok(write_guard.unwrap())
  }

  pub fn new(root_dir: PathBuf) -> Result<DB> {
    println!("starting db in {:?}", root_dir);
    if !std::path::Path::new(&root_dir).exists() {
      let res = fs::create_dir(root_dir);
      if res.is_err() {
        return err!(
          ErrorType::InternalError,
          Location::DB,
          "Failed to create root directory".to_owned()
        );
      }
    }

    let mut conn = DBConnection::new()?;
    Namespace::init(&mut conn)?;
    Ok(DB {
      conn: RwLock::new(conn),
    })
  }
}

pub struct DBConnection(PickleDb);

impl DBConnection {
  pub fn exists(&self, key: &str) -> bool {
    self.0.exists(key)
  }

  pub fn get<V: for<'de> rocket::serde::Deserialize<'de>>(&self, key: &str) -> Option<V> {
    self.0.get(key)
  }

  pub fn put<T: Serialize>(&mut self, key: &str, value: &T) -> Result<()> {
    match self.0.set(key, &value) {
      Ok(_) => Ok(()),
      Err(e) => Err(Error {
        error_type: ErrorType::InternalError,
        location: Location::DB,
        message: format!("Failed to put key: {}, error: {}", key, e),
      }),
    }
  }

  pub fn delete(&mut self, key: &str) -> Result<()> {
    match self.0.rem(key) {
      Ok(_) => Ok(()),
      Err(e) => Err(Error {
        error_type: ErrorType::InternalError,
        location: Location::DB,
        message: format!("Failed to delete key: {}, error: {}", key, e),
      }),
    }
  }

  fn new() -> Result<DBConnection> {
    // Load the database from disk, if no database exists, create a new one.
    match PickleDb::load(
      "./database/catalog.namespace",
      pickledb::PickleDbDumpPolicy::AutoDump,
      pickledb::SerializationMethod::Json,
    ) {
      Ok(conn) => Ok(DBConnection(conn)),
      Err(_) => {
        let conn = PickleDb::new(
          "./database/catalog.namespace",
          pickledb::PickleDbDumpPolicy::AutoDump,
          pickledb::SerializationMethod::Json,
        );
        Ok(DBConnection(conn))
      }
    }
  }
}
