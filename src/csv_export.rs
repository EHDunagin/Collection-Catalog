// use std::error::Error;
// use std::fs::File;

// use csv::Writer;
// use serde::Serialize;
use crate::models::Item;

pub fn export_to_csv(items: &[Item], path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut wtr = csv::Writer::from_path(path)?;
    for item in items {
        wtr.serialize(item)?;
    }
    wtr.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Item, ItemCategory, ItemAction};
    use std::fs;
    use std::path::Path;
    use chrono::NaiveDate;

    fn sample_item() -> Item {
        Item {
            id: 1,
            name: "Test".to_string(),
            description: "desc".to_string(),
            category: ItemCategory::Book,
            date_added: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            action: ItemAction::Keep, 
            // fill out all required fields...
            ..Default::default()
        }
    }

    #[test]
    fn test_export_to_csv() {
        let path = "test_output.csv";
        let items = vec![sample_item()];

        export_to_csv(&items, path).expect("Export failed");
        assert!(Path::new(path).exists());

        let content = fs::read_to_string(path).unwrap();
        assert!(content.contains("Test")); // Ensure data is written

        fs::remove_file(path).unwrap(); // Clean up
    }
}
