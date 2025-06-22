use collection_catalog::models::*;
use collection_catalog::db::*;
use rusqlite::Connection;
use chrono::NaiveDate;

fn make_item() -> Item {
    let item = Item {
        id: 1,
        name: "Valid Name".to_string(),
        description: "Nice item".to_string(),
        category: ItemCategory::Book,
        action: ItemAction::Keep,
        date_added: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        last_updated: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        age_years: Some(10),
        date_acquired: None,
        purchase_price: Some(100.0),
        estimated_value: Some(120.0),
        creator: None,
        working: Some(true),
        provenance: None,
        deleted: false,
    };

    item
}


#[test]
fn test_add_and_get_item() {
    let conn = Connection::open_in_memory().unwrap();
    init_db(&conn).unwrap(); // ensure table is created

    let item = make_item();
    add_item(&conn, &item).unwrap();
    let fetched = get_item_by_id(&conn, 1).unwrap().unwrap();
    assert_eq!(fetched.name, item.name);
}

#[test]
fn test_update_item() {
    let conn = Connection::open_in_memory().unwrap();
    init_db(&conn).unwrap(); // ensure table is created

    let mut item = make_item();
    add_item(&conn, &item).unwrap();

    item.name = "new name".to_string();
    update_item(&conn, &item);

    let fetched = get_item_by_id(&conn, 1).unwrap().unwrap();
    assert_eq!(fetched.name, "new name".to_string());
}     
