use collection_catalog_core::{
    init_db,
    ItemFilter,
    get_filtered_items,
    export_to_csv
};
use rusqlite::Connection;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Grab arguments (skip arg[0] which is the program name)
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        println!("Collection Catalog CLI");
        println!("Usage:");
        println!("  list            - List all items");
        println!(" export <path>    - Export all items to CSV");
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

        _ => {
            eprintln!("unknown command: {}", args[0]);
        }
    }
    Ok(())

}
