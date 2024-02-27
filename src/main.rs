#[macro_use]
extern crate rocket;

mod server;
use server::routes::*;

#[launch]
fn rocket() -> _ {
  rocket::build().mount(
    "/v1",
    routes![
      namespace::get_namespace,
      namespace::post_namespace,
      namespace::head_namespace_by_name,
      namespace::get_namespace_by_name,
      namespace::delete_namespace_by_name,
      namespace::post_namespace_properties,
      table::get_table_by_namespace,
      table::post_table_by_namespace,
      table::register_table,
      table::get_table,
      table::post_table,
      table::delete_table,
      table::head_table,
      table::rename_table,
      metric::post_metrics,
      config::get_config,
    ],
  )
}
