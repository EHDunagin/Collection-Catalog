use chrono::NaiveDate;
use rusqlite::Row;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum ItemAction {
    #[default]
    Keep,
    Sell,
}

impl fmt::Display for ItemAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ItemAction::Keep => "Keep",
                ItemAction::Sell => "Sell",
            }
        )
    }
}

impl FromStr for ItemAction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Keep" => Ok(ItemAction::Keep),
            "Sell" => Ok(ItemAction::Sell),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum ItemCategory {
    #[default]
    Antique,
    Book,
    Decor,
    ElectronicDevice,
    Furniture,
    HouseholdItem,
    Kitchenware,
    MineralSpecimen,
    Tool,
    Wood,
    Other,
}

impl fmt::Display for ItemCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            ItemCategory::Antique => "Antique",
            ItemCategory::Book => "Book",
            ItemCategory::Decor => "Decor",
            ItemCategory::ElectronicDevice => "ElectronicDevice",
            ItemCategory::Furniture => "Furniture",
            ItemCategory::HouseholdItem => "HouseholdItem",
            ItemCategory::Kitchenware => "Kitchenware",
            ItemCategory::MineralSpecimen => "MineralSpecimen",
            ItemCategory::Tool => "Tool",
            ItemCategory::Wood => "Wood",
            ItemCategory::Other => "Other",
        };
        write!(f, "{}", name)
    }
}

impl FromStr for ItemCategory {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Antique" => Ok(ItemCategory::Antique),
            "Book" => Ok(ItemCategory::Book),
            "Decor" => Ok(ItemCategory::Decor),
            "ElectronicDevice" => Ok(ItemCategory::ElectronicDevice),
            "Furniture" => Ok(ItemCategory::Furniture),
            "HouseholdItem" => Ok(ItemCategory::HouseholdItem),
            "Kitchenware" => Ok(ItemCategory::Kitchenware),
            "MineralSpecimen" => Ok(ItemCategory::MineralSpecimen),
            "Tool" => Ok(ItemCategory::Tool),
            "Wood" => Ok(ItemCategory::Wood),
            "Other" => Ok(ItemCategory::Other),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub category: ItemCategory,
    pub action: ItemAction,
    #[serde(default)]
    pub date_added: NaiveDate,
    #[serde(default)]
    pub last_updated: NaiveDate,
    pub deleted: bool,

    // Optional fields
    pub age_years: Option<u32>,
    pub date_acquired: Option<NaiveDate>,
    pub purchase_price: Option<f64>,
    pub estimated_value: Option<f64>,
    pub creator: Option<String>,
    pub working: Option<bool>,
    pub provenance: Option<String>,
}

impl Item {
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = vec![];

        // Required: name
        if self.name.trim().is_empty() {
            errors.push("Name cannot be empty.".to_string());
        } else if self.name.len() > 50 {
            errors.push("Name cannot be more than 50 characters.".to_string());
        }

        // Required: description
        if self.description.trim().is_empty() {
            errors.push("Description cannot be empty.".to_string());
        }

        // Optional: price & value must be non-negative
        if let Some(price) = self.purchase_price {
            if price < 0.0 {
                errors.push("Purchase price cannot be negative.".to_string());
            }
        }

        if let Some(value) = self.estimated_value {
            if value < 0.0 {
                errors.push("Estimated value cannot be negative.".to_string());
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn from_row(row: &Row) -> rusqlite::Result<Self> {
        Ok(Item {
            id: row.get("id")?,
            name: row.get("name")?,
            description: row.get("description")?,
            category: ItemCategory::from_str(&row.get::<_, String>("category")?)
                .unwrap_or(ItemCategory::Other),
            action: ItemAction::from_str(&row.get::<_, String>("action")?)
                .unwrap_or(ItemAction::Keep),
            // Default dates don't matter because this should always exist
            date_added: NaiveDate::parse_from_str(&row.get::<_, String>("date_added")?, "%Y-%m-%d")
                .unwrap_or(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap()),
            last_updated: NaiveDate::parse_from_str(
                &row.get::<_, String>("last_updated")?,
                "%Y-%m-%d",
            )
            .unwrap_or(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap()),
            deleted: row.get("deleted")?,

            // Optional fields
            age_years: row.get("age_years")?,
            date_acquired: row
                .get::<_, Option<String>>("date_acquired")?
                .map(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").unwrap()),
            purchase_price: row.get("purchase_price")?,
            estimated_value: row.get("estimated_value")?,
            creator: row.get("creator")?,
            working: row.get("working")?,
            provenance: row.get("provenance")?,
        })
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ItemFilter {
    // Partial string matches
    pub name_contains: Option<String>,
    pub description_contains: Option<String>,
    pub creator_contains: Option<String>,
    pub provenance_contains: Option<String>,

    // Enums / Exact matches
    pub category: Option<ItemCategory>,
    pub action: Option<ItemAction>,
    pub working: Option<bool>,
    pub deleted: Option<bool>,

    // Date filters
    pub date_added_min: Option<NaiveDate>,
    pub date_added_max: Option<NaiveDate>,
    pub last_updated_min: Option<NaiveDate>,
    pub last_updated_max: Option<NaiveDate>,
    pub date_acquired_min: Option<NaiveDate>,
    pub date_acquired_max: Option<NaiveDate>,

    // Numeric filers
    pub age_years_min: Option<u32>,
    pub age_years_max: Option<u32>,
    pub purchase_price_min: Option<f64>,
    pub purchase_price_max: Option<f64>,
    pub estimated_value_min: Option<f64>,
    pub estimated_value_max: Option<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_item_category_display_and_parse() {
        let cat = ItemCategory::Book;
        assert_eq!(cat.to_string(), "Book");
        let parsed = ItemCategory::from_str("Book").unwrap();
        assert!(matches!(parsed, ItemCategory::Book));
    }

    #[test]
    fn test_item_action_display_and_parse() {
        let action = ItemAction::Sell;
        assert_eq!(action.to_string(), "Sell");
        let parsed = ItemAction::from_str("Sell").unwrap();
        assert!(matches!(parsed, ItemAction::Sell));
    }

    #[test]
    fn test_invalid_category_parse() {
        assert!(ItemCategory::from_str("NotARealCategory").is_err());
    }
}
