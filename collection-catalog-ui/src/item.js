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
    if (!item) {
      document.getElementById("item-details").innerText = "Item not found.";
      return;
    }

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
  } catch (err) {
    console.error("Error loading item:", err);
    document.getElementById("item-details").innerText =
      "Failed to load item details.";
  }

  // Wire up buttons (placeholders for now)
  document.getElementById("update-button").addEventListener("click", () => {
    alert("TODO: open update form for item " + id);
  });

  document.getElementById("delete-button").addEventListener("click", () => {
    alert("TODO: confirm & delete item " + id);
  });
});
