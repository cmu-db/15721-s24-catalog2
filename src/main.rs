mod catalog;
mod common;
mod db;
mod server;
mod util;

#[macro_use]
extern crate rocket;

use common::result::EmptyResult;
use db::DB;

use server::routes::*;

use crate::common::result::{ErrorType, Location};

#[catch(404)]
fn general_not_found() -> EmptyResult {
  err!(
    ErrorType::NotFound,
    Location::DB,
    "Resource not found, please check the URL".to_string()
  )
}

#[launch]
fn rocket() -> _ {
  let db = DB::new();
  if db.is_err() {
    panic!("Failed to initialize database: {:?}", db.err());
  }

  rocket::build()
    .attach(namespace::stage())
    .manage(db.unwrap())
    .register("/", catchers![general_not_found])
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
