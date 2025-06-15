use chrono::NaiveDate;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum ItemAction {
    Keep,
    Sell,
}

impl fmt::Display for ItemAction {
    fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            ItemAction::Keep => "Keep",
            ItemAction::Sell => "Sell",
        })
    }
}

impl FromStr for ItemAction{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Keep" => Ok(ItemAction::Keep),
            "Sell" => Ok(ItemAction::Sell),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ItemCategory {
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
    fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

impl FromStr for ItemCategory{
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

#[derive(Debug, Clone)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub category: ItemCategory,
    pub action: ItemAction,
    pub date_added: NaiveDate,
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
