use std::str::FromStr;
use rusqlite::{params, Connection, Result, ToSql };
use crate::models::{Item, ItemAction, ItemCategory, ItemFilter};
use chrono::NaiveDate;
use anyhow::{ Result as AnyResult, anyhow };

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

pub fn get_item_by_id(conn: &Connection, id: i32) -> Result<Option<Item>> {
    let mut stmt = conn.prepare(
        "SELECT
            id, name, description, category, action,
            date_added, last_updated, 
            age_years, date_acquired, purchase_price,
            estimated_value, creator, working, provenance, deleted
        FROM items
        WHERE id = ?1"
    )?;

    let mut rows = stmt.query(params![id])?;

    if let Some(row) = rows.next()? {
        Ok(Some(Item {   
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
        }))
    } else {
        Ok(None)
    }
}

pub fn get_filtered_items(conn: &Connection, filter: ItemFilter) -> rusqlite::Result<Vec<Item>> {
    let mut sql = String::from("SELECT * FROM items WHERE 1=1");
    let mut param_values: Vec<(String, Box<dyn ToSql>)> = Vec::new();

    // Helper macro to append filters
    macro_rules! push_filter {
        ($opt:expr, $field:expr, $param:expr, $op:expr) => {
            if let Some(val) = $opt.clone() {
                let name = format!(":{}", $param);
                sql.push_str(&format!(" AND {} {} {}", $field, $op, &name));
                param_values.push((name, Box::new(val)));
            }
        };
    }

    macro_rules! push_like {
        ($opt:expr, $field:expr) => {
            if let Some(val) = $opt.clone() {
                let name = format!(":{}", $field);
                sql.push_str(&format!(" AND {} LIKE {}", $field, &name));
                param_values.push((name, Box::new(format!("%{}%", val))));
            }
        };
    }

    // LIKE filters
    push_like!(filter.name_contains, "name");
    push_like!(filter.description_contains, "description");
    push_like!(filter.creator_contains, "creator");
    push_like!(filter.provenance_contains, "provenance");

    // Exact match filters (Enums and bools)
    push_filter!(filter.category.map(|c| c.to_string()), "category", "category", "=");
    push_filter!(filter.action.map(|a| a.to_string()), "action", "action", "=");
    push_filter!(filter.working, "working", "working", "=");
    push_filter!(filter.deleted, "deleted", "deleted", "=");

    // Date range filters
    push_filter!(filter.date_added_min.map(|d| d.to_string()), "date_added", "date_added_min", ">=");
    push_filter!(filter.date_added_max.map(|d| d.to_string()), "date_added", "date_added_max", "<=");
    push_filter!(filter.last_updated_min.map(|d| d.to_string()), "last_updated", "last_updated_min", ">=");
    push_filter!(filter.last_updated_max.map(|d| d.to_string()), "last_updated", "last_updated_max", "<=");
    push_filter!(filter.date_acquired_min.map(|d| d.to_string()), "date_acquired", "date_acquired_min", ">=");
    push_filter!(filter.date_acquired_max.map(|d| d.to_string()), "date_acquired", "date_acquired_max", "<=");

    // Numeric range filters
    push_filter!(filter.age_years_min, "age_years", "age_years_min", ">=");
    push_filter!(filter.age_years_max, "age_years", "age_years_max", "<=");
    push_filter!(filter.purchase_price_min, "purchase_price", "purchase_price_min", ">=");
    push_filter!(filter.purchase_price_max, "purchase_price", "purchase_price_max", "<=");
    push_filter!(filter.estimated_value_min, "estimated_value", "estimated_value_min", ">=");
    push_filter!(filter.estimated_value_max, "estimated_value", "estimated_value_max", "<=");

    // Prepare named params: Vec<(&str, &dyn ToSql)>
    let params: Vec<(&str, &dyn ToSql)> = param_values
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_ref() as &dyn ToSql))
        .collect();

    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map(&params[..], Item::from_row)?;
    let items = rows.collect::<Result<Vec<_>, _>>()?;
    Ok(items)
}


pub fn add_item(conn: &Connection, item: &Item) -> AnyResult<()> {
    item.validate()
        .map_err(|errs| anyhow!("Validation failed: {}", errs.join("; ")))?;

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

pub fn update_item(conn: &Connection, item: &Item) -> AnyResult<()> {

    item.validate()
        .map_err(|errs| anyhow!("Validation failed: {}", errs.join("; ")))?;

    conn.execute(
        "UPDATE items SET
            name = ?1,
            description = ?2,
            category = ?3,
            action = ?4,
            date_added = ?5,
            last_updated = ?6,
            age_years = ?7,
            date_acquired = ?8,
            purchase_price = ?9,
            estimated_value = ?10,
            creator = ?11,
            working = ?12,
            provenance = ?13,
            deleted = ?14
        WHERE id = ?15",
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
            item.id,
        ],
    )?;

    Ok(())
}


pub fn soft_delete_item(conn: &Connection, item_id: i32) -> Result<()> {
    let today = chrono::Local::now().naive_local().date();
    conn.execute(
        "UPDATE items
        SET deleted = 1,
            last_updated = ?1
        WHERE id = ?2",
        params![today.to_string(), item_id],
    )?;

    Ok(())
}

