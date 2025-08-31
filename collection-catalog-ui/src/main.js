const { invoke } = window.__TAURI__.core;

document.addEventListener("DOMContentLoaded", () => {
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
});




// const greetButton = document.getElementById("greet-button");
// const greetMsg = document.getElementById("greet-msg");

// greetButton.addEventListener("click", async () => {
// 	try {
// 		// Call the Rust command
// 		const response = await invoke("greet", { name: "Eleanor" });
// 		greetMsg.textContent = response;
// 	} catch (error) {
// 		console.error("Error calling Rust:", error);
// 	}
// });

