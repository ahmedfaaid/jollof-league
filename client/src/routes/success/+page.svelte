<script>
	import Icon from '@iconify/svelte';
</script>

<div class="xs:px-8 h-[calc(100vh-80px)] px-2 sm:px-16">
	<div class="relative h-full">
		<a
			href="/"
			class="text-dark-cerulean xs:left-8 absolute left-2 top-8 flex items-center sm:left-16"
		>
			<Icon icon="solar:home-line-duotone" class="text-xl" />
			<span class="ml-2 text-sm">Home</span>
		</a>
		<div class="flex h-full items-center justify-center">
			<div>
				<div class="mx-auto w-32">
					<img src="/undraw_greek_freak_j-2-qb.svg" alt="The Greek Freak illustration" />
				</div>
				<p class="mt-8 text-center text-2xl">Awesome!!</p>
				<p class="mt-4 text-center">
					You'll receive an email and/or text message with details about signing up and drafting.
				</p>
			</div>
		</div>
	</div>
</div>

<svelte:head>
	<title>Success! - Jollof League</title>
	<meta
		name="description"
		content="You have successfully signed up for the jollof fantasy basketball league."
	/>
</svelte:head>
