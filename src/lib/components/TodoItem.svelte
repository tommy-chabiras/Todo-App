<script lang="ts">
	let { todo }: { todo: Todo } = $props();
	let completed = $state(todo.completed);
</script>

<li>
	<button onclick={() => (completed = !completed)}>
		{#if completed}
			✅
		{:else}
			⬜
		{/if}
	</button>
	<p>{todo.title}</p>
	<p>{todo.description}</p>
	<button>expand button</button>
</li>
