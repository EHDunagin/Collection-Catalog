// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::path::Path;
use std::sync::Mutex;

use collection_catalog_core::{ init_db, get_all_items, add_item, Item };
use rusqlite::Connection;
use tauri::State;

// Shared state wrapper
struct DbState(Mutex<Connection>);


#[tauri::command]
fn list_items(db: State<DbState>) -> Result<Vec<Item>, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    Ok(get_all_items(&*conn).map_err(|e| e.to_string())?)
}

// TODO figure out how to use this in such a way as to take info from the frontend, create an Item,
// and pass it into the function
#[tauri::command]
fn new_item(db: State<DbState>, item: Item) -> Result<(), String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    item.validate().map_err(|errs| errs.join(", "))?;
    Ok(add_item(&conn, &item).map_err(|e| e.to_string())?)
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
        .invoke_handler(tauri::generate_handler![list_items, new_item])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
