<script lang="ts">
	import { browser } from '$app/environment';

	let hostname = '';
	if (browser) {
		hostname = window.location.hostname;
	}

	let errorText = '';
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

	function handleFileChange(ev: Event) {
		const input = ev.target as HTMLInputElement;
		if (input.files && input.files.length > 0) {
			let file = input.files[0];
			fileName = file.name;
		}
	}

	async function handleSubmit(e: SubmitEvent) {
		errorText = '';
		mediaHref = '';
		const data = new FormData(e.target as HTMLFormElement);
		const key = data.get('key');
		data.delete('key');

		if (!key) {
			errorText = `You must enter a key.`;
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
				errorText = `An error occurred when submitting: ${json.msg}`;
			} else {
				mediaHref = `/media/${json.msg}`;
			}
		} catch (e) {
			errorText = `An error occurred when submitting: ${e}`;
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
				class="file-upload-wrapper {hovering ? 'file-upload-wrapper-hover' : ''}"
				on:click={forwardRefClick}
				on:dragenter|preventDefault={() => (hovering = !hovering)}
				on:dragleave|preventDefault={() => (hovering = !hovering)}
				on:dragover|preventDefault
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
		<div class="card success-card">
			<h2>Success!</h2>
			<p class="card-text">Your image URL is ready.</p>
			<a href={mediaHref} class="unstyled-anchor">{hostname}{mediaHref}</a>
		</div>
	{/if}

	{#if errorText}
		<div class="card error-card">
			<h2>Error!</h2>
			<p class="card-text">{errorText}</p>
		</div>
	{/if}
</div>

<style>
	.upload-container {
		display: flex;
		align-items: stretch;
		justify-content: center;
		flex-direction: column;
		gap: 6em;
		width: 20em;
		margin: 5em auto;
	}

	.card {
		display: flex;
		flex-direction: column;
		background-color: rgba(255, 255, 255, 0.1);
		align-items: center;
		justify-content: center;
		padding: 2em 2em;
		border-radius: 20px;
	}

	.card-text {
		color: rgba(255, 255, 255, 0.8);
	}

	.success-card {
		background-color: #498c50;
		padding: 1em 2em;
		font-size: small;
		gap: 0.1rem;
	}

	.error-card {
		background-color: #8b2635;
		font-size: small;
		gap: 0.1rem;
	}

	.card form {
		display: flex;
		flex-direction: column;
		gap: 2em;
	}

	.file-upload-wrapper {
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

	.form-footer-wrapper input[type="text"] {
		background-color: rgba(255, 255, 255, 0.15);
		border-radius: 16px;
		padding: 0.4em;
		border-color: rgba(255, 255, 255, 0.2);
		border: none;
		color: white;
	}

	.form-footer-wrapper button {
		background-color: #90705c;
		display: flex;
		padding: 0.5em 1.5em;
		border-radius: 16px;
		cursor: pointer;
		border: none;
		color: white;
	}

	.file-upload::file-selector-button {
		display: none;
	}

	.unstyled-anchor {
		text-decoration: none;
		color: inherit;
	}

	.unstyled-anchor:visited {
		color: inherit;
	}

	@media screen and (min-width: 800px) {
		.card {
			transform: scale(1.3);
		}
	}

	:global(body) {
		background-color: #122d35;
		color: white;
		font-family: sans-serif;
	}
</style>
