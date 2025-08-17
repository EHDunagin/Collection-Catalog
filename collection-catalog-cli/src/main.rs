use std::env;
use std::str::FromStr;

use collection_catalog_core::{
    init_db,
    add_item,
    Item,
    ItemCategory, 
    ItemAction,
    ItemFilter,
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
        println!("  list            - List all items");
        println!("  export <path>    - Export all items to CSV");
        println!("  add <name> <description> <category> <action> - Add a new item");
        return Ok(());
    }

    // Connect to the database (adjust the path to your actual DB)
    let conn = Connection::open("data/catalog.db")?;
    init_db(&conn)?; 

    match args[0].as_str() {
        "list" => {
            // Call a core function to list items
            println!("Listing items...");
            let items = get_filtered_items(&conn, ItemFilter::default())?;
            for item in items {
                println!("{:?}", item);
            }
        }
        "export" => {
            if args.len() < 2 {
                eprintln!("Error: export requires a file path argument");
                return Ok(());
            }
            let items = get_filtered_items(&conn, ItemFilter::default())?;
            export_to_csv(&items, &args[1])?;
            // TODO fetch items from core and export them
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
        _ => {
            eprintln!("unknown command: {}", args[0]);
        }
    }
    Ok(())

}
