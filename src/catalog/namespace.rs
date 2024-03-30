use std::rc::Rc;

use crate::common::error::Result;
use crate::db::DBConnection;

type NamespaceIdent = String;
type NamespaceIdentRef<'a> = &'a str;

type NamespaceRef = Rc<Namespace>;

pub struct Namespace {
  pub ident: NamespaceIdent,
  pub parent: Option<NamespaceIdent>,
  pub children: Vec<NamespaceRef>,
}

fn hash(ident: NamespaceIdentRef<'_>, parent: Option<NamespaceIdentRef<'_>>) -> String {
  format!(
    "{}::{}",
    parent.map_or_else(|| "root".to_string(), |p| p.to_owned()),
    ident
  )
}

impl Namespace {
  // exist will not return an error
  pub async fn exists(
    conn: &DBConnection,
    ident: NamespaceIdentRef<'_>,
    parent: Option<NamespaceIdentRef<'_>>,
  ) -> Result<bool> {
    let key = hash(ident, parent);
    conn.exists(&key)
  }
}
