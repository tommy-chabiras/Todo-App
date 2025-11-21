<script lang="ts">
	import CalendarCell from "$lib/components/CalendarCell.svelte";
	import TodoItem from "$lib/components/TodoItem.svelte";
	import { nav } from "$lib/stores/nav";
	import { invoke } from "@tauri-apps/api/core";
	import { todos } from "$lib/stores/todos";

	let todosFiltered: Todo[] = $state([]);

	$effect(() => {
		invoke<Todo[]>("get_todos")
			.then((data) => {
				todos.set(
					data.map((todo) => ({
						...todo,
						startDate: new Date(todo.startDate),
						endDate: new Date(todo.endDate),
						createdAt: new Date(todo.createdAt),
					}))
				);
			})
			.catch(console.error);
	});

	$effect(() => {
		let filtered = [...$todos];
		switch ($nav.view) {
			case "list":
				switch ($nav.list.sort) {
					case "date":
						filtered.sort(
							(a, b) => b.startDate.getTime() - a.startDate.getTime()
						);
						break;
					case "a-z":
						filtered.sort((a, b) => a.title.localeCompare(b.title));
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

	function toggle(todo: Todo) {
		todo.completed = !todo.completed;
		invoke("update_todo", {
			todo: {
				id: todo.id,
				title: todo.title,
				description: todo.description,
				completed: todo.completed,
				endDate: todo.endDate.toISOString(),
			},
		})
			.then(() => {
				todos.update((data) => {
					const t = data.find((t) => t.id === todo.id);
					if (t) t.completed = !t.completed;
					return data;
				});
			})
			.catch((err) => {
				console.error("Failed to update todo:", err);
				todo.completed = !todo.completed;
			});
	}
</script>

{#if $nav.view === "calendar"}
	{#each $todos as todo}
		<CalendarCell {todo}></CalendarCell>
	{/each}
{:else}
	<ul>
		{#each todosFiltered as todo}
			{#if ($nav.list.filter === "current" && !todo.completed) || ($nav.list.filter === "completed" && todo.completed) || $nav.list.filter === "all"}
				<TodoItem {todo} toggle={() => toggle(todo)} />
			{/if}
		{/each}
	</ul>
{/if}

<style>
</style>
