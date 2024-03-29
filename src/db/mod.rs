use crate::common::error::{Code, Error, Result};
use pickledb::PickleDb;
use rocket::serde::Serialize;

pub struct DB {
  conn: DBConnection,
}

impl DB {
  pub fn get_connection(&self) -> &DBConnection {
    &self.conn
  }

  pub fn new() -> Result<DB> {
    let conn = DBConnection::new()?;
    Ok(DB { conn })
  }
}

pub struct DBConnection(PickleDb);

impl DBConnection {
  pub fn exists(&self, key: &str) -> Result<bool> {
    Ok(self.0.exists(key))
  }

  pub fn get(&self, key: &str) -> Result<Option<String>> {
    Ok(self.0.get(key))
  }

  pub fn put<T: Serialize>(&mut self, key: &str, value: &T) -> Result<()> {
    match self.0.set(key, &value) {
      Ok(_) => Ok(()),
      Err(e) => Err(Error::new(Code::Internal(e.to_string()))),
    }
  }

  fn new() -> Result<DBConnection> {
    // Load the database from disk, if no database exists, create a new one.
    match PickleDb::load(
      "catalog.namespace",
      pickledb::PickleDbDumpPolicy::AutoDump,
      pickledb::SerializationMethod::Json,
    ) {
      Ok(conn) => Ok(DBConnection(conn)),
      Err(_) => {
        let conn = PickleDb::new(
          "catalog.namespace",
          pickledb::PickleDbDumpPolicy::AutoDump,
          pickledb::SerializationMethod::Json,
        );
        Ok(DBConnection(conn))
      }
    }
  }
}
