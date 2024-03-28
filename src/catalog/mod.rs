use std::collections::{HashMap, HashSet};
use rocket::State;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
  static ref CATALOG: Mutex<Catalog> = Mutex::new(Catalog::new());
}

#[derive(Clone)]
pub struct Catalog {
  namespace_map: HashMap<String, String>,
  table_map: HashMap<String, String>, // key = namespace_name + "_" + table_name
}


// pub struct Namespace {
//   name: String,
//   namespace_map: HashMap<String, Vec<String>>,
// }


impl Catalog {
  // pub fn new() -> Catalog {
  //   Catalog {}
  // }
  pub fn new() -> Catalog {
    Catalog {
        namespace_map: HashMap::new(), // Initialize the HashMap
        table_map: HashMap::new(),     // Initialize the HashMap
    }
  }

  pub fn post_namespace_func(&self, namespace_name: String) -> (bool, String) {
    // let catalog_instance = catalog.inner();
    // let mut catalog_instance = catalog.inner().namespace_map.clone();
    
    // let mut catalog_instance = catalog.inner().clone();

    let mut catalog_instance = CATALOG.lock().unwrap();
    
    if namespace_name.is_empty(){
      (false, "400 Bad Request".to_string())
    } else {
      if catalog_instance.namespace_map.contains_key(&namespace_name) {
        (false, "409 Conflict; Namespace already exists".to_string())
      } else {
        catalog_instance.namespace_map.insert(namespace_name.clone(), "Some table metadata".to_string()); // Insert namespace_name into map
        // let mut catalog_ref = catalog.inner();
        // catalog_ref.namespace_map = catalog_instance;
        (true, "200 OK".to_string())
      }
    }
  }


  pub fn post_table_func(&self, namespace_name: String, table_name: String) -> (bool, String) {
    let mut catalog_instance = CATALOG.lock().unwrap();
    if namespace_name.is_empty() || table_name.is_empty(){
      (false, "400 Bad Request".to_string())
    } else {
      if catalog_instance.namespace_map.contains_key(&namespace_name) {
        let table_key = format!("{}_{}", namespace_name, table_name); // namespace_name + "_" + table_name
        if catalog_instance.table_map.contains_key(&table_key){
          (false, "409 Conflict; Table already exists".to_string())
        } else {
          catalog_instance.table_map.insert(table_key.clone(), "Some value".to_string());
          (true, "200 OK".to_string())
        }
      } else {
        (false, "404 NotFound; Namespace not found".to_string())
      }
    }
  }

  pub fn delete_table_func(&self, namespace_name: String, table_name: String) -> (bool, String) {
    let mut catalog_instance = CATALOG.lock().unwrap();
    if namespace_name.is_empty() || table_name.is_empty(){
      (false, "400 Bad Request".to_string())
    } else {
      if catalog_instance.namespace_map.contains_key(&namespace_name) {
        let table_key = format!("{}_{}", namespace_name, table_name); // namespace_name + "_" + table_name
        if catalog_instance.table_map.contains_key(&table_key){ // then delete
          catalog_instance.table_map.remove(&table_key);
          (true, "204 NoContent; Success".to_string())
        } else {
          // catalog_instance.table_map.insert(table_key.clone(), "Some value".to_string());
          (false, "404 NotFound; Table not found".to_string())
        }
      } else {
        (false, "402 NotFound; Namespace not found".to_string())
      }
    }
  }


  pub fn head_table_func(&self, namespace_name: String, table_name: String) -> (bool, String) {
    let mut catalog_instance = CATALOG.lock().unwrap();
    if namespace_name.is_empty() || table_name.is_empty(){
      (false, "400 Bad Request".to_string())
    } else {
      if catalog_instance.namespace_map.contains_key(&namespace_name) {
        let table_key = format!("{}_{}", namespace_name, table_name); // namespace_name + "_" + table_name
        if catalog_instance.table_map.contains_key(&table_key){ // then delete
          (true, "204 NoContent; Success".to_string())
        } else {
          (false, "404 NotFound; Table not found".to_string())
        }
      } else {
        (false, "402 NotFound; Namespace not found".to_string())
      }
    }
  }


}





