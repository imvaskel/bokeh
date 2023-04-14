<script lang="ts">
	import { page } from '$app/stores';
	page.subscribe(() => {});
</script>

<nav class="navbar">
	<h1 class="navbar-branding">Bokeh</h1>
	<ul>
		<li><a href="/" class="link" aria-current={$page.route.id === '/'}>Home</a></li>
		<li><a href="/admin" class="link" aria-current={$page.route.id === '/admin'}>Admin</a></li>
	</ul>
</nav>

<slot />

<style>
	.navbar {
		min-width: 100%;
		min-height: 50px;
		flex-direction: row;
		display: flex;
		font-size: 1.3em;
	}

	.navbar ul {
		list-style: none;
		display: flex;
		gap: 0.5em;
		align-items: center;

	}

	.link {
		text-decoration: inherit;
		color: inherit;
	}

	.link[aria-current] {
		position: relative;
		display: inline-block;
	}

	.link[aria-current="true"]::after {
		position: absolute;
		content: '';
		width: 100%;
		height: 2px;
		background-color: #E3655B;
		display: block;
		bottom: -1px;
	}

	:global(body) {
		background-color: #122d35;
		color: white;
		font-family: sans-serif;
	}
</style>
