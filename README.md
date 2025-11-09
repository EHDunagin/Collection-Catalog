# Collection-Catalog
A lightweight desktop application built with RUst, Tauri and vanilla JavaScript for cataloging personal collections. The goal of this project is primarily practice. The idea was to create a simple tool that might be useful to some relatives.

## Features
- **Add, View, Update, and Delete Items** Maintain detailed records of items in your collections.
- **Restore Deleted Items** Allows for restoration of previously deleted items.
- Offline-first Desktop App Built with Tauri -- runs locally with no server required.
- Simple HTML + JavaScript Interface
- **Local Storage** Data is saved locally using Rust backend logic with SQLite (through Tauri).

## Application Structure
```
Collection-Catalog/
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── README.md
├── collection-catalog-cli
│   ├── Cargo.toml
│   ├── data
│   └── src
│       └── main.rs
├── collection-catalog-core
│   ├── Cargo.toml
│   └── src
│       ├── csv_export.rs
│       ├── db.rs
│       ├── lib.rs
│       ├── main.rs
│       └── models.rs
├── collection-catalog-ui
│   ├── README.md
│   ├── src
│   │   ├── add-item.html
│   │   ├── add-item.js
│   │   ├── assets
│   │   ├── filter.html
│   │   ├── filter.js
│   │   ├── index.html
│   │   ├── item.html
│   │   ├── item.js
│   │   ├── main.js
│   │   └── styles.css
│   └── src-tauri
│       ├── Cargo.toml
│       ├── src
│       │   ├── lib.rs
│       │   └── main.rs
│       └── tauri.conf.json
├── data
└── tests
    ├── db_tests.rs
    └── models_tests.rs
```

## Key Files
`item.js`
* Displays full item details
* Handles:
    * **Update:** Edits item data and refreshes page.
    * **Delete:** Marks an item as deleted, then shows "Item Deleted" message.
    * **Restore:** Reverses deletion. Option visible only for deleted items.

`index.html`
* Initial display page
* Provides links for adding and restoring items.
* Provides form to search for items based on several criteria and retrieve a list

`filter.js`
* Filters items and provides user with a selectable list

`src-tauri/src/main.rs`
* Defines Tauri commands accessible from JavaScript including:
    * list_items
    * new_item
    * filter_items
    * get_item
    * update_item
    * delete_item
    * export_filtered_items_to_csv

## How it Works
1. Data Flow
    * The frontend uses `window.__TAURI__.core.invoke()` to call Rust commands.
    * Rust functions interact with the local SQLite database to perform CRUD operations.
2. Frontend Navigation
    * Navigation uses standard HTML links (`<a href="...">`).
3. Deletion & Restoration
    * “Deleting” marks the item’s `deleted` flag as `true` in the database.
    * The “Restore Deleted Items” view lists all records with `deleted = true`.
    * Restoring an item updates `deleted` to `false`.
