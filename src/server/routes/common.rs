use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TableIdentifier {
  pub namespace: Namespace,
  pub name: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Namespace(pub Vec<String>);

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
  type_: String,
  fields: Vec<StructField>,
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
  #[serde(flatten)]
  struct_type: StructType,
  schema_id: i32,
  identifier_field_ids: Vec<i32>,
}
