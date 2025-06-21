use std::str::FromStr;
use rusqlite::{params, Connection, Result};
use crate::models::{Item, ItemAction, ItemCategory};
use chrono::NaiveDate;

pub fn init_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS items (
            id              INTEGER PRIMARY KEY AUTOINCREMENT, 
            name            TEXT NOT NULL, 
            description     TEXT NOT NULL,
            category        TEXT NOT NULL,
            action          TEXT NOT NULL,
            date_added      TEXT NOT NULL, 
            last_updated    TEXT NOT NULL,
            age_years       INTEGER,
            date_acquired   TEXT,
            purchase_price  REAL,
            estimated_value REAL, 
            creator         TEXT,
            working         WORKING,
            provenance      TEXT,
            deleted         INTEGER NOT NULL DEFAULT 0
        )",
        [],
    )?;
    Ok(())
}

pub fn add_item(conn: &Connection, item: &Item) -> Result<()> {
    conn.execute(
        "INSERT INTO items (
            name,
            description,
            category,
            action,
            date_added,
            last_updated,
            age_years,
            date_acquired,
            purchase_price,
            estimated_value,
            creator,
            working,
            provenance,
            deleted
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
        params![
            item.name, 
            item.description,
            item.category.to_string(),
            item.action.to_string(),
            item.date_added.to_string(),
            item.last_updated.to_string(),
            item.age_years,
            item.date_acquired.map(|d| d.to_string()),
            item.purchase_price,
            item.estimated_value,
            item.creator,
            item.working.map(|b| b as i32), //SQLite has no bool type
            item.provenance,
            item.deleted as i32,
        ],
    )?;
    Ok(())
}

pub fn get_all_items(conn: &Connection) -> Result<Vec<Item>> {
    let mut stmt = conn.prepare(
        "SELECT
            id, name, description, category, action,
            date_added, last_updated, 
            age_years, date_acquired, purchase_price,
            estimated_value, creator, working, provenance, deleted
        FROM items
        WHERE deleted = 0"
    )?;

    let item_iter = stmt.query_map([], |row| {
        Ok(Item {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            category: ItemCategory::from_str(row.get::<_, String>(3)?.as_str()).unwrap_or(ItemCategory::Other),
            action: ItemAction::from_str(row.get::<_, String>(4)?.as_str()).unwrap_or(ItemAction::Keep),
            date_added: NaiveDate::parse_from_str(&row.get::<_, String>(5)?, "%Y-%m-%d").unwrap(),
            last_updated: NaiveDate::parse_from_str(&row.get::<_, String>(6)?, "%Y-%m-%d").unwrap(),
            age_years: row.get(7)?,
            date_acquired: row.get::<_, Option<String>>(8)?.and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()),
            purchase_price: row.get(9)?,
            estimated_value: row.get(10)?,
            creator: row.get(11)?,
            working: row.get::<_, Option<i32>>(12)?.map(|b| b != 0),
            provenance: row.get(13)?,
            deleted: row.get::<_, i32>(14)? != 0,
        })
    })?;

    let items: Vec<Item> = item_iter.filter_map(Result::ok).collect();
    Ok(items)
}
