<script lang="ts">
	export let showModal: boolean;
	export let id: string = "";

	let dialog: HTMLDialogElement;

	$: if (dialog && showModal) dialog.showModal();
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<dialog
	bind:this={dialog}
	on:close={() => (showModal = false)}
	on:click|self={() => dialog.close()}
	id={id}
>
	<div on:click|stopPropagation>
		<slot name="header" />

		<slot />

		<!-- svelte-ignore a11y-autofocus -->
		<button autofocus on:click={() => dialog.close()} class="modal-close-button">Close</button>
	</div>
</dialog>

<style>
	dialog {
		min-width: 32em;
		border-radius: 16px;
		border: none;
		padding: 0;
        color: white;
        padding: 1.5em 1.5em;
		font-size: large;
	}

	dialog::backdrop {
		background: rgba(0, 0, 0, 0.3);
	}

    dialog[open] {
		animation: zoom 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
	}

	.modal-close-button {
		all: inherit;
		background-color: white;
		color: black;
		display: flex;
		padding: 0.5em;
		border-radius: 0.5em;
		cursor: pointer;
	}

	@keyframes zoom {
		from {
			transform: scale(0.95);
		}
		to {
			transform: scale(1);
		}
	}
</style>
