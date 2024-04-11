// use crate::catalog::namespace::Namespace;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TableIdentifier {
  pub namespace: NamespaceResponse, // TODO: Should update this Namespace
  pub name: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NamespaceResponse(pub Vec<String>);

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[serde(untagged)]
pub enum Type {
  Primitive(PrimitiveType),
  Struct(StructType),
  List(Box<ListType>),
  Map(Box<MapType>),
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PrimitiveType(String);

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct StructField {
  id: i32,
  name: String,
  #[serde(flatten)]
  type_: Type,
  required: bool,
  // doc: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct StructType {
  #[serde(rename = "type")]
  pub type_: String,
  pub fields: Vec<StructField>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ListType {
  #[serde(rename = "type")]
  type_: String,
  element_id: i32,
  #[serde(flatten)]
  element: Type,
  element_required: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct MapType {
  #[serde(rename = "type")]
  type_: String,
  key_id: i32,
  #[serde(flatten)]
  key: Type,
  value_id: i32,
  #[serde(flatten)]
  value: Type,
  value_required: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Schema {
  // #[serde(flatten)]
  // pub struct_type: StructType, // TODO: update!
  pub schema_id: i32,
  pub identifier_field_ids: Vec<i32>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TableMetadata {
  pub format_version: i32,
  pub table_uuid: String,
  // pub location: Option<String>,
  // pub last_updated_ms: Option<i64>,
  // pub properties: HashMap<String, String>,
  // pub schemas: Vec<Schema>,
  // pub current_schema_id: Option<i32>,
  // pub last_column_id: Option<i32>,
  // pub partition_specs: Vec<PartitionSpec>,
  // pub default_spec_id: Option<i32>,
  // pub last_partition_id: Option<i32>,
  // pub sort_orders: Vec<SortOrder>,
  // pub default_sort_order_id: Option<i32>,
  // pub snapshots: Vec<Snapshot>,
  // pub refs: SnapshotReferences,
  // pub current_snapshot_id: Option<i64>,
  // pub last_sequence_number: Option<i64>,
  // pub snapshot_log: SnapshotLog,
  // pub metadata_log: MetadataLog,
}

use std::sync::atomic::{AtomicUsize, Ordering};

pub struct TableMetadataAtomicIncr {
  table_uuid_counter: AtomicUsize,
}

impl TableMetadataAtomicIncr {
  pub fn new() -> Self {
    TableMetadataAtomicIncr {
      table_uuid_counter: AtomicUsize::new(0),
    }
  }

  pub fn generate_table_metadata(&self, format_version: i32) -> TableMetadata {
    let uuid = self.table_uuid_counter.fetch_add(1, Ordering::SeqCst);
    let table_uuid = format!("uuid{}", uuid); // Generate UUID based on the counter value
    TableMetadata {
      format_version,
      table_uuid,
    }
  }
}
