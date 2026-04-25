<script lang="ts">
	import '../app.css';
	import { currentSection, type NavigationSection } from '$lib/navigation';
	import { scanStore } from '$lib/scanStore';
	import { 
		Shield, 
		Network, 
		AlertTriangle, 
		Settings, 
		Activity,
		FileText,
		Server,
		Lock,
		CheckCircle,
		XCircle,
		Loader
	} from 'lucide-svelte';

	const navItems: Array<{ id: NavigationSection; label: string; icon: typeof Activity }> = [
		{ id: 'dashboard', label: 'Dashboard', icon: Activity },
		{ id: 'services', label: 'Services', icon: Server },
		{ id: 'policies', label: 'Policies', icon: Lock },
		{ id: 'mesh', label: 'Mesh', icon: Network },
		{ id: 'attacks', label: 'Attacks', icon: AlertTriangle },
		{ id: 'audit', label: 'Audit', icon: FileText },
		{ id: 'settings', label: 'Settings', icon: Settings },
	];

	function formatRelativeTime(isoString: string | null): string {
		if (!isoString) return 'Never';
		const diff = Math.floor((Date.now() - new Date(isoString).getTime()) / 1000);
		if (diff < 60) return `${diff}s ago`;
		if (diff < 3600) return `${Math.floor(diff / 60)}m ago`;
		return `${Math.floor(diff / 3600)}h ago`;
	}
</script>

<div class="flex h-screen bg-slate-900">
	<!-- Sidebar -->
	<aside class="w-64 bg-slate-800 border-r border-slate-700 flex flex-col">
		<!-- Logo -->
		<div class="p-4 border-b border-slate-700">
			<div class="flex items-center gap-3">
				<Shield class="w-8 h-8 text-blue-500" />
				<div>
					<h1 class="text-lg font-bold text-slate-100">ZeroTrust Mesh</h1>
					<p class="text-xs text-slate-400">v0.1.0</p>
				</div>
			</div>
		</div>
		
		<!-- Navigation -->
		<nav class="flex-1 p-4">
			<ul class="space-y-1">
				{#each navItems as item}
					<li>
						<button
							on:click={() => currentSection.set(item.id)}
							class="w-full flex items-center gap-3 px-3 py-2 rounded-lg transition-colors {$currentSection === item.id ? 'bg-blue-600 text-white' : 'text-slate-300 hover:bg-slate-700'}"
						>
							<svelte:component this={item.icon} class="w-5 h-5" />
							<span>{item.label}</span>
						</button>
					</li>
				{/each}
			</ul>
		</nav>
		
		<!-- Live Scan Status Footer -->
		<div class="p-4 border-t border-slate-700 space-y-2">
			{#if $scanStore.isScanning}
				<div class="flex items-center gap-2 text-blue-400">
					<Loader class="w-4 h-4 animate-spin" />
					<span class="text-xs font-medium">Scanning services…</span>
				</div>
			{:else if $scanStore.lastScanSummary}
				<div class="flex items-center gap-2">
					{#if $scanStore.lastScanSummary.failed > 0}
						<XCircle class="w-4 h-4 text-red-400 flex-shrink-0" />
					{:else}
						<CheckCircle class="w-4 h-4 text-green-400 flex-shrink-0" />
					{/if}
					<div class="min-w-0">
						<p class="text-xs font-medium text-slate-200 leading-tight">
							{$scanStore.lastScanSummary.passed} passed
							{#if $scanStore.lastScanSummary.failed > 0}
								· <span class="text-red-400">{$scanStore.lastScanSummary.failed} failed</span>
							{/if}
						</p>
						<p class="text-xs text-slate-500">
							{formatRelativeTime($scanStore.lastScanTime)}
						</p>
					</div>
				</div>
			{:else}
				<div class="flex items-center gap-2">
					<div class="w-2 h-2 rounded-full bg-slate-500 flex-shrink-0"></div>
					<span class="text-xs text-slate-400">No scan run yet</span>
				</div>
			{/if}
			<div class="flex items-center gap-2">
				<div class="w-2 h-2 rounded-full bg-green-500 flex-shrink-0"></div>
				<span class="text-xs text-slate-500">Desktop runtime active</span>
			</div>
		</div>
	</aside>
	
	<!-- Main content -->
	<main class="flex-1 overflow-auto">
		<slot />
	</main>
</div>
