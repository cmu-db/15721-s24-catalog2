use crate::{
  common::result::{ErrorType, Location, Result},
  err,
  server::routes::common::*,
};
use rocket::{
  serde::{Deserialize, Serialize},
  State,
};

// use crate::Location::Namespace; // TODO: update
use crate::catalog::namespace::Namespace;

use crate::db::DBConnection;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Table {
  pub name: String,
  //   pub properties: Value,
  pub schema: Schema,
  pub metadata: TableMetadata,
}

impl Table {
  // exist will not return an error
  pub fn exists(conn: &DBConnection, namespace_name: String, table_name: String) -> bool {
    let table_key = format!("{}_{}", namespace_name, table_name);
    conn.exists(&table_key)

    // TODO: probably want to know whether it is namespace not found or table not found
  }

  pub fn create(
    conn: &mut DBConnection,
    namespace: String,
    table: String,
    table_metedata_generator: &State<TableMetadataAtomicIncr>,
  ) -> Result<Table> {
    let namespace_key = namespace.clone();
    let table_key = format!("{}_{}", namespace, table);
    let table_name = table.clone();
    let table_clone = table.clone();

    // add checking for whether namespace exists
    if !conn.exists(&namespace) {
      return err!(
        ErrorType::NotFound,
        Location::Namespace, // ??
        format!("Namespace {} not found", namespace)
      );
    }

    if Table::exists(conn, namespace, table) {
      return err!(
        ErrorType::AlreadyExists,
        Location::Table,
        format!("Table {} already exists", table_key)
      );
    }
    let new_table = Table {
      name: table_name,
      schema: Schema {
        // struct_type: StructType {
        //   type_: "".to_string(),
        //   fields: vec![],
        //   // Populate StructType fields as needed
        // },
        schema_id: 0,                 // Set schema_id to a valid value
        identifier_field_ids: vec![], // Provide identifier_field_ids if needed
      },
      metadata: table_metedata_generator.generate_table_metadata(1),
    };
    conn.put(&table_key, &new_table)?;

    // add the table to the namespace tables
    if let Some(mut namespace_instance) = conn.get::<Namespace>(&namespace_key) {
      namespace_instance.tables.push(table_clone.clone());
      conn.put(&namespace_key, &namespace_instance)?;
    }

    Ok(new_table)
  }

  pub fn delete(conn: &mut DBConnection, namespace: String, table: String) -> Result<()> {
    let table_key = format!("{}_{}", namespace, table);
    let table_name = table.clone();
    if !Table::exists(conn, namespace.clone(), table) {
      return err!(
        ErrorType::NotFound,
        Location::Table,
        format!("Table {} not found", table_key)
      );
    }

    let namespace_key = namespace.clone();
    if let Some(mut namespace_instance) = conn.get::<Namespace>(&namespace_key) {
      // Remove the table from the namespace's tables vector
      if let Some(index) = namespace_instance
        .tables
        .iter()
        .position(|t| t == &table_name)
      {
        namespace_instance.tables.remove(index);
        conn.put(&namespace_key, &namespace_instance)?;
      }
    }

    conn.delete(&table_key)
  }

  pub fn list(conn: &DBConnection, namespace: String) -> Option<Vec<String>> {
    let key = namespace;
    // let namespace_instance = conn.get(key);
    // let tables = namespace_instance.tables;

    // add checking for whether namespace exists
    // let namespace_clone = namespace.clone();
    // if !conn.exists(&namespace_clone){
    //   return err!(
    //       ErrorType::NotFound,
    //       Location::Namespace,
    //       format!("Namespace {} not found", namespace_clone)
    //   );
    // }

    if let Some(namespace_instance) = conn.get::<Namespace>(&key) {
      Some(namespace_instance.tables.clone())
    } else {
      None
    }
    // Some(tables)
  }

  pub fn get(conn: &DBConnection, namespace_name: String, table_name: String) -> Option<Table> {
    let table_key = format!("{}_{}", namespace_name, table_name);
    if let Some(table_instance) = conn.get::<Table>(&table_key) {
      Some(table_instance)
    } else {
      None
    }

    // TODO: probably want to know whether it is namespace not found or table not found
  }

  pub fn rename(
    conn: &mut DBConnection,
    namespace_name: String,
    old_table_name: String,
    new_table_name: String,
  ) -> Result<bool> {
    let old_table_key = format!("{}_{}", namespace_name.clone(), old_table_name.clone());
    let new_table_key = format!("{}_{}", namespace_name.clone(), new_table_name.clone());

    if let Some(mut old_table) = conn.get::<Table>(&old_table_key) {
      old_table.name = new_table_name.clone();
      conn.put(&new_table_key, &old_table)?;
      conn.delete(&old_table_key)?; // Remove the old key
    } else {
      return Ok(false); // If the old table does not exist, return false or handle it accordingly
    }

    // true
    let namespace_key = namespace_name.clone();
    if let Some(mut namespace) = conn.get::<Namespace>(&namespace_key) {
      if let Some(index) = namespace
        .tables
        .iter()
        .position(|name| name == &old_table_name)
      {
        namespace.tables[index] = new_table_name.clone();
        conn.put(&namespace_key, &namespace)?;
        Ok(true)
      } else {
        return Ok(false); // If the old table name is not found in the tables vector, return false or handle it accordingly
      }
    } else {
      return Ok(false); // If the namespace does not exist, return false or handle it accordingly
    }
  }
}
