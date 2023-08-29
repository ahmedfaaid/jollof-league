<script lang="ts">
	// @ts-nocheck

	import { enhance } from '$app/forms';
	export let form;
</script>

<div class="flex w-full flex-col sm:h-[calc(100vh-80px)] md:flex-row">
	<div
		class="bg-columbia-blue w-full bg-[url('/undraw_basketball_re_7701.svg?url')] bg-contain bg-right-bottom bg-no-repeat md:w-1/2 md:bg-[left_top_20rem]"
	>
		<div
			class="flex h-full w-full items-center justify-center bg-white/70 px-8 py-8 sm:px-16 md:py-0"
		>
			<div>
				<p class="mb-4 text-2xl font-semibold leading-tight lg:text-3xl">
					Are you interested in joining the <span>Jollof Fantasy League</span>?
				</p>
				<p class="mb-4 text-xl lg:text-2xl">Leave your details in the form.</p>
				<p class="lg:text-lg">We'll contact you with instructions and a link to join the league</p>
			</div>
		</div>
	</div>
	<div class="flex w-full items-center justify-center px-8 py-8 sm:px-16 md:w-1/2 md:py-0">
		<div class="w-full">
			<h2 class="mb-8 text-lg font-semibold lg:text-xl">Sign up here</h2>
			<form method="POST" use:enhance>
				<div class="mb-4">
					{#if form?.error}
						<p class="text-red-500">{form?.message}</p>
					{/if}
				</div>
				<div class="form-control mb-4">
					<label
						for="first_name"
						class={`mb-2 block text-sm font-thin lg:text-base ${
							form?.error && form?.field === 'first_name' ? 'text-red-500' : 'text-dark-cerulean'
						}`}>First Name</label
					>
					<input
						type="text"
						name="first_name"
						id="first_name"
						placeholder="Enter your first name"
						class="form-input placeholder:text-cadet-blue border-cadet-blue focus:border-dark-cerulean text-dark-cerulean w-full rounded border-2 text-sm placeholder:font-thin focus:outline-none lg:text-base"
						value={form?.first_name ?? ''}
					/>
				</div>
				<div class="form-control mb-4">
					<label
						for="last_name"
						class={`text-dark-cerulean mb-2 block text-sm font-thin lg:text-base ${
							form?.error && form?.field === 'last_name' ? 'text-red-500' : 'text-dark-cerulean'
						}`}>Last Name</label
					>
					<input
						type="text"
						name="last_name"
						id="last_name"
						placeholder="Enter your last name"
						class="form-input text-dark-cerulean placeholder:text-cadet-blue border-cadet-blue focus:border-dark-cerulean w-full rounded border-2 text-sm placeholder:font-thin focus:outline-none lg:text-base"
						value={form?.last_name ?? ''}
					/>
				</div>
				<div class="form-control mb-4">
					<label
						for="email"
						class={`mb-2 block text-sm font-thin lg:text-base ${
							form?.error && form?.field === 'email' ? 'text-red-500' : 'text-dark-cerulean'
						}`}>Email</label
					>
					<input
						type="email"
						name="email"
						id="email"
						placeholder="Enter your email"
						class="form-input placeholder:text-cadet-blue border-cadet-blue focus:border-dark-cerulean text-dark-cerulean w-full rounded border-2 text-sm placeholder:font-thin focus:outline-none lg:text-base"
						value={form?.email ?? ''}
					/>
				</div>
				<div class="form-control mb-8">
					<label
						for="phone_number"
						class={`mb-2 block text-sm font-thin lg:text-base ${
							form?.error && form?.field === 'phone_number' ? 'text-red-500' : 'text-dark-cerulean'
						}`}>Phone Number</label
					>
					<input
						type="tel"
						name="phone_number"
						id="phone_number"
						placeholder="416-555-5555"
						class="form-input placeholder:text-cadet-blue border-cadet-blue focus:border-dark-cerulean text-dark-cerulean w-full rounded border-2 text-sm placeholder:font-thin focus:outline-none lg:text-base"
						value={form?.phone_number ?? ''}
					/>
				</div>
				<button
					class="bg-dark-cerulean hover:bg-queen-blue w-full rounded p-2 text-sm text-white lg:text-base"
					>Sign up</button
				>
			</form>
		</div>
	</div>
</div>
