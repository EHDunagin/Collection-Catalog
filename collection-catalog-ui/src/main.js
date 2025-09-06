const { invoke } = window.__TAURI__.core;

document.addEventListener("DOMContentLoaded", () => {
	// List items button
	document.getElementById("list-button").addEventListener("click", async () => {
		const items = await invoke("list_items");
		console.log("loaded items:", items);
		// document.getElementById("greet-msg").innerText = items.join(", ");
		const item_list = document.getElementById("items-list");
		item_list.innerHTML = "";
		items.forEach(item => {
			const li = document.createElement("li");
			li.textContent = `${item.name} - ${item.description}`;
			item_list.appendChild(li);
		});
	});

	// Add item form
	// document.getElementById("add-item-form").addEventListener("submit", async (e) => {
	// 	e.preventDefault();

	// 	const item = {
	// 		id: 0, // backend will assign
	// 		name: document.getElementById("item-name").value,
	// 		description: document.getElementById("item-description").value,
	// 		category: document.getElementById("item-category").value,
	// 		action: document.getElementById("item-action").value,
	// 		deleted: false
	// 	};

	// 	try {
	// 		await invoke("new_item", { item });
	// 		alert("Item added!");
	// 		document.getElementById("list-button").click(); //refresh list
	// 	} catch (err) {
	// 		alert("Failed to add item: " + err);
	// 	}
	// });
});



