// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Mutex;

use collection_catalog_core::{
    add_item, export_to_csv, get_all_items, get_filtered_items, get_item_by_id, init_db,
    soft_delete_item, update_item_fields, Item, ItemFilter,
};
use rusqlite::Connection;
use tauri::{ AppHandle, State} ;
use tauri_plugin_dialog::DialogExt;

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
fn update_item(
    db: State<DbState>,
    id: i32,
    updates: HashMap<String, String>,
) -> Result<(), String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;

    let owned: HashMap<String, String> = updates;
    let borrowed: HashMap<&str, String> =
        owned.iter().map(|(k, v)| (k.as_str(), v.clone())).collect();
    update_item_fields(&conn, id, borrowed).map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_item(db: State<DbState>, id: i32) -> Result<(), String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    soft_delete_item(&*conn, id).map_err(|e| e.to_string())
}

#[tauri::command]
async fn export_filtered_items_to_csv(
    db: State<'_, DbState>, 
    app_handle: AppHandle,
    filter: ItemFilter
) -> Result<Option<String>, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let items = get_filtered_items(&*conn, filter).map_err(|e| e.to_string())?;

    // Show "Save As" dialog
    let save_path = app_handle
        .dialog()
        .file()
        .set_title("Save CSV")
        .add_filter("CSV file", &["csv"])
        .blocking_save_file();

    // Convert the file path to a string and pass to export_to_csv
    if let Some(path) = save_path {
        export_to_csv(&items, &path.to_string())
            .map_err(|e| e.to_string())?;
        Ok(Some(path.to_string()))
    } else {
        // user cancelled
        Ok(None)
    }
}

fn main() {
    // Ensure data dir exists
    let data_dir = Path::new("../../data");
    if !data_dir.exists() {
        fs::create_dir_all(data_dir)
            .expect("Failed to create database and no existing database found");
    }

    // Initialize DB connection
    let conn = Connection::open("../../data/catalog.db").expect("failed to open db");
    init_db(&conn).expect("failed to init db");

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(DbState(Mutex::new(conn))) // add DB to app state
        .invoke_handler(tauri::generate_handler![
            list_items,
            new_item,
            filter_items,
            get_item,
            update_item,
            delete_item,
            export_filtered_items_to_csv
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
