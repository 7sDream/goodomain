<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import * as goodomain from 'goodomain-wasm';

	const tlds = goodomain.tld_list();

	let searchWord = '';
	$: searchWordLower = searchWord.toLowerCase();
	$: filtered = searchWord == '' ? tlds : tlds.filter((tld) => tld.indexOf(searchWordLower) !== -1);
</script>

<div class="flex flex-col space-y-4">
	<Input class="text-md" type="search" bind:value={searchWord} />
	<ul class="h-64 w-full space-y-4 overflow-y-auto overflow-x-hidden">
		{#each filtered as tld (tld)}
			<li>{tld}</li>
		{/each}
	</ul>
	<div class="flex flex-row-reverse">
		<p class="text-secondary-foreground/50 text-xs">
			Data source: <a
				class="hover:text-primary underline underline-offset-auto"
				href="https://www.iana.org/"
				target="_blank">IANA</a
			>.
		</p>
	</div>
</div>
