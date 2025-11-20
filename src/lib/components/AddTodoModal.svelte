<script lang="ts">
	import { stopPropagation } from "svelte/legacy";

	let { showModal = $bindable() } = $props();

</script>

<div class="modal-backdrop" onclick={(e) => {if (e.target == e.currentTarget) showModal = false}} role="presentation" aria-label="Close modal">
	<div class="modal-con">
		<h1>Add Todo</h1>
		<form class="task-form">
			<div class="field">
				<label for="title">Title:</label>
				<input type="text" id="title" name="title" required />
			</div>

			<div class="field">
				<label for="description">Description:</label>
				<textarea id="description" name="description" rows="4" required
				></textarea>
			</div>

			<div class="field">
				<label for="date">Finish By:</label>
				<input type="date" id="date" name="date" required />
			</div>

			<div class="buttons">
				<button onclick={() => showModal = false} class="modal-button" type="button">Cancel</button>
				<button onclick={() => showModal = false} class="modal-button" type="submit">Add Todo</button>
			</div>
		</form>
	</div>
</div>

<style>
	h1 {
		text-align: center;
		margin-bottom: 20px;
	}

	.field > label {
		font-weight: 700;
		margin-bottom: 5px;
	}

	.field input,
	.field textarea {
		padding: 5px 10px;
		border-radius: 5px;
		font-size: var(--font-md);
	}

	.modal-backdrop {
		position: fixed;
		top: 0;
		left: 0;
		width: 100vw;
		height: 100vh;
		background: rgba(0, 0, 0, 0.5);
		display: flex;
		justify-content: center;
		align-items: center;
		z-index: 1000;
	}

	.field textarea {
		resize: none;
	}

	.modal-con {
		position: absolute;
		background-color: var(--card-bg-colour);
		border-radius: var(--border-r);
		padding: 25px;
		width: 40%;
	}

	.field {
		display: flex;
		flex-direction: column;
		margin-bottom: 20px;
	}

	.buttons {
		display: flex;
		justify-content: space-between;
	}

	.modal-button {
		font-weight: 700;
		padding: 15px 30px;
		border-radius: var(--border-r);
		background-color: var(--primary-colour);
	}
</style>
