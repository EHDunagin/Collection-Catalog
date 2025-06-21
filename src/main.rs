use chrono::NaiveDate;
use collection_catalog::db::{add_item, init_db, get_all_items};
use collection_catalog::models::{Item, ItemAction, ItemCategory};
use rusqlite::Connection;
use anyhow::Result;

fn main() -> anyhow::Result<()> {
    let conn = Connection::open("catalog.db")?;
    init_db(&conn)?;
    println!("Database initialized!");

    let sample_item = Item {
        id: 0, // SQLite will assign this
        name: "Vintage Clock".to_string(),
        description: "An antique wall-mounted clock".to_string(),
        category: ItemCategory::Antique,
        action: ItemAction::Keep,
        date_added: chrono::Local::now().naive_local().date(),
        last_updated: chrono::Local::now().naive_local().date(),
        age_years: Some(70),
        date_acquired: Some(NaiveDate::from_ymd_opt(1980, 5, 15).unwrap()),
        purchase_price: Some(45.00),
        estimated_value: Some(120.00),
        creator: Some("Unknown".to_string()),
        working: Some(true),
        provenance: Some("Inherited from grandmother".to_string()),
        deleted: false,

    };

    add_item(&conn, &sample_item)?;
    println!("Item added!");
 
    let all_items = get_all_items(&conn)?;
    for item in all_items{
        println!("ID {}: {} ({}), Action: {:?}", item.id, item.name, item.category, item.action);
    }

    Ok(())
}
