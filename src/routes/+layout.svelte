<script lang="ts">
	import { nav } from "$lib/stores/nav";
</script>

<header>
	<nav class="main-nav">
		<button
			onclick={() => ($nav.view = "list")}
			class={$nav.view === "list" ? "active" : ""}>List</button
		>
		<button
			onclick={() => ($nav.view = "calendar")}
			class={$nav.view === "calendar" ? "active" : ""}>Calendar</button
		>
	</nav>
</header>
<nav class="sub-nav">
	{#if $nav.view === "list"}
		<button
			onclick={() =>
				nav.update((n) => ({ ...n, list: { ...n.list, filter: "completed" } }))}
			class={$nav.list.filter === "completed" ? "active" : ""}>Completed</button
		>
		<button
			onclick={() =>
				nav.update((n) => ({ ...n, list: { ...n.list, filter: "current" } }))}
			class={$nav.list.filter === "current" ? "active" : ""}>Current</button
		>
		<div class="sort-con">
			<label for="sort">Sort By:</label>
			<select id="sort" bind:value={$nav.list.sort}>
				<option value="date">Date</option>
				<option value="a-z">A-Z</option>
				<option value="custom">Custom</option>
			</select>
		</div>
	{:else if $nav.view === "calendar"}
		<button
			onclick={() =>
				nav.update((n) => ({
					...n,
					calendar: { ...n.calendar, filter: "day" },
				}))}
			class={$nav.calendar.filter === "day" ? "active" : ""}>Day</button
		>
		<button
			onclick={() =>
				nav.update((n) => ({
					...n,
					calendar: { ...n.calendar, filter: "week" },
				}))}
			class={$nav.calendar.filter === "week" ? "active" : ""}>Week</button
		>
		<button
			onclick={() =>
				nav.update((n) => ({
					...n,
					calendar: { ...n.calendar, filter: "month" },
				}))}
			class={$nav.calendar.filter === "month" ? "active" : ""}>Month</button
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
		background-color: #FFFFFF;
		padding: 10px 25px;
		font-size: var(--font-md);
		border-radius: var(--border-r);
		font-weight: bold;
		cursor: pointer;
	}

	select {
		padding: 10px 30px 10px 10px;
		appearance: none;

		background-image: url("data:image/svg+xml;utf8,<svg fill='currentColor' height='16' viewBox='0 0 24 24' width='16' xmlns='http://www.w3.org/2000/svg'><path d='M7 10l5 5 5-5z'/></svg>");
		background-repeat: no-repeat;
		background-position: right 0.5em center;
		background-size: var(--font-md);
	}

	option:checked {
		font-weight: bold;
		background-color: var(--primary-colour);
	}

	button.active {
		background-color: var(--primary-colour);
	}
</style>
