<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { t } from 'svelte-i18n';
	import { LayoutDashboard, User, Lock, Link as LinkIcon } from '@lucide/svelte';
	import { CMD_LOGIN } from '$lib/commands.svelte';
	import InstanceTypeToggle from '$lib/components/auth/InstanceTypeToggle.svelte';
	import AuthInput from '$lib/components/auth/AuthInput.svelte';
	import { setCurrentUser } from '$lib/stores/user.svelte';
	import type { User as UserType } from '$lib/types';

	let instanceType = $state<'cloud' | 'self'>('cloud');
	let customUrl = $state('');
	let username = $state('');
	let password = $state('');
	let loading = $state(false);
	let errorMsg = $state<string | null>(null);

	let { onLoginSuccess } = $props<{ onLoginSuccess: () => void }>();

	async function handleLogin() {
		loading = true;
		errorMsg = null;

		const apiUrl = instanceType === 'cloud' ? 'https://api.taiga.io' : customUrl.replace(/\/$/, '');

		try {
			const user = await invoke<UserType>(CMD_LOGIN, {
				apiUrl,
				username,
				password
			});
			setCurrentUser(user);
			onLoginSuccess();
		} catch (err) {
			console.error('Login failed:', err);
			if (err && typeof err === 'object' && 'TaigaClient' in err) {
				errorMsg = `${$t('errors.prefix')} ${err.TaigaClient}`;
			} else {
				errorMsg = $t('errors.unknown');
			}
		} finally {
			loading = false;
		}
	}
</script>

<div class="relative z-10 flex h-full w-full grow flex-col items-center justify-center p-4 sm:p-8">
	<div class="flex w-full max-w-[440px] flex-col gap-6">
		<div class="flex flex-col items-center gap-4 text-center">
			<div
				class="flex h-12 w-12 items-center justify-center rounded-xl bg-gradient-to-br from-[var(--login-primary)] to-blue-600"
			>
				<LayoutDashboard class="h-7 w-7 text-white" />
			</div>
			<div class="flex flex-col gap-1">
				<h1 class="text-2xl font-semibold tracking-tight text-gray-900 dark:text-white">
					{$t('login.welcomeBack')}
				</h1>
				<p class="text-sm text-gray-500 dark:text-[var(--login-muted)]">
					{$t('login.signInSubtitle')}
				</p>
			</div>
		</div>

		<div
			class="rounded-xl border border-gray-200 bg-white p-6 shadow-xl shadow-black/5 dark:border-[var(--login-border)] dark:bg-[#161e2a] dark:shadow-black/20"
		>
			<form
				class="flex flex-col gap-5"
				onsubmit={(e) => {
					e.preventDefault();
					handleLogin();
				}}
			>
				<InstanceTypeToggle bind:value={instanceType} />

				<div class="flex flex-col gap-4">
					<div class="flex flex-col gap-1.5">
						<label class="text-sm font-medium text-gray-700 dark:text-gray-200" for="username">
							{$t('login.usernameOrEmail')}
						</label>
						<AuthInput
							id="username"
							bind:value={username}
							placeholder={$t('login.usernamePlaceholder')}
							icon={User}
							disabled={loading}
							required
						/>
					</div>

					<div class="flex flex-col gap-1.5">
						<div class="flex items-center justify-between">
							<label class="text-sm font-medium text-gray-700 dark:text-gray-200" for="password">
								{$t('login.passwordLabel')}
							</label>
						</div>
						<AuthInput
							id="password"
							type="password"
							bind:value={password}
							placeholder={$t('login.passwordPlaceholder')}
							icon={Lock}
							disabled={loading}
							required
						/>
					</div>

					{#if instanceType === 'self'}
						<div
							class="animate-in fade-in slide-in-from-top-2 flex flex-col gap-1.5 pt-2 duration-300"
						>
							<label
								class="text-sm font-medium text-gray-700 dark:text-gray-200"
								for="instance-url"
							>
								{$t('login.instanceUrl')}
							</label>
							<AuthInput
								id="instance-url"
								type="url"
								bind:value={customUrl}
								placeholder={$t('login.instanceUrlPlaceholder')}
								icon={LinkIcon}
								disabled={loading}
								required={instanceType === 'self'}
							/>
							<p class="text-xs text-gray-500 dark:text-[var(--login-muted)]">
								{$t('login.instanceUrlHint')}
							</p>
						</div>
					{/if}

					{#if errorMsg}
						<div
							class="rounded-lg bg-red-50 p-3 text-sm text-red-500 dark:bg-red-900/20 dark:text-red-400"
						>
							{errorMsg}
						</div>
					{/if}

					<button
						class="mt-2 flex w-full items-center justify-center rounded-lg bg-[var(--login-primary)] py-2.5 text-sm font-medium text-white transition-all hover:bg-blue-600 focus:ring-2 focus:ring-[var(--login-primary)] focus:ring-offset-2 focus:outline-none disabled:cursor-not-allowed disabled:opacity-50 dark:focus:ring-offset-[#161e2a]"
						disabled={loading}
					>
						{#if loading}
							<svg
								class="mr-2 -ml-1 h-4 w-4 animate-spin text-white"
								xmlns="http://www.w3.org/2000/svg"
								fill="none"
								viewBox="0 0 24 24"
							>
								<circle
									class="opacity-25"
									cx="12"
									cy="12"
									r="10"
									stroke="currentColor"
									stroke-width="4"
								></circle>
								<path
									class="opacity-75"
									fill="currentColor"
									d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
								></path>
							</svg>
						{/if}
						{$t('login.signInButton')}
					</button>
				</div>
			</form>
		</div>

		<div class="flex items-center justify-center gap-4 text-xs text-gray-400 dark:text-[#4d5e75]">
			<span class="flex items-center gap-1">
				{$t('login.poweredBy')}
			</span>
		</div>
	</div>
</div>
