<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { fly } from 'svelte/transition';

	import { Button } from '$lib/components/ui/button';

	import { TLDInWord } from 'goodomain-wasm';

	export let result: TLDInWord[];

	const dispatcher = createEventDispatcher<{
		domainClick: TLDInWord;
	}>();
</script>

{#if result.length > 0}
	<ul
		class="flex w-full flex-col items-center space-y-4 overflow-y-auto overflow-x-hidden p-2 text-lg"
	>
		{#each result as tld (tld.domain)}
			<li transition:fly={{ x: 250, duration: 200 }}>
				<Button
					variant="outline"
					class="h-auto px-1 py-0 text-lg"
					on:click={() => dispatcher('domainClick', tld)}>{tld.domain}</Button
				><span>{tld.path}</span>
			</li>
		{/each}
	</ul>
{/if}
