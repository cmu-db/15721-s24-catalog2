/// List all table identifiers underneath a given namespace
#[get("/namespaces/<namespace>/tables")]
pub fn get_table_by_namespace(namespace: &str) {
  todo!("get_table_by_namespace")
}

/// Create a table in the given namespace
#[post("/namespaces/<namespace>/tables")]
pub fn post_table_by_namespace(namespace: &str) {
  todo!("post_table_by_namespace")
}

/// Register a table in the given namespace using given metadata file location
#[post("/namespaces/<namespace>/register")]
pub fn register_table(namespace: &str) {
  todo!("register_table")
}

/// Load a table from the catalog
#[get("/namespaces/<namespace>/tables/<table>")]
pub fn get_table(namespace: &str, table: &str) {
  todo!("post_namespace_table")
}

/// Commit updates to a table
#[post("/namespaces/<namespace>/tables/<table>")]
pub fn post_table(namespace: &str, table: &str) {
  todo!("post_namespace_table")
}

/// Drop a table from the catalog
#[delete("/namespaces/<namespace>/tables/<table>")]
pub fn delete_table(namespace: &str, table: &str) {
  todo!("post_namespace_table")
}

/// Check if a table exists
#[head("/namespaces/<namespace>/tables/<table>")]
pub fn head_table(namespace: &str, table: &str) {
  todo!("post_namespace_table")
}

/// Rename a table from its current name to a new name
#[post("/tables/rename")]
pub fn rename_table() {
  todo!("rename_table")
}
