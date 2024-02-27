/// List namespaces, optionally providing a parent namespace to list underneath
#[get("/namespaces?<parent..>")]
pub fn get_namespace(parent: Option<String>) -> String {
  // An optional namespace, underneath which to list namespaces.
  // If not provided or empty, all top-level namespaces should be listed.
  // If parent is a multipart namespace, the parts must be separated by the unit separator (`0x1F`) byte.
  return format!("get_namespace({:?})", parent);
}

/// Create a namespace
#[post("/namespaces")]
pub fn post_namespace() {
  todo!("post_namespace")
}

/// Check if a namespace exists
#[head("/namespaces/<namespace>")]
pub fn head_namespace_by_name(namespace: &str) {
  todo!("head_namespace_by_name")
}

/// Load the metadata properties for a namespace
#[get("/namespaces/<namespace>")]
pub fn get_namespace_by_name(namespace: &str) {
  todo!("get_namespace_by_name")
}

/// Drop a namespace from the catalog. Namespace must be empty.
#[delete("/namespaces/<namespace>")]
pub fn delete_namespace_by_name(namespace: &str) {
  todo!("delete_namespace_by_name")
}

/// Set or remove properties on a namespace
#[post("/namespaces/<namespace>/properties")]
pub fn post_namespace_properties(namespace: &str) {
  todo!("post_namespace_properties")
}