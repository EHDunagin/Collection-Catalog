use collection_catalog::models::{Item, ItemCategory, ItemAction};
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
fn test_valid_item_passes() {

    let item = make_item();

    assert!(item.validate().is_ok());

}

#[test]
fn test_empty_name_fails() {

    let mut item = make_item();

    item.name = " ".to_string();

    assert!(item.validate().is_err());

}
 
#[test]
fn test_name_too_long_fails() {

    let mut item = make_item();

    item.name = "This is a really long name too long really. It shouldn't be so long.".to_string();

    assert!(item.validate().is_err());

}

#[test]
fn test_no_description_fails() {

    let mut item = make_item();

    item.description = "   ".to_string();

    assert!(item.validate().is_err());

}


#[test]
fn test_negative_price_fails() {

    let mut item = make_item();

    item.purchase_price = Some(-123.00);

    assert!(item.validate().is_err());

}
 #[test]
fn test_multiple_errors_fails() {

    let mut item = make_item();

    item.description = "   ".to_string();
    item.purchase_price = Some(-10.0);

    assert!(item.validate().is_err());

}      
