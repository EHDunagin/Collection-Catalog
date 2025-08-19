use std::env;
use std::str::FromStr;
use std::fs;
use std::path::Path;
use std::collections::HashMap;

use collection_catalog_core::{
    init_db,
    add_item,
    soft_delete_item,
    update_item_fields,
    Item,
    ItemCategory, 
    ItemAction,
    ItemFilter,
    get_all_items,
    get_filtered_items,
    export_to_csv
};
use rusqlite::Connection;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Grab arguments (skip arg[0] which is the program name)
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        println!("Collection Catalog CLI");
        println!("Usage:");
        println!("  list                                         - List all items");
        println!("  export <path>                                - Export all items to CSV");
        println!("  add <name> <description> <category> <action> - Add a new item");
        println!("  delete <id>                                  - Export all items to CSV");
        return Ok(());
    }

    // Ensure data folder exists
    let path = Path::new("data");
    if !path.exists() {
        fs::create_dir_all(path)?;
    }

    // Connect to the database (adjust the path to your actual DB)
    let conn = Connection::open("data/catalog.db")?;
    init_db(&conn)?; 

    match args[0].as_str() {
        "list" => {
            // Call a core function to list items
            println!("Listing items...");
            let items = get_all_items(&conn)?;
            for item in items {
                println!("{:?}", item);
            }
        }
        "export" => {
            if args.len() < 2 {
                eprintln!("Error: export requires a file path argument");
                return Ok(());
            }
            let items = get_all_items(&conn)?;
            export_to_csv(&items, &args[1])?;
            // Fetch items from core and export them
            println!("Exported {} items to {}", items.len(), &args[1]);
        }

        "add" => {
            if args.len() < 5 {
                eprintln!("Error: add requires <name> <description> <category> <action>");
                return Ok(());
            }
            
            let name = args[1].to_string();
            let description = args[2].to_string();

            let category = match ItemCategory::from_str(&args[3]) {
                Ok(c) => c,
                Err(_) => {
                    eprintln!("Invalid category: {}", args[3]);
                    return Ok(());
                }
            };

            let action = match ItemAction::from_str(&args[4]) {
                Ok(a) => a,
                Err(_) => {
                    eprintln!("Invalid action: {}", args[4]);
                    return Ok(());
                }
            };

            let today = chrono::Local::now().date_naive();

            let item = Item {
                id: 0, // DB will auto-assing
                name: name,
                description: description,
                category: category,
                action: action,
                date_added: today,
                last_updated: today,
                deleted: false,

                // Default remaining
                ..Default::default()
            };

            add_item(&conn, &item)?;
            println!("Added item: {}", item.name);
        }
        "delete" => {
            if args.len() < 2 {
                eprintln!("Usage: delete <item_id>");
                return Ok(());
            }

            let id_str = &args[1];
            let item_id: i32 = match id_str.parse() {
                Ok(id) => id,
                Err(_) => {
                    eprintln!("Error: item_id must be an integer, got '{}'", id_str);
                    return Ok(());
                }
            };

            println!("Deleting item with ID {}...", item_id);
            soft_delete_item(&conn, item_id)?;
            println!("Item {} marked as deleted.", item_id);
        }
         "update" => {
            if args.len() < 3 {
                    eprintln!("Usage: update <item_id> field=value [field=value ...]");
                return Ok(());
            }

            // Parse item_id
            let item_id: i32 = match args[1].parse() {
                Ok(id) => id,
                Err(_) => {
                    eprintln!("Error: item_id must be an integer, got '{}'", args[1]);
                    return Ok(());
                }
            };

            // Collect updates into a HashMap<&str, String>
            let mut updates = HashMap::new();
            for update in &args[2..] {
                if let Some((field, value)) = update.split_once('=') {
                    updates.insert(field.trim(), value.trim().to_string());
                } else {
                    eprintln!("Invalid update format: {update}. Use field=value");
                    return Ok(());
                }
            }

            // Apply update
            match update_item_fields(&conn, item_id, updates) {
                Ok(_) => println!("Item {item_id} updated successfully."),
                Err(e) => eprintln!("Failed to update item {item_id}: {e}"),
            }

    }     
    _ => {
            eprintln!("unknown command: {}", args[0]);
        }
    }


    Ok(())

}
