<script lang="ts">
	import { browser } from '$app/environment';
	import { PUBLIC_GOOGLE_ANALYTICS_ID } from '$env/static/public';
	import '../app.pcss';

	import { ModeWatcher } from 'mode-watcher';

	if (browser) {
		if (PUBLIC_GOOGLE_ANALYTICS_ID) {
			(window as any).dataLayer = (window as any).dataLayer || [];

			function gtag(..._args: any[]) {
				(window as any).dataLayer.push(arguments);
			}

			gtag('js', new Date());
			gtag('config', PUBLIC_GOOGLE_ANALYTICS_ID);
			console.info('Google Analytics initialized.');
		} else {
			console.debug('NO Google Analytics ID provided.');
		}
	}
</script>

<svelte:head>
	{#if PUBLIC_GOOGLE_ANALYTICS_ID}
		<script
			async
			src="https://www.googletagmanager.com/gtag/js?id={PUBLIC_GOOGLE_ANALYTICS_ID}"
		></script>
	{/if}
</svelte:head>

<ModeWatcher />
<slot />
