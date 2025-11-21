<script lang="ts">
	import CalendarCell from "$lib/components/CalendarCell.svelte";
	import TodoItem from "$lib/components/TodoItem.svelte";
	import { nav } from "$lib/stores/nav";
	import { invoke } from "@tauri-apps/api/core";


	let todos: Todo[] = $state([]);
	invoke<Todo[]>("get_todos")
	.then((data) => {
		todos = data.map(todo => ({
			...todo,
			startDate: new Date(todo.startDate),
			endDate: new Date(todo.endDate),
			createdAt: new Date(todo.createdAt),
		}));
	})
	.catch((err) => {
		console.error("Failure to load todos:", err);
	})
	
	// const todos: Todo[] = [
	// 	{
	// 		id: "1",
	// 		title: "first todo",
	// 		description: "example descrgfewiug feuifew ghifuewbviuieowb iuuiveboifeq uivewiovwebv oweivweuiobefwi bvweioubgeiu geb",
	// 		completed: false,
	// 		startDate: new Date(),
	// 		endDate: new Date(),
	// 		createdAt: new Date(),
	// 	},
	// 	{
	// 		id: "2",
	// 		title: "a second todo",
	// 		description: "example descr",
	// 		completed: false,
	// 		startDate: new Date(),
	// 		endDate: new Date(),
	// 		createdAt: new Date(),
	// 	},
	// ];

	let todosFiltered: Todo[] = $state([]);

	$effect(() => {
		let filtered = todos;
		switch ($nav.view) {
			case "list":
				switch ($nav.list.sort) {
					case "date":
						filtered = filtered.sort(
							(a, b) => b.startDate.getTime() - a.startDate.getTime()
						);
						break;
					case "a-z":
						filtered = filtered.sort((a, b) => a.title.localeCompare(b.title));
						break;
					case "custom":
						break;
				}
				break;
			case "calendar":
				const today = new Date();
				let tempDate = new Date();
				tempDate.setHours(0, 0, 0, 0);

				switch ($nav.calendar.filter) {
					case "day":
						filtered = filtered.filter(
							(todo) =>
								todo.startDate.getFullYear() === today.getFullYear() &&
								todo.startDate.getMonth() === today.getMonth() &&
								todo.startDate.getDate() === today.getDate()
						);
						break;
					case "week":
						tempDate.setDate(tempDate.getDate() - tempDate.getDay());
						filtered = filtered.filter((todo) => todo.startDate >= tempDate);
						break;
					case "month":
						tempDate.setDate(1);
						filtered = filtered.filter((todo) => todo.startDate >= tempDate);
						break;
				}
		}
		todosFiltered = filtered;
	});


	const message = invoke('greet', { name: 'Tommy' });
	message.then(data => {
		console.log(data)
	});

</script>

{#if $nav.view === "calendar"}
	{#each todos as todo}
		<CalendarCell {todo}></CalendarCell>
	{/each}
{:else}
	<ul>
		{#each todosFiltered as todo}
			{#if ($nav.list.filter === "current" && !todo.completed) || ($nav.list.filter === "completed" && todo.completed) || $nav.list.filter === "all"}
				<TodoItem
					{todo}
					toggle={() => {
						const original = todos.find((t) => t.id === todo.id);
						todo.completed = !todo.completed;
						if (original) original.completed = !original.completed;
					}}
				/>
			{/if}
		{/each}
	</ul>
{/if}



<style>
</style>
