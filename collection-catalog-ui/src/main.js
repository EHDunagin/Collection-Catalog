const { invoke } = window.__TAURI__.core;

const greetButton = document.getElementById("greet-button");
const greetMsg = document.getElementById("greet-msg");

greetButton.addEventListener("click", async () => {
	try {
		// Call the Rust command
		const response = await invoke("greet", { name: "Eleanor" });
		greetMsg.textContent = response;
	} catch (error) {
		console.error("Error calling Rust:", error);
	}
});

