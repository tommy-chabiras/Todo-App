<script lang="ts">
	let { todo }: { todo: Todo } = $props();
</script>

<li>
	<div class="t-front">
		<button onclick={() => (todo.completed = !todo.completed)}>
			{#if todo.completed}
				✅
			{:else}
				⬜
			{/if}
		</button>
		<p class="title">{todo.title}</p>
	</div>
	<p>{todo.description}</p>
	<p>{todo.createdAt.toDateString()}</p>
	<button>expand button</button>
</li>

<style>
	li {
		display: flex;
		justify-content: space-between;
		border: 1px solid black;
		cursor: pointer;
		border-radius: var(--border-r);
		background-color: var(--card-bg-colour);
		padding: 25px 20px;
		align-items: center;
	}

	.t-front, .t-back {
		display: flex;
		gap: 25px;
		align-items: center;
	}

	.t-front > *, .t-back > * {
		display: inline-block;
	}

	.title {
		font-weight: 700;
		font-size: 1.8rem;
	}
</style>
