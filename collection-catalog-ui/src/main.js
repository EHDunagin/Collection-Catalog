const { invoke } = window.__TAURI__.core;

document.addEventListener("DOMContentLoaded", () => {
	// List items button
	document.getElementById("list-button").addEventListener("click", async () => {
		const items = await invoke("list_items");
		console.log("loaded items:", items);
		// document.getElementById("greet-msg").innerText = items.join(", ");
		const item_list = document.getElementById("list_button");
		item_list.innerHTML = "";
		items.forEach(item => {
			const li = document.createElement("li");
			li.textContent = `${item.name} - ${item.description}`;
			item_list.appendChild(li);
		});
	});

	document.getElementById("filter-form").addEventListener("submit", (e) => {
		e.preventDefault();
		const formData = new FormData(e.target);
		const params = new URLSearchParams(formData);
		window.location.href = `filter.html?${params.toString()}`;
		});


});



