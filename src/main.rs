mod catalog;
mod cli;
mod common;
mod db;
mod server;
mod util;

#[macro_use]
extern crate rocket;

use db::DB;

use server::{catches, routes::*};

#[launch]
fn rocket() -> _ {
  let cli = cli::parse();

  let db = DB::new(cli.db_root.unwrap());
  if db.is_err() {
    panic!("Failed to initialize database: {:?}", db.err());
  }

  rocket::build()
    .attach(namespace::stage())
    .attach(catches::stage())
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
