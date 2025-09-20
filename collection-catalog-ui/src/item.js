const { invoke } = window.__TAURI__.core;

document.addEventListener("DOMContentLoaded", async () => {
  const params = new URLSearchParams(window.location.search);
  const id = Number(params.get("id"));

  if (!id) {
    document.getElementById("item-details").innerText = "Invalid item ID.";
    return;
  }

  try {
    const item = await invoke("get_item", { id });
    currentItem = item;

    if (!item) {
      document.getElementById("item-details").innerText = "Item not found.";
      return;
    }

    renderItem(item);
    prefillForm(item);
    
  } catch (err) {
    console.error("Error loading item:", err);
    document.getElementById("item-details").innerText =
      "Failed to load item details.";
  }

  document.getElementById("update-button").addEventListener("click", () => {
    document.getElementById("update-form-container").style.display = "block";
    document.getElementById("item-details").style.display = "none";
  });

  document.getElementById("cancel-update").addEventListener("click", () => {
    document.getElementById("update-form-container").style.display = "none";
    document.getElementById("item-details").style.display = "block";
  });

  document.getElementById("delete-button").addEventListener("click", async () => {
    if(!confirm("Are you sure you want to delete this item?")) return;

    try {
      await invoke("delete_item", { id: parseInt( id, 10 ) });
      alert("Item deleted successfully.");
      window.location.href = "index.html"; // Redirect back to home
    } catch (error) {
      console.error("Error deleting item:", error);
      alert("Failed to delete item.");
    }
    
  });

  document.getElementById("update-form").addEventListener("submit", async (e) => {
    e.preventDefault();
    if (!currentItem) return;

    const formData = new FormData(e.target);
    const updates = {};

    for (const [key, value] of formData.entries()) {
      if (value !== "") {
        updates[key] = value;
      }
    }

    try {
      await invoke("update_item", { id: currentItem.id, updates });
      alert("Item updated successfully!");
      window.location.reload(); // reload to show updated data
    } catch (err) {
      console.error("Update failed:", err);
      alert("Failed to update item.");
    }
  });

});


function renderItem(item) {
  // Render item details in a table
  const container = document.getElementById("item-details");
  container.innerHTML = `
    <table border="1">
      <tr><th>ID</th><td>${item.id}</td></tr>
      <tr><th>Name</th><td>${item.name}</td></tr>
      <tr><th>Description</th><td>${item.description || ""}</td></tr>
      <tr><th>Category</th><td>${item.category || ""}</td></tr>
      <tr><th>Action</th><td>${item.action || ""}</td></tr>
      <tr><th>Creator</th><td>${item.creator || ""}</td></tr>
      <tr><th>Place of Origin</th><td>${item.provenance || ""}</td></tr>
      <tr><th>Age (Years)</th><td>${item.age_years ?? ""}</td></tr>
      <tr><th>Date Acquired</th><td>${item.date_acquired || ""}</td></tr>
      <tr><th>Purchase Price</th><td>${item.purchase_price ?? ""}</td></tr>
      <tr><th>Estimated Value</th><td>${item.estimated_value ?? ""}</td></tr>
      <tr><th>Working Condition</th><td>${
        item.working === null ? "Unknown" : item.working ? "Yes" : "No"
      }</td></tr>
      <tr><th>Date Added</th><td>${item.date_added}</td></tr>
      <tr><th>Last Updated</th><td>${item.last_updated}</td></tr>
    </table>
  `;
}

function prefillForm(item) {
  // Pre-fill form with current item data
  document.getElementById("update-name").value = item.name;
  document.getElementById("update-description").value = item.description || "";
  document.getElementById("update-category").value = item.category || "";
  document.getElementById("update-action").value = item.action || "";
  document.getElementById("update-creator").value = item.creator || "";
  document.getElementById("update-provenance").value = item.provenance || "";
  document.getElementById("update-age").value = item.age_years ?? "";
  document.getElementById("update-date-acquired").value = item.date_acquired || "";
  document.getElementById("update-purchase-price").value = item.purchase_price ?? "";
  document.getElementById("update-estimated-value").value = item.estimated_value ?? "";
  if (item.working !== null) {
    document.getElementById("update-working").value = item.working ? "true" : "false";
  }
}