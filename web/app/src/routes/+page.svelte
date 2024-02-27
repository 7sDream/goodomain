<script lang="ts">
	import { page } from '$app/stores';
	import { onMount } from 'svelte';

	import { buttonVariants } from '$lib/components/ui/button';
	import * as Dialog from '$lib/components/ui/dialog';
	import { Input } from '$lib/components/ui/input';
	import { cn } from '$lib/utils';

	import DarkModeSwitchButton from '$lib/components/DarkModeSwitchButton.svelte';
	import DomainList from '$lib/components/DomainList.svelte';
	import TldSearchList from '$lib/components/TLDSearchList.svelte';

	import * as goodomain from 'goodomain-wasm';

	const tldVersion = goodomain.tld_version();
	const tldCount = goodomain.tld_count();

	let word = '';
	let result: goodomain.TLDInWord[] = [];
	let error: string | null = null;

	onMount(() => {
		word = $page.url.searchParams.get('q') ?? '';
	});

	$: {
		try {
			result = goodomain.find(word);
			error = null;
		} catch (e: any) {
			error = e;
		}
	}

	const handleClick = (tld: goodomain.TLDInWord) => {
		window.open(`https://www.iana.org/whois?q=${encodeURIComponent(tld.tld)}`, '_blank');
	};
</script>

<svelte:head>
	<title>Goodomain - Find your good domain</title>
</svelte:head>

<div>
	<div class="fixed right-0 top-0 m-4">
		<DarkModeSwitchButton />
	</div>
	<div class="flex h-screen w-full flex-col place-items-center space-y-4">
		<div class="h-1/4"></div>
		<h1 class="text-4xl font-extrabold">Goodomain</h1>
		<div class="flex h-3/4 w-3/4 flex-col items-center space-y-4 md:w-1/2">
			<Input
				class="text-lg shadow-lg"
				type="search"
				placeholder="Your favorite word"
				bind:value={word}
			/>
			{#if error !== null}
				<p class="text-destructive">{error}</p>
			{:else}
				{#if word.length > 0 && result.length === 0}
					<p class="text-secondary-foreground">
						No TLD in this word, try another word or continue typing...
					</p>
				{/if}
				<DomainList {result} on:domainClick={(ev) => handleClick(ev.detail)} />
			{/if}
		</div>
	</div>
	<div class="fixed bottom-0 right-0 m-4">
		Data version: {tldVersion}, see
		<Dialog.Root closeOnOutsideClick={false}>
			<Dialog.Trigger class={cn(buttonVariants({ variant: 'outline' }), 'px-1', 'py-0', 'h-auto')}
				>all TLDs</Dialog.Trigger
			>
			<Dialog.Content>
				<Dialog.Header>
					<Dialog.Title>{tldCount} TLDs in version {tldVersion}:</Dialog.Title>
				</Dialog.Header>
				<TldSearchList />
			</Dialog.Content>
		</Dialog.Root>.
	</div>
</div>
