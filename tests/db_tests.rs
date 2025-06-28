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
    let _ = update_item(&conn, &item);

    let fetched = get_item_by_id(&conn, 1).unwrap().unwrap();
    assert_eq!(fetched.name, "new name".to_string());
}     

#[test]
fn test_soft_delete() {
    let conn = Connection::open_in_memory().unwrap();
    init_db(&conn).unwrap(); // ensure table is created

    let mut item = make_item();
    add_item(&conn, &item).unwrap();

    item.id = 2;
    item.name = "item 2".to_string();

    add_item(&conn, &item).unwrap();

    item.id = 3;
    item.name = "item 3".to_string();

    add_item(&conn, &item).unwrap();

    let _ = soft_delete_item(&conn, 2);

    let all_items = get_all_items(&conn).unwrap();
    assert_eq!(all_items.len(), 2);

    let ids: Vec<i32> = all_items.iter().map(|item| item.id).collect();
    assert!(ids.contains(&1), "Missing item with id 1");
    assert!(!ids.contains(&2), "Contains deleted item with id 2");
    assert!(ids.contains(&3), "Missing item with id 3");

}





