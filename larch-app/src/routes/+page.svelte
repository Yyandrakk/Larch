<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  // --- State ---
  let apiUrl = $state("https://api.taiga.io/api/v1");
  let username = $state("");
  let password = $state("");
  let error = $state<any>(null);
  let user = $state<any>(null);
  let isLoading = $state(false);

  // --- Actions ---
  async function login(event: Event) {
    event.preventDefault();
    isLoading = true;
    error = null;

    try {
      const result = await invoke("login", {
        apiUrl,
        username,
        password,
      });
      user = result;
    } catch (e) {
      error = e;
    } finally {
      isLoading = false;
    }
  }

  function logout() {
    user = null;
    username = "";
    password = "";
    error = null;
    invoke("logout"); // Fire and forget
  }
</script>

<main class="container">
  <h1>Larch - Taiga Client</h1>

  {#if user}
    <div class="user-card">
      <h2>Welcome, {user.full_name}</h2>
      <p>{user.email}</p>
      <button onclick={logout}>Logout</button>
    </div>
  {:else}
    <form class="login-form" onsubmit={login}>
      <div class="form-group">
        <label for="api-url">Taiga API URL</label>
        <input id="api-url" type="text" bind:value={apiUrl} required />
      </div>
      <div class="form-group">
        <label for="username">Username</label>
        <input id="username" type="text" bind:value={username} required />
      </div>
      <div class="form-group">
        <label for="password">Password</label>
        <input id="password" type="password" bind:value={password} required />
      </div>
      <button type="submit" disabled={isLoading}>
        {#if isLoading}Logging in...{:else}Login{/if}
      </button>
    </form>

    {#if error}
      <div class="error-box">
        <p><strong>Error:</strong></p>
        <pre>{JSON.stringify(error, null, 2)}</pre>
      </div>
    {/if}
  {/if}
</main>

<style>
  .container {
    margin: 0 auto;
    padding: 2rem;
    max-width: 500px;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  h1 {
    text-align: center;
  }

  .login-form, .user-card {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    padding: 2rem;
    border-radius: 8px;
    background-color: #f9f9f9;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  label {
    font-weight: 500;
  }

  input {
    padding: 0.8em 1em;
    border-radius: 4px;
    border: 1px solid #ccc;
    font-size: 1em;
  }

  button {
    padding: 0.8em 1.2em;
    border-radius: 4px;
    border: none;
    background-color: #396cd8;
    color: white;
    font-size: 1em;
    font-weight: 500;
    cursor: pointer;
    transition: background-color 0.25s;
  }

  button:hover {
    background-color: #2c5bbd;
  }

  button:disabled {
    background-color: #a0a0a0;
    cursor: not-allowed;
  }

  .error-box {
    margin-top: 1rem;
    padding: 1rem;
    border-radius: 4px;
    background-color: #ffebee;
    color: #c62828;
    border: 1px solid #c62828;
  }

  pre {
    white-space: pre-wrap;
    word-wrap: break-word;
  }

  @media (prefers-color-scheme: dark) {
    .login-form, .user-card {
      background-color: #2f2f2f;
      color: #f6f6f6;
    }

    input {
      background-color: #3a3a3a;
      border-color: #555;
      color: #f6f6f6;
    }

    .error-box {
      background-color: #4d2222;
      color: #ffcdd2;
      border-color: #ef9a9a;
    }
  }
</style>
