<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import * as goodomain from 'goodomain-wasm';

	const version = goodomain.tld_version();
	const tlds = goodomain.tld_list();
	let searchWord = '';

	$: filtered = searchWord == '' ? tlds : tlds.filter((tld) => tld.indexOf(searchWord) > 0);
</script>

<div class="flex flex-col space-y-4">
	<Input class="text-md" type="search" bind:value={searchWord} />
	<ul class="w-full h-64 overflow-x-hidden overflow-y-auto space-y-4">
		{#each filtered as tld (tld)}
			<li>{tld}</li>
		{/each}
	</ul>
</div>
