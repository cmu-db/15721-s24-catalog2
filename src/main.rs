mod catalog;
mod common;
mod db;

#[macro_use]
extern crate rocket;

mod server;
use db::DB;
use server::routes::*;

#[launch]
fn rocket() -> _ {
  let db = DB::new();
  if db.is_err() {
    panic!("Failed to initialize database: {:?}", db.err());
  }

  rocket::build()
    .attach(namespace::stage())
    .manage(db.unwrap())
    .mount(
      "/v1",
      routes![
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
