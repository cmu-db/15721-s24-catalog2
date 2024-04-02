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
pub fn rocket() -> _ {
  let cli = cli::parse();
  let db = DB::new(cli.db_root.unwrap());
  if db.is_err() {
    panic!("Failed to initialize database: {:?}", db.err());
  }

  rocket::build()
    .manage(db.unwrap())
    .attach(namespace::stage())
    .attach(catches::stage())
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

#[cfg(test)]
mod test {
  use rocket::local::asynchronous::Client;

  #[rocket::async_test]
  async fn test_create_server() {
    let client = Client::tracked(crate::rocket()).await;
    assert_eq!(client.is_ok(), true);
  }
}
