use crate::models::Item;

pub fn export_to_csv(items: &[Item], path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut wtr = csv::Writer::from_path(path)?;
    for item in items {
        wtr.serialize(item)?;
    }
    wtr.flush()?;
    Ok(())
}

pub fn export_to_csv_string(items: &[Item]) -> Result<String, String> {
    let mut wtr = csv::Writer::from_writer(vec![]);
    for item in items {
        wtr.serialize(item).map_err(|e| e.to_string())?;
    }
    wtr.flush().map_err(|e| e.to_string())?;

    let data = String::from_utf8(wtr.into_inner().map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;

    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Item, ItemAction, ItemCategory};
    use chrono::NaiveDate;
    use std::fs;
    use std::path::Path;

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

    #[test]
    fn test_export_to_csv_string() {
        let items = vec![sample_item()];

        let csv_output = export_to_csv_string(&items).expect("Export to string failed");

        // Check that the output contains expected content
        assert!(csv_output.contains("Test")); // Ensure item name is present
        assert!(csv_output.contains("desc")); // Ensure item description is present
        assert!(csv_output.contains("Book")); // Ensure item category is present
    }
}
