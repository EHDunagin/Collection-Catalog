import { invoke } from "@tauri-apps/api/tauri";

document.getElementById("addItemForm").addEventListener("submit", async (e) => {
  e.preventDefault();

  const item = {
    id: 0, // backend assigns ID
    name: document.getElementById("name").value,
    description: document.getElementById("description").value,
    category: document.getElementById("category").value,
    action: document.getElementById("action").value,
    age_years: document.getElementById("age_years").value ? parseInt(document.getElementById("age_years").value) : null,
    date_acquired: document.getElementById("date_acquired").value || null,
    purchase_price: document.getElementById("purchase_price").value ? parseFloat(document.getElementById("purchase_price").value) : null,
    estimated_value: document.getElementById("estimated_value").value ? parseFloat(document.getElementById("estimated_value").value) : null,
    creator: document.getElementById("creator").value || null,
    working: document.getElementById("working").checked,
    provenance: document.getElementById("provenance").value || null,

    // These will be set by backend
    date_added: null,
    last_updated: null,
    deleted: false,
  };

  try {
    await invoke("new_item", { item });
    alert("Item added successfully!");
    window.location.href = "index.html"; // redirect home
  } catch (err) {
    console.error("Failed to add item:", err);
    alert("Failed to add item: " + err);
  }
});
