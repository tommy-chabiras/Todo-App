<script lang="ts">
	import EditIcon from "$lib/components/icons/Edit.svelte";
	import TrashIcon from "$lib/components/icons/Trash.svelte";
	let { todo = $bindable<Todo>(), toggle }: { todo: Todo; toggle: () => void } =
		$props();
</script>

<li>
	<div class="t-front">
		<button onclick={toggle}>
			{#if todo.completed}
				✅
			{:else}
				⬜
			{/if}
		</button>
		<p class="title">{todo.title}</p>
	</div>
	<div class="t-centre">
		<p>{todo.description}</p>
		<p>{todo.startDate.toDateString()}</p>
	</div>
	<div class="t-back">

		<button><EditIcon /></button>
		<button><TrashIcon /></button>
	</div>
</li>

<style>
	li {
		display: flex;
		justify-content: flex-start;
		gap: 25px;
		border: 1px solid black;
		cursor: pointer;
		border-radius: var(--border-r);
		background-color: var(--card-bg-colour);
		padding: 25px 20px;
		align-items: center;
	}

	.t-front,
	.t-centre,
	.t-back {
		display: flex;
		gap: 25px;
		align-items: center;
	}

	.t-front,
	.t-centre {
		flex: 3;
	}


	.t-back {
		flex: 1;
	}

	.t-front > *,
	.t-back > * {
		display: inline-block;
	}

	.title {
		font-weight: 700;
		font-size: 1.8rem;
	}
</style>
