const { invoke } = window.__TAURI__.core;

document.getElementById("addItemForm").addEventListener("submit", async (e) => {
  e.preventDefault();

  let item = { id: 0, deleted: false }; // always included

  function parseBooleanOrNull(value) {
    if (value === "") return null;
    if (value === "true") return true;
    if (value === "false") return false;
    return undefined; // fallback (shouldn't happen)
  }
  // Field configuration
  const fieldConfig = {
    name: { type: "string" },
    description: { type: "string" },
    category: { type: "string" },
    action: { type: "string" },
    age_years: { type: "int" },
    date_acquired: { type: "string" }, // backend parses as date
    purchase_price: { type: "float" },
    estimated_value: { type: "float" },
    creator: { type: "string" },
    provenance: { type: "string" },
    working: { type: "bool" }, // checkbox
  };

  // Helper: add values, with special handling for booleans
  const addIfValue = (key, value, type) => {
    if (type === "bool") {
      item[key] = parseBooleanOrNull(value);
    } else if (value !== null && value !== undefined && value !== "") {
      item[key] = value;
    }
  };

  // Process all configured fields
  Object.entries(fieldConfig).forEach(([id, cfg]) => {
    const el = document.getElementById(id);
    let raw;

    if (cfg.type === "bool") {
      raw = el.checked; // always boolean
    } else {
      raw = el.value.trim();
      if (raw === "") {
        raw = null;
      } else if (cfg.type === "int") {
        raw = parseInt(raw);
      } else if (cfg.type === "float") {
        raw = parseFloat(raw);
      }
      // "string" stays as-is
    }

    addIfValue(id, raw, cfg.type);
  });

  try {
    await invoke("new_item", { item });
      // Add success message to item-details replacing form
      let details = document.getElementById("item-details");
      details.innerHTML = "<p style='color: green; font-weight: bold;'>Item added successfully.</p>"

    // alert("Item added successfully!");
    // window.location.href = "index.html";
  } catch (err) {
    console.error("Failed to add item:", err);
    alert("Failed to add item: " + err);
  }
});
