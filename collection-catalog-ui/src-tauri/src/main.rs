// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::path::Path;

use collection_catalog_core::{init_db, get_all_items, Item};
use rusqlite::Connection;

#[tauri::command]
fn list_items() -> Result<Vec<Item>, String> {
    // Ensure data dir exists
    let data_dir = Path::new("data");
    if !data_dir.exists() {
        fs::create_dir_all(data_dir).map_err(|e| e.to_string())?;
    }

    let conn = Connection::open("data/catalog.db").map_err(|e| e.to_string())?;
    init_db(&conn).map_err(|e| e.to_string())?;
    
    // Return the items
    let items =  get_all_items(&conn).map_err(|e| e.to_string())?;
    Ok(items)
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to the Collection Catalog!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri applicateion");
}
