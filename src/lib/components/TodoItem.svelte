<script lang="ts">
	import EditIcon from "./icons/EditIcon.svelte";
	import TrashIcon from "./icons/TrashIcon.svelte";
	import CheckIcon from "./icons/CheckIcon.svelte";
	import UncheckedIcon from "./icons/UncheckIcon.svelte";
	let { todo = $bindable<Todo>(), toggle }: { todo: Todo; toggle: () => void } =
		$props();
</script>

<li>
	<div class="t-front">
		<button onclick={toggle}>
			{#if todo.completed}
				<CheckIcon />
			{:else}
				<UncheckedIcon />
			{/if}
		</button>
		<p class="title">{todo.title}</p>
	</div>
	<div class="t-centre">
		<p>{todo.description}</p>
		<p class="date">{todo.startDate.toDateString()}</p>
	</div>
	<div class="t-back">
		<button class="edit-btn"><EditIcon /></button>
		<button class="trash-btn"><TrashIcon /></button>
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
		height: 100px;
	}

	p {
		max-height: 80%;
		overflow: hidden;
		flex: 1 1 auto;
		line-clamp: 2;

		overflow-y: auto;
	}

	.t-front,
	.t-centre,
	.t-back {
		display: flex;
		gap: 25px;
		align-items: center;
		height: 100%;
	}

	.t-front {
		flex: 3;
	}
	.t-centre {
		flex: 4;
	}

	.t-back {
		flex: 1;
	}

	.date {
		flex: 0 0 auto;
	}

	.t-front > *,
	.t-back > *,
	.t-centre * {
		display: inline-block;
	}

	.title {
		font-weight: 700;
		font-size: 1.8rem;
	}

	button {
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: 100px;
		width: 36px;
		height: 36px;
	}


	button:hover {
		background-color: #ffffff18;
	}
	.edit-btn {
		color: #ffffff;
	}

	.edit-btn:hover {
		transform: scale(1.1);
	}

	.trash-btn {
		color: #ff0000;
	}

	.trash-btn:hover {
		transform: scale(1.1);
	}
</style>
