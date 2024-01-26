<script lang="ts">
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import { fly } from 'svelte/transition';

	import { Button, buttonVariants } from '$lib/components/ui/button';
	import * as Dialog from '$lib/components/ui/dialog';
	import { Input } from '$lib/components/ui/input';
	import { cn } from '$lib/utils';

	import DarkModeSwitchButton from '$lib/components/DarkModeSwitchButton.svelte';
	import TldSearchList from '$lib/components/TLDSearchList.svelte';

	import * as goodomain from 'goodomain-wasm';

	const tldCount = goodomain.tld_count();
	const tldVersion = goodomain.tld_version();

	let word = '';
	let result: goodomain.TLDInWord[] = [];
	let error: string | null = null;

	onMount(() => {
		word = $page.url.searchParams.get('q') ?? '';
	});

	$: {
		if (word.length === 0) {
			result = [];
			error = null;
		} else {
			try {
				result = goodomain.find(word);
				error = null;
			} catch (e: any) {
				error = e;
			}
		}
	}

	const handleClick = (event: MouseEvent, tld: goodomain.TLDInWord) => {
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
	<div class="fixed bottom-0 right-0 m-4">
		Version: {tldVersion}, see
		<Dialog.Root closeOnOutsideClick={false}>
			<Dialog.Trigger class={cn(buttonVariants({ variant: 'outline' }), 'px-1', 'py-0', 'h-auto')}
				>all TLDs</Dialog.Trigger
			>
			<Dialog.Content>
				<Dialog.Header>
					<Dialog.Title>All TLDs in {tldVersion}:</Dialog.Title>
				</Dialog.Header>
				<TldSearchList />
			</Dialog.Content>
		</Dialog.Root>.
	</div>
	<div class="flex h-screen w-full flex-col place-items-center">
		<div class="h-1/4"></div>
		<h1 class="mb-2 text-4xl font-extrabold">Find Your Good Domain</h1>
		<div class="flex h-3/4 w-3/4 flex-col items-center space-y-2 md:w-1/2">
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
				{#if result.length > 0}
					<ul
						class="flex w-full flex-col items-center space-y-4 overflow-y-auto overflow-x-hidden p-2 text-lg"
					>
						{#each result as tld (tld.domain)}
							<li transition:fly={{ x: 200, duration: 200 }}>
								<Button
									variant="outline"
									class="h-auto px-1 py-0 text-lg"
									on:click={(e) => handleClick(e, tld)}>{tld.domain}</Button
								><span>{tld.path}</span>
							</li>
						{/each}
					</ul>
				{/if}
			{/if}
		</div>
	</div>
</div>
