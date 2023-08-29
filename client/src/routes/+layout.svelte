<script>
	import '../app.css';
</script>

<div class="min-h-screen md:h-screen md:min-h-0">
	<slot />
	<footer class="flex h-20 items-center justify-center text-sm font-light">
		<div class="text-center">
			<p>&#169; {new Date().getFullYear()} Ahmed Alhassan. All Rights Reserved.</p>
			<div class="mt-2">
				<a href="/privacy" class="text-weldon-blue mr-2">Privacy Policy</a>
				<a href="/terms-conditions" class="text-weldon-blue">Terms and Conditions</a>
			</div>
		</div>
	</footer>
</div>
