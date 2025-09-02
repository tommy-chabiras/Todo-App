<script lang="ts">
	export let data;
	let filter: "current" | "completed" = "current";
	let mode: "day" | "week" | "month" = "week";
	let view: "list" | "calendar" = "list";

	const { todos } = data;
</script>

<header>
	<nav class="main-nav">
		<button
			onclick={() => (view = "list")}
			class={view === "list" ? "active" : ""}>List</button
		>
		<button
			onclick={() => (view = "calendar")}
			class={view === "calendar" ? "active" : ""}>Calendar</button
		>
	</nav>
</header>
<nav class="sub-nav">
	{#if view === "list"}
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
	{:else if view === "calendar"}
		<button
			onclick={() => (mode = "day")}
			class={mode === "day" ? "active" : ""}>Day</button
		>
		<button
			onclick={() => (mode = "week")}
			class={mode === "week" ? "active" : ""}>Week</button
		>
		<button
			onclick={() => (mode = "month")}
			class={mode === "month" ? "active" : ""}>Month</button
		>
	{/if}
</nav>
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

	.main-nav > button {
		background-color: var(--bg-colour);
		color: var(--font-colour);
		border: 2px solid var(--card-bg-colour);
		flex: 1;
	}

	.sub-nav {
		display: flex;
		gap: 25px;
		margin-bottom: 20px;
	}

	.main-nav,
	.sub-nav {
		display: flex;
		gap: 25px;
		justify-content: center;
		align-items: center;
	}
	.main-nav {
		justify-content: stretch;
	}

	label {
		font-size: var(--font-md);
	}

	button,
	select {
		padding: 10px 25px;
		font-size: var(--font-md);
		border-radius: var(--border-r);
		font-weight: bold;
		cursor: pointer;
	}

	select {
		padding: 10px;
	}

	option:checked {
		font-weight: bold;
		background-color: var(--primary-colour);
	}

	button.active {
		background-color: var(--primary-colour);
	}
</style>
