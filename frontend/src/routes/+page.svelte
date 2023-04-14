<script lang="ts">
	import Modal from '$lib/components/Modal.svelte';
	import { element } from 'svelte/internal';

	let showErrorModal = false;
	let showModal = false;
	let fileName = '';
	let mediaHref = '';
	let hovering = false;

	function forwardRefClick() {
		let upload = document.querySelector('input[type="file"]') as HTMLInputElement;
		upload.click();
	}

	function forwardDropHandler(ev: DragEvent) {
		if (ev.dataTransfer?.files) {
			let input = document.querySelector('input[type="file"]') as HTMLInputElement;
			input.files = ev.dataTransfer.files;
			let file = ev.dataTransfer.files[0];
			if (file) {
				fileName = file.name;
			}
		}
	}

	function handleDragOver(ev: Event) {
		let elem = ev.target as HTMLDivElement
		if (!hovering) {
			elem.style.backgroundColor = "rgba(255, 255, 255, 0.2)"
			hovering = !hovering
		}
		else {
			elem.style.backgroundColor = "rgba(255, 255, 255, 0.15)"
			hovering = !hovering
		}
	}

	function handleFileChange(ev: Event) {
		const input = ev.target as HTMLInputElement;
		if (input.files && input.files.length > 0) {
			let file = input.files[0];
			fileName = file.name;
		}
	}

	async function handleSubmit(e: SubmitEvent) {
		const data = new FormData(e.target as HTMLFormElement);
		const key = data.get('key');
		data.delete('key');

		if (!key) {
			let textLabel = document.getElementById('error-modal-text') as HTMLParagraphElement;
			textLabel!!.innerText = `You must enter a key.`;
			showErrorModal = true;
			return;
		}

		try {
			const res = await fetch('/media/upload', {
				method: 'POST',
				headers: new Headers({
					Authorization: 'Bearer ' + key
				}),
				body: data
			});
			const json = await res.json();
			if (res.status === 401) {
				// Oops! Unauthorized.
				let textLabel = document.getElementById('error-modal-text') as HTMLParagraphElement;
				textLabel!!.innerText = `An error occurred when submitting: ${json.msg}`;
				showErrorModal = true;
			} else {
				mediaHref = `/media/${json.msg}`;
				showModal = true;
			}
		} catch (e) {
			let textLabel = document.getElementById('error-modal-text') as HTMLParagraphElement;
			textLabel!!.innerText = `An error occurred when submitting: ${e}`;
			showErrorModal = true;
		}
	}
</script>

<div class="upload-container">
	<div class="card">
		<div class="header-container">
			<h1>Upload</h1>
		</div>
		<form on:submit|preventDefault={handleSubmit}>
			<!-- svelte-ignore a11y-click-events-have-key-events -->
			<div
				class="file-upload-wrapper"
				on:click={forwardRefClick}
				on:dragenter={handleDragOver}
				on:dragleave={handleDragOver}
				on:drop|preventDefault|stopPropagation={forwardDropHandler}
			>
				<label for="file">
					{#if fileName}
						{fileName}
					{:else}
						Drag a File or Click Here
					{/if}
				</label>
				<input on:change={handleFileChange} type="file" name="file" class="file-upload" />
			</div>
			<div class="form-footer-wrapper">
				<input type="text" name="key" id="key" class="key-entry" placeholder="API Key" />
				<button class="submit-button">Submit</button>
			</div>
		</form>
	</div>
	{#if mediaHref}
		<a href={mediaHref} class="card output-link"> Link </a>
	{/if}
</div>

<Modal bind:showModal={showErrorModal} id="error-modal">
	<h1 slot="header">Error!</h1>

	<p id="error-modal-text" />
</Modal>

<style>
	.upload-container {
		display: flex;
		height: 75vh;
		align-items: center;
		justify-content: center;
		flex-direction: column;
		gap: 8em;
	}

	.card {
		display: flex;
		flex-direction: column;
		background-color: rgba(255, 255, 255, 0.1);
		align-items: center;
		justify-content: center;
		padding: 2em 2em;
		border-radius: 20px;
		width: 16em;
	}

	.card form {
		display: flex;
		flex-direction: column;
		gap: 2em;
	}

	.file-upload-wrapper {
		all: inherit;
		height: 8em;
		background-color: rgba(255, 255, 255, 0.15);
		border-radius: 16px;
		display: flex;
		justify-content: center;
		align-items: center;
		cursor: pointer;
	}

	.file-upload-wrapper:hover {
		background-color: rgba(255, 255, 255, 0.2);
	}

	.file-upload-wrapper-hover {
		background-color: rgba(255, 255, 255, 0.2) !important;
	}

	.file-upload {
		width: 0.1px;
		height: 0.1px;
		opacity: 0;
		overflow: hidden;
		position: absolute;
		z-index: -1;
	}

	.form-footer-wrapper {
		display: flex;
		gap: 1em;
	}

	.form-footer-wrapper input {
		all: inherit;
		background-color: rgba(255, 255, 255, 0.15);
		border-radius: 16px;
		padding: 0.5em 0.5em;
		border-color: rgba(255, 255, 255, 0.2);
	}

	.form-footer-wrapper button {
		all: inherit;
		background-color: #90705c;
		display: flex;
		padding: 0.5em;
		border-radius: 0.5em;
		cursor: pointer;
	}

	.file-upload::file-selector-button {
		display: none;
	}

	.output-link {
		color: #b4e7ce;
		text-decoration: none;
	}

	@media screen and (min-width: 800px) {
		.card {
			transform: scale(1.3);
		}
	}

	:global(dialog#error-modal) {
		background-color: #8b2635;
	}
</style>
