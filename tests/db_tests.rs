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

#[test]
fn test_filter_by_name_contains() {
    let conn = Connection::open_in_memory().unwrap();
    init_db(&conn).unwrap();

    let item = make_item();
    add_item(&conn, &item).unwrap();

    let mut filter = ItemFilter::default();
    filter.name_contains = Some("lid".to_string());

    let results = get_filtered_items(&conn, filter).unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "Valid Name");

}

#[test]
fn test_filter_by_category() {
    let conn = Connection::open_in_memory().unwrap();
    init_db(&conn).unwrap();

    let item = make_item();
    add_item(&conn, &item).unwrap();

    let mut filter = ItemFilter::default();
    filter.category = Some(ItemCategory::Book);

    let results = get_filtered_items(&conn, filter).unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "Valid Name");

}

#[test]
fn test_filter_by_date_added_min() {
    let conn = Connection::open_in_memory().unwrap();
    init_db(&conn).unwrap();

    let item = make_item();
    add_item(&conn, &item).unwrap();

    let mut filter = ItemFilter::default();
    filter.date_added_min = Some(NaiveDate::from_ymd_opt(2021, 1, 1).unwrap());

    let results = get_filtered_items(&conn, filter).unwrap();
    assert_eq!(results.len(), 1);
}

#[test]
fn test_filter_by_date_added_min_not_in_range() {
    let conn = Connection::open_in_memory().unwrap();
    init_db(&conn).unwrap();

    let item = make_item();
    add_item(&conn, &item).unwrap();

    let mut filter = ItemFilter::default();
    filter.date_added_min = Some(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap());

    let results = get_filtered_items(&conn, filter).unwrap();
    assert_eq!(results.len(), 0);
}    

#[test]
fn test_filter_by_multiple_fields() {
    let conn = Connection::open_in_memory().unwrap();
    init_db(&conn).unwrap();

    let item = make_item();
    add_item(&conn, &item).unwrap();

    let mut filter = ItemFilter::default();
    filter.name_contains = Some("Valid".to_string());
    filter.description_contains = Some("Nice".to_string());
    filter.category = Some(ItemCategory::Book);
    filter.date_added_min = Some(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap());
    filter.working = Some(true);
    filter.purchase_price_min = Some(50.0);
    filter.purchase_price_max = Some(150.0);

    let results = get_filtered_items(&conn, filter).unwrap();
    assert_eq!(results.len(), 1);
}

#[test]
fn test_filter_by_multiple_fields_missing_one() {
    let conn = Connection::open_in_memory().unwrap();
    init_db(&conn).unwrap();

    let item = make_item();
    add_item(&conn, &item).unwrap();

    let mut filter = ItemFilter::default();
    filter.name_contains = Some("Valid".to_string());
    filter.description_contains = Some("Nice".to_string());
    filter.category = Some(ItemCategory::Book);
    filter.date_added_min = Some(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap());
    // This one should exclude the item
    filter.working = Some(false);
    filter.purchase_price_min = Some(50.0);
    filter.purchase_price_max = Some(150.0);

    let results = get_filtered_items(&conn, filter).unwrap();
    assert_eq!(results.len(), 0);
}

#[test]
fn test_update_item_fields_success() {
    let conn = Connection::open_in_memory().unwrap();
    init_db(&conn).unwrap();

    let item = make_item();
    let added_item = add_item(&conn, &item).unwrap();

    // Prepare updates
    let mut updates = HashMap::new();
    updates.insert("name", "Updated Name".to_string());
    updates.insert("description", "Updated Description".to_string());
    updates.insert("category", "Book".to_string());
    updates.insert("working", "false".to_string());

    // Call function
    update_item_fields(&conn, &added_item.id, updates).unwrap();

    let updated = get_item_by_id(&conn, added_item.id).unwrap().unwrap();

    // Check changes
    assert_eq!(updated.name, "Updated Name");
    assert_eq!(updated.description, "Updated Description");
    assert_eq!(updated.category, ItemCategory::Book);
    assert_eq!(updated.working, Some(false));

    // Verify last_updated changed
    assert_ne!(updated.last_updated, added_item.last_updated);
}

#[test]
fn test_update_item_fields_invalid_field() {
    let conn = Connection::open_in_memory().unwrap();
    init_db(&conn).unwrap();

    let item = make_item();
    let added_item = add_item(&conn, &item).unwrap();

    // Try an invalid update
    let mut updated = HashMap::new();
    updates.insert("not_a_field", "oops".to_string());

    let result = update_item_fields(&conn, item.id, updates);

    assert!(result.is_err());
    let err_msg = format!("{}", result.unwrap_err());
    assert!(err_msg.contains("Unknown field"));
}


