use chrono::NaiveDate;
use collection_catalog::db::{add_item, init_db, get_all_items, soft_delete_item, get_item_by_id, update_item, get_filtered_items };
use collection_catalog::models::{Item, ItemAction, ItemCategory, ItemFilter };
use rusqlite::Connection;

fn main() -> anyhow::Result<()> {
    let conn = Connection::open("catalog.db")?;
    init_db(&conn)?;
    println!("Database initialized!");

    let sample_item = Item {
        id: 0, // SQLite will assign this
        name: "Different thing".to_string(),
        description: "An herilook ceramic bowl new thing because we have to many bowls".to_string(),
        category: ItemCategory::Antique,
        action: ItemAction::Keep,
        date_added: chrono::Local::now().naive_local().date(),
        last_updated: chrono::Local::now().naive_local().date(),
        age_years: Some(80),
        date_acquired: Some(NaiveDate::from_ymd_opt(2005, 5, 15).unwrap()),
        purchase_price: Some(45.00),
        estimated_value: Some(120.00),
        creator: Some("Unknown".to_string()),
        working: Some(true),
        provenance: Some("Inherited from grandmother".to_string()),
        deleted: false,

    };

    match add_item(&conn, &sample_item) {
        Ok(()) => println!("Item added!"),
        Err(e) => eprintln!("Failed to add item: {}", e),
    }
 
    let all_items = get_all_items(&conn)?;
    for item in all_items{
        println!("ID {}: {} ({}), Action: {:?} Deleted status: {}", item.id, item.name, item.category, item.action, item.deleted);
    }

    soft_delete_item(&conn, 3)?;
    println!("Item with id 3 has been soft deleted.");

    if let Some(mut item) = get_item_by_id(&conn, 1)? {
        item.name = "Updated Clock Name:".to_string();
        item.last_updated = chrono::Local::now().naive_local().date();

        match update_item(&conn, &item) {
            Ok(()) => println!("Item updated."),
            Err(e) => eprintln!("Update failed: {}", e),
        }

    } else {
        println!("Item with ID 1 not found.");
    }

    let filter = ItemFilter {
        name_contains: Some(String::from("Different")),
        category: Some(ItemCategory::Antique),
        action: Some(ItemAction::Keep),
        working: Some(true),
        deleted: Some(false),
        date_acquired_min: Some(NaiveDate::from_ymd_opt(1980, 1, 1).unwrap()),
        age_years_min: Some(3),
        estimated_value_min: Some(100.25),
        ..Default::default()
    };

    match get_filtered_items(&conn, filter) {
        Ok(items) => {
            println!("Filtered items function ran successfully!");
            println!("{:?}", items);
        },
        Err(e) => eprintln!("Filtering failed: {}", e),
    }

    Ok(())

}

