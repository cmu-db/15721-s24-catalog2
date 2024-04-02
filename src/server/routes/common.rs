use crate::catalog::namespace::Namespace;
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
