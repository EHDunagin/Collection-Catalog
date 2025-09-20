const { invoke } = window.__TAURI__.core;

document.addEventListener("DOMContentLoaded", () => {

	document.getElementById("filter-form").addEventListener("submit", (e) => {
		e.preventDefault();
		const formData = new FormData(e.target);
		const params = new URLSearchParams(formData);
		window.location.href = `filter.html?${params.toString()}`;
		});


});



