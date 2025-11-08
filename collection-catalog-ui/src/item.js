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
    // Show update form and hide current item details, update button and delete button
    document.getElementById("update-form-container").style.display = "block";
    document.getElementById("item-details").style.display = "none";
    document.getElementById("update-button").style.display = "none";
    document.getElementById("delete-button").style.display = "none";
  });

  document.getElementById("cancel-update").addEventListener("click", () => {
    document.getElementById("update-form-container").style.display = "none";
    document.getElementById("item-details").style.display = "block";
    document.getElementById("update-button").style.display = "inline";
    document.getElementById("delete-button").style.display = "inline";
  });

  document.getElementById("delete-button").addEventListener("click", async () => {
    
    const confirmed = await confirmDialog("Are you sure you want to delete this item?");
    if (!confirmed) {
      console.log("Deletion cancelled by user");
      return;
    }

    try {
      await invoke("delete_item", { id: parseInt( id, 10 ) });

      // Replace item details with success message
      const details = document.getElementById("item-details");
      details.innerHTML = "<p style='color: green; font-weight: bold;'>Item deleted successfully.</p>";

      // Hide update and delete buttons
      document.getElementById("update-button").style.display = "none";
      document.getElementById("delete-button").style.display = "none";

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
      // Re-fetch the item and re-render it
      const updatedItem = await invoke("get_item", { id: currentItem.id });
      renderItem(updatedItem);

      // Hide the form, show the details and buttons again
      document.getElementById("update-form-container").style.display = "none";
      document.getElementById("item-details").style.display = "block";
      document.getElementById("update-button").style.display = "inline";
      document.getElementById("delete-button").style.display = "inline";

      // Add success message to item-details
      let details = document.getElementById("item-details");
      details.innerHTML += "<p style='color: green; font-weight: bold;'>Item updated successfully.</p>";
    } catch (err) {
      console.error("Update failed:", err);
      alert("Failed to update item.");
    }
  });

  document.getElementById("restore-button").addEventListener("click", async () => {
    const updates = {deleted: "false"};
    try {
      await invoke("update_item", {id: currentItem.id, updates});
      // Re-fetch the item and re-render it
      const restoredItem = await invoke("get_item", { id: currentItem.id });
      renderItem(restoredItem);

      // Add success message to item-details
      let details = document.getElementById("item-details");
      details.innerHTML += "<p style='color: green; font-weight: bold;'>Item restored successfully.</p>";
    } catch (err) {
      console.error("Restore failed:", err);
      alert("Failed to restore item.");
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
  
  // check whether item.deleted is true. If so, hide update/delete options. Show a Restore button instead.
  if (item.deleted) {
    document.getElementById("update-button").style.display = "none";
    document.getElementById("delete-button").style.display = "none";
    const rb = document.getElementById("restore-button");
    rb.style.display = "inline-block";
  }
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

function confirmDialog(message) {
  return new Promise((resolve) => {
    const modal = document.getElementById("confirm-modal");
    const text = document.getElementById("confirm-text");
    const yesBtn = document.getElementById("confirm-yes");
    const noBtn = document.getElementById("confirm-no");

    text.textContent = message;
    modal.style.display = "flex";

    function cleanup(result) {
      modal.style.display = "none";
      yesBtn.removeEventListener("click", onYes);
      noBtn.removeEventListener("click", onNo);
      resolve(result);
    }

    function onYes() { cleanup(true); }
    function onNo() { cleanup(false); }

    yesBtn.addEventListener("click", onYes);
    noBtn.addEventListener("click", onNo);
  });
}
