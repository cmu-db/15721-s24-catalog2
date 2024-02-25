
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::collections::{HashMap, HashSet};


struct Schema;

struct TableHeap;

struct TableInfo;

struct Database{
    next_table_oid: AtomicUsize,
    tables: Mutex<HashMap<usize, TableInfo>>, // table_oid to TableInfo
    table_names: Mutex<HashMap<String, usize>>, // table_name to table_oid
}



impl Database {
    fn create_table(&self, table_name: String, schema: Schema, create_table_heap: bool) -> *const TableInfo {
        if self.table_names.lock().unwrap().contains_key(&table_name) {
            return std::ptr::null(); // the table already exists
        }

        let table: Option<TableHeap>;
        if create_table_heap {
            table = Some(TableHeap);
        } else {
            table = None; // Not creating table heap
        }

        let table_oid = self.next_table_oid.fetch_add(1, Ordering::SeqCst); // get next available table OID

        let meta = TableInfo::new(schema, table_name.clone(), table, table_oid);
        let tmp = Box::into_raw(Box::new(meta)); // tmp is a pointer (*const TableInfo)

        self.tables.lock().unwrap().insert(table_oid, unsafe { *Box::from_raw(tmp) });
        self.table_names.lock().unwrap().insert(table_name, table_oid);

        tmp // this points to the TableInfo object
    }


    fn get_table_by_name(&self, table_name: String) -> *const TableInfo {
        let tables_lock = self.tables.lock().unwrap();
        if let Some(meta) = tables_lock.get(&table_oid) {
            return meta as *const TableInfo;
        }
        NULL_TABLE_INFO
    }


    fn get_table_by_id(&self, table_name: usize) -> *const TableInfo {
        let tables_lock = self.tables.lock().unwrap();
        if let Some(meta) = tables_lock.get(&table_oid) {
            return meta as *const TableInfo;
        }
        NULL_TABLE_INFO
    }


    fn delete_table(&self, table_oid: usize) -> Option<TableInfo> {
        let mut tables_lock = self.tables.lock().unwrap();
        let mut table_names_lock = self.table_names.lock().unwrap();

        if let Some(table_info) = tables_lock.remove(&table_oid) {
            
            let table_name = table_info.table_name.clone();
            table_names_lock.remove(&table_name); // Remove table name from table_names map

            Some(table_info)
        } else {
            None
        }
    }



    fn rename_table(&self, table_oid: usize, new_table_name: String) -> Option<()> {
        let mut tables_lock = self.tables.lock().unwrap();
        let mut table_names_lock = self.table_names.lock().unwrap();

        if let Some(mut table_info) = tables_lock.get_mut(&table_oid) { // if the table_oid is found
            let old_table_name = table_info.table_name.clone();
            table_info.table_name = new_table_name.clone();
            
            table_names_lock.remove(&old_table_name); // Update the table name in the table_names map
            table_names_lock.insert(new_table_name, table_oid);
            
            Some(())
        } else {
            None
        }
    }


    fn list_tables(&self) -> Vec<&str> {
        let table_names_lock = self.table_names.lock().unwrap();

        table_names_lock.keys().map(|name| name.as_str()).collect()
    }

}

