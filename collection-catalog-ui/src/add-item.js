const { invoke } = window.__TAURI__.core;

document.getElementById("addItemForm").addEventListener("submit", async (e) => {
  e.preventDefault();

  // Helper function to add a field only if it has a value
  const addIfValue = (obj, key, value) => {
    if (value !== null && value !== undefined && value !== "") {
      obj[key] = value;
    }
  };

  let item = {};
  addIfValue(item, "id", 0); // backend assigns ID
  addIfValue(item, "name", document.getElementById("name").value);
  addIfValue(item, "description", document.getElementById("description").value);
  addIfValue(item, "category", document.getElementById("category").value);
  addIfValue(item, "action", document.getElementById("action").value);
  addIfValue(
    item,
    "age_years",
    document.getElementById("age_years").value
      ? parseInt(document.getElementById("age_years").value)
      : null
  );
  addIfValue(item, "date_acquired", document.getElementById("date_acquired").value || null);
  addIfValue(
    item,
    "purchase_price",
    document.getElementById("purchase_price").value
      ? parseFloat(document.getElementById("purchase_price").value)
      : null
  );
  addIfValue(
    item,
    "estimated_value",
    document.getElementById("estimated_value").value
      ? parseFloat(document.getElementById("estimated_value").value)
      : null
  );
  addIfValue(item, "creator", document.getElementById("creator").value || null);
  item["working"] = document.getElementById("working").checked; // boolean, always included
  addIfValue(item, "provenance", document.getElementById("provenance").value || null);

  // These fields will be set by backend
  item["deleted"] = false;

  try {
    await invoke("new_item", { item });
    alert("Item added successfully!");
    window.location.href = "index.html"; // redirect home
  } catch (err) {
    console.error("Failed to add item:", err);
    alert("Failed to add item: " + err);
  }
});
