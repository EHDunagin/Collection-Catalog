// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::path::Path;
use std::sync::Mutex;
use std::collections::HashMap;

use collection_catalog_core::{ init_db, get_all_items, get_filtered_items, get_item_by_id, add_item, update_item_fields, soft_delete_item, export_to_csv, Item, ItemFilter };
use rusqlite::Connection;
use tauri::State;

// Shared state wrapper
struct DbState(Mutex<Connection>);


#[tauri::command]
fn list_items(db: State<DbState>) -> Result<Vec<Item>, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    Ok(get_all_items(&*conn).map_err(|e| e.to_string())?)
}

#[tauri::command]
fn new_item(db: State<DbState>, item: Item) -> Result<(), String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    item.validate().map_err(|errs| errs.join(", "))?;
    Ok(add_item(&conn, &item).map_err(|e| e.to_string())?)
}

#[tauri::command]
fn filter_items(db: State<DbState>, filter: ItemFilter) -> Result<Vec<Item>, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    Ok(get_filtered_items(&*conn, filter).map_err(|e| e.to_string())?)
}

#[tauri::command]
fn get_item(db: State<DbState>, id: i32) -> Result<Option<Item>, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    get_item_by_id(&*conn, id).map_err(|e| e.to_string())
}

#[tauri::command]
fn update_item(db: State<DbState>, id: i32, updates: HashMap<String, String>) -> Result<(), String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;

    let owned: HashMap<String, String> = updates;
    let borrowed: HashMap<&str, String> = 
        owned.iter().map(|(k, v)| (k.as_str(), v.clone())).collect();
    update_item_fields(&conn, id, borrowed)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_item(db: State<DbState>, id: i32) -> Result<(), String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    soft_delete_item(&*conn, id).map_err(|e| e.to_string())
}



fn main() {
    // Ensure data dir exists
    let data_dir = Path::new("../../data");
    if !data_dir.exists() {
        fs::create_dir_all(data_dir).expect("Failed to create database and no existing database found");
    }

    // Initialize DB connection
    let conn = Connection::open("../../data/catalog.db").expect("failed to open db");
    init_db(&conn).expect("failed to init db");

    tauri::Builder::default()
        .manage(DbState(Mutex::new(conn))) // add DB to app state
        .invoke_handler(tauri::generate_handler![
            list_items, 
            new_item,
            filter_items,
            get_item,
            update_item,
            delete_item

        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
