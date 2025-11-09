use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::str::FromStr;

use collection_catalog_core::{
    Item, ItemAction, ItemCategory, ItemFilter, add_item, export_to_csv, get_filtered_items,
    init_db, soft_delete_item, update_item_fields,
};
use rusqlite::Connection;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Grab arguments (skip arg[0] which is the program name)
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        println!("Collection Catalog CLI");
        println!("Usage:");
        println!("  list field=value [field=value...]               - List all items");
        println!("  export <path> field=value [field=value...]      - Export all items to CSV");
        println!("  add <name> <description> <category> <action>    - Add a new item");
        println!("  delete <id>                                     - Export all items to CSV");
        println!("  update <item_id> field=value [field=value...]   - Update an existing item");
        println!("  help                                            - Show help message");
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
            let mut filter = ItemFilter::default();

            // Parse optional filters: field=value
            for arg in args.iter().skip(1) {
                if let Some((field, value)) = arg.split_once('=') {
                    match field {
                        // Partial string match filters
                        "name" => filter.name_contains = Some(value.to_string()),
                        "description" => filter.description_contains = Some(value.to_string()),
                        "creator" => filter.creator_contains = Some(value.to_string()),
                        "provenance" => filter.provenance_contains = Some(value.to_string()),
                        // Enums / Exact match filters
                        "category" => filter.category = ItemCategory::from_str(value).ok(),
                        "action" => filter.action = ItemAction::from_str(value).ok(),
                        "working" => filter.working = value.parse::<bool>().ok(),
                        "deleted" => filter.deleted = value.parse::<bool>().ok(),
                        // Date filters
                        "date_added_min" => {
                            filter.date_added_min = chrono::NaiveDate::from_str(value).ok()
                        }
                        "date_added_max" => {
                            filter.date_added_max = chrono::NaiveDate::from_str(value).ok()
                        }
                        "last_updated_min" => {
                            filter.last_updated_min = chrono::NaiveDate::from_str(value).ok()
                        }
                        "last_updated_max" => {
                            filter.last_updated_max = chrono::NaiveDate::from_str(value).ok()
                        }
                        "date_acquired_min" => {
                            filter.date_acquired_min = chrono::NaiveDate::from_str(value).ok()
                        }
                        "date_acquired_max" => {
                            filter.date_acquired_max = chrono::NaiveDate::from_str(value).ok()
                        }
                        // Number filters
                        "age_years_min" => filter.age_years_min = value.parse::<u32>().ok(),
                        "age_years_max" => filter.age_years_max = value.parse::<u32>().ok(),
                        "purchase_price_min" => {
                            filter.purchase_price_min = value.parse::<f64>().ok()
                        }
                        "purchase_price_max" => {
                            filter.purchase_price_max = value.parse::<f64>().ok()
                        }
                        "estimated_value_min" => {
                            filter.estimated_value_min = value.parse::<f64>().ok()
                        }
                        "estimated_value_max" => {
                            filter.estimated_value_max = value.parse::<f64>().ok()
                        }
                        // Catchall
                        _ => eprintln!("Warning: unknown filter field'{}'", field),
                    }
                }
            }

            // Call core function to list items
            let items = get_filtered_items(&conn, filter)?;
            println!("Listing {} items...", items.len());
            for item in items {
                println!("{:?}", item);
            }
        }
        "export" => {
            if args.len() < 2 {
                eprintln!("Error: export requires a file path argument");
                return Ok(());
            }
            let path = &args[1];
            let mut filter = ItemFilter::default();

            // Parse optional filters after path
            for arg in args.iter().skip(2) {
                if let Some((field, value)) = arg.split_once('=') {
                    match field {
                        // Partial string match filters
                        "name" => filter.name_contains = Some(value.to_string()),
                        "description" => filter.description_contains = Some(value.to_string()),
                        "creator" => filter.creator_contains = Some(value.to_string()),
                        "provenance" => filter.provenance_contains = Some(value.to_string()),
                        // Enums / Exact match filters
                        "category" => filter.category = ItemCategory::from_str(value).ok(),
                        "action" => filter.action = ItemAction::from_str(value).ok(),
                        "working" => filter.working = value.parse::<bool>().ok(),
                        "deleted" => filter.deleted = value.parse::<bool>().ok(),
                        // Date filters
                        "date_added_min" => {
                            filter.date_added_min = chrono::NaiveDate::from_str(value).ok()
                        }
                        "date_added_max" => {
                            filter.date_added_max = chrono::NaiveDate::from_str(value).ok()
                        }
                        "last_updated_min" => {
                            filter.last_updated_min = chrono::NaiveDate::from_str(value).ok()
                        }
                        "last_updated_max" => {
                            filter.last_updated_max = chrono::NaiveDate::from_str(value).ok()
                        }
                        "date_acquired_min" => {
                            filter.date_acquired_min = chrono::NaiveDate::from_str(value).ok()
                        }
                        "date_acquired_max" => {
                            filter.date_acquired_max = chrono::NaiveDate::from_str(value).ok()
                        }
                        // Number filters
                        "age_years_min" => filter.age_years_min = value.parse::<u32>().ok(),
                        "age_years_max" => filter.age_years_max = value.parse::<u32>().ok(),
                        "purchase_price_min" => {
                            filter.purchase_price_min = value.parse::<f64>().ok()
                        }
                        "purchase_price_max" => {
                            filter.purchase_price_max = value.parse::<f64>().ok()
                        }
                        "estimated_value_min" => {
                            filter.estimated_value_min = value.parse::<f64>().ok()
                        }
                        "estimated_value_max" => {
                            filter.estimated_value_max = value.parse::<f64>().ok()
                        }
                        // Catchall
                        _ => eprintln!("Warning: unknown filter field'{}'", field),
                    }
                }
            }
            let items = get_filtered_items(&conn, filter)?;
            export_to_csv(&items, path)?;
            println!("Exported {} items to {}", items.len(), path);
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
        "help" => {
            println!("Collection Catalog CLI");
            println!("Usage:");
            println!("  list field=value [field=value...]               - List all items");
            println!("  export <path> field=value [field=value...]      - Export all items to CSV");
            println!("  add <name> <description> <category> <action>    - Add a new item");
            println!("  delete <id>                                     - Export all items to CSV");
            println!("  update <item_id> field=value [field=value...]   - Update an existing item");
            println!("  help                                            - Show this help message");

            println!("\nFilterable fields for list/export:");
            println!("  name, description, creator, provenance (partial match)");
            println!(
                "  category (Exact: Book, Artwork, Collectible, Document, Electronic, Furniture, Jewelry, Other)"
            );
            println!("  action (Exact: Keep, Sell)");
            println!("  working (true/false)");
            println!("  deleted (true/false)");
            println!("  date_added_min, date_added_max (YYYY-MM-DD)");
            println!("  last_updated_min, last_updated_max (YYYY-MM-DD)");
            println!("  date_acquired_min, date_acquired_max (YYYY-MM-DD)");
            println!("  age_years_min, age_years_max (integer)");
            println!("  purchase_price_min, purchase_price_max (float)");
            println!("  estimated_value_min, estimated_value_max (float)");
        }
        _ => {
            eprintln!("unknown command: {}", args[0]);
        }
    }

    Ok(())
}
