const { invoke } = window.__TAURI__.core;

let currentFilter = {}; // Defined globally so export-csv can use after created on DOMContentLoaded

// Convert DB enum-like categories to a nicer display format
const CATEGORY_MAP = {
  Antique: "Antique",
  Book: "Book",
  Decor: "Decor",
  ElectronicDevice: "Electronic Device",
  Furniture: "Furniture",
  HouseholdItem: "Household Item",
  Kitchenware: "Kitchenware",
  MineralSpecimen: "Mineral Specimen",
  Tool: "Tool",
  Wood: "Wood",
  Other: "Other"
};

function prettyCategory(code) {
  return CATEGORY_MAP[code] || code || "";
}


const exportBtn = document.getElementById("export-csv");
exportBtn.disabled = true; // disable until results are loaded

document.addEventListener("DOMContentLoaded", async () => {
  const params = new URLSearchParams(window.location.search);
  const numeric_filters = [
    "age_years_min",
    "age_years_max",
    "purchase_price_min",
    "purchase_price_max",
    "estimated_value_min",
    "estimated_value_max",
  ];
  const date_filters = [
    "date_added_min",
    "date_added_max",
    "last_updated_min",
    "last_updated_max",
    "date_acquired_min",
    "date_acquired_max",
  ];
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

  currentFilter = filter;
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
          <td><a href="item.html?id=${item.id}">${item.id}</a></td>
          <td>${item.name}</td>
          <td>${item.description || ""}</td>
          <td>${prettyCategory(item.category)}</td>
          <td>${item.creator || ""}</td>
          <td>${item.working === null ? "Unknown" : item.working ? "Yes" : "No"}</td>
          <td>${item.action || ""}</td>
          <td>${item.age_years ?? ""}</td>
          <td>${item.date_acquired || ""}</td>
          <td>${item.purchase_price ?? ""}</td>
          <td>${item.estimated_value ?? ""}</td>
          <td>${item.provenance || ""}</td>
          <td>${item.date_added}</td>
          <td>${item.last_updated}</td>
        `;
        tbody.appendChild(tr);
      });
      exportBtn.disabled = false; // enable export only if items exist
    }
  } catch (err) {
    console.error("Error filtering items:", err);
    exportBtn.disabled = true;
  }
});
 
document.getElementById("export-csv").addEventListener("click", async () => {
  try {
    // Use Rust function to export filtered items to CSV
    const csvContent = await invoke("export_filtered_items_to_csv", { 
      filter: currentFilter,
    });

  } catch (err) {
    console.error("Export failed:", err);
  }
});

