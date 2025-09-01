<script lang="ts">
	export let data;
	let filter: "current" | "completed" = "current";

	const { todos } = data;
</script>

<header>
	<h1>Todo App</h1>
	<nav>
		<button
			onclick={() => (filter = "completed")}
			class={filter === "completed" ? "active" : ""}>Completed</button
		>
		<button
			onclick={() => (filter = "current")}
			class={filter === "current" ? "active" : ""}>Current</button
		>
		<div class="sort-con">
			<label for="sort">Sort By:</label>
			<select id="sort">
				<option value="date">Date</option>
				<option value="a-z">A-Z</option>
				<option value="custom">Custom</option>
			</select>
		</div>
	</nav>
</header>

<main>
	<slot />
</main>

<style>
	h1 {
		text-align: center;
	}

	header {
		margin-bottom: 10px;
	}

	nav {
		display: flex;
		gap: 25px;
		justify-content: center;
	}


</style>
