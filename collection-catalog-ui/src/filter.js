const { invoke } = window.__TAURI__.core;

document.addEventListener("DOMContentLoaded", async () => {
  const params = new URLSearchParams(window.location.search);
  const numeric_filters = ["age_years_min", "age_years_max", "purchase_price_min", "purchase_price_max", "estimated_value_min", "estimated_value_max"];
  const date_filters = ["date_added_min", "date_added_max", "last_updated_min", "last_updated_max", "date_acquired_min", "date_acquired_max"];
  const filter = {};

  // Convert query params → filter object (ignoring blanks)
  params.forEach((value, key) => {
    if (value !== "") {
      if (key === "working") {
        filter[key] = value === "true" ? true : value === "false" ? false : null;
      } else if (key === "deleted") {
        filter[key] = value === "true" ? true : value === "false" ? false : null;
      } else if ( numeric_filters.includes(key) ){
        filter[key] = Number(value); // numeric filters
      } else if ( date_filters.includes(key) )
       {
        filter[key] = value; // date filters
      } else {
        filter[key] = value; // string filters
      }
    }
  });

  console.log("Built filter:", filter);

  try {
    const items = await invoke("filter_items", { filter });
    const tbody = document.getElementById("results-body");
    tbody.innerHTML = "";

    if (items.length === 0) {
      tbody.innerHTML =
        '<tr><td colspan="8">No items found.</td></tr>';
    } else {
      items.forEach((item) => {
        const tr = document.createElement("tr");
        tr.innerHTML = `
          <td><a href=item.html?id=${item.id}>${item.id}</a></td>
          <td>${item.name}</td>
          <td>${item.description || ""}</td>
          <td>${item.category || ""}</td>
          <td>${item.creator || ""}</td>
          <td>${item.working === null ? "Unknown" : item.working ? "Yes" : "No"}</td>
          <td>${item.action || ""}</td>
          <td>${item.last_updated}</td>
        `;
        tbody.appendChild(tr);
      });
    }
  } catch (err) {
    console.error("Error filtering items:", err);
  }
});
 

document.getElementById("export-btn").addEventListener("click", async () => {
  const params = new URLSearchParams(window.location.search);
  const filter = Object.fromEntries(params.entries());
  await invoke("export_items_csv", { filter });
  alert("Exported items to CSV.");
});

loadFilteredItems();
