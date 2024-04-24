/// Send a metrics report to this endpoint to be processed by the backend
#[post("/namespaces/<namespace>/tables/<table>/metrics")]
pub fn post_metrics(namespace: &str, table: &str) {
  todo!("post_metrics {} {}", namespace, table)
}
