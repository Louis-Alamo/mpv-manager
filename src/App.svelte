<script lang="ts">
    import Library from "./view/library/LibraryView.svelte";
    import UpscalingView from "./view/UpscalingView.svelte";

    interface NavItem {
        id: "library" | "shaders" | "settings";
        label: string;
        icon: string;
    }

    let currentView: NavItem["id"] = "library";

    const navItems: NavItem[] = [
        { id: "library", label: "Library", icon: "📁" },
        { id: "shaders", label: "Upscaling", icon: "🚀" },
        { id: "settings", label: "MPV Settings", icon: "⚙️" },
    ];

    const setView = (viewId: NavItem["id"]): void => {
        currentView = viewId;
    };
</script>

<div class="app-wrapper">
    <aside class="sidebar">
        <div class="brand">
            <h1>MPV manager</h1>
            <small>By ElPanadero</small>
        </div>

        <nav class="nav-menu">
            {#each navItems as item}
                <button
                    class:active={currentView === item.id}
                    onclick={() => setView(item.id)}
                    aria-label={item.label}
                >
                    <span class="icon">{item.icon}</span>
                    <span class="label">{item.label}</span>
                </button>
            {/each}
        </nav>

        <footer class="sidebar-footer">
            <code>v1.0.0-dev (ts)</code>
        </footer>
    </aside>

    <main class="main-content">
        {#if currentView === "library"}
            <section class="view-header">
                <Library />
            </section>
        {:else if currentView === "shaders"}
            <section class="view-header">
                <UpscalingView />
            </section>
        {:else if currentView === "settings"}
            <section class="view-header">
                <h2>MPV Settings</h2>
                <p>Configure your MPV player.</p>
            </section>
        {/if}
    </main>
</div>

<style>
    :global(body) {
        margin: 0;
        background-color: #f5f5f3;
        color: #1a1a1a;
        font-family:
            "Inter",
            -apple-system,
            sans-serif;
        overflow: hidden;
    }

    .app-wrapper {
        display: flex;
        width: 100vw;
        height: 100vh;
    }

    .sidebar {
        width: 220px;
        background-color: #ffffff;
        border-right: 0.5px solid #e0ddd6;
        display: flex;
        flex-direction: column;
        padding: 1.25rem 1rem;
    }

    .brand {
        padding: 0 0.25rem 1.25rem;
        border-bottom: 0.5px solid #e0ddd6;
    }

    .brand h1 {
        font-size: 1rem;
        color: #e05a00;
        margin: 0;
    }

    .brand small {
        font-size: 0.7rem;
        color: #aaa;
    }

    .nav-menu {
        margin-top: 1rem;
        flex-grow: 1;
        display: flex;
        flex-direction: column;
        gap: 2px;
    }

    .nav-menu button {
        display: flex;
        align-items: center;
        gap: 0.6rem;
        background: transparent;
        border: none;
        color: #777;
        padding: 0.5rem 0.65rem;
        border-radius: 8px;
        cursor: pointer;
        text-align: left;
        font-size: 0.85rem;
        font-family: inherit;
        transition:
            background 0.15s,
            color 0.15s;
    }

    .nav-menu button:hover {
        background-color: #f5f5f3;
        color: #1a1a1a;
    }

    .nav-menu button.active {
        background-color: #fff4ee;
        color: #e05a00;
        font-weight: 500;
    }

    .main-content {
        flex-grow: 1;
        padding: 2rem 2.5rem;
        overflow-y: auto;
        background: #f5f5f3;
    }

    .view-header h2 {
        margin-top: 0;
        font-size: 1.25rem;
        font-weight: 600;
        color: #1a1a1a;
    }

    .view-header p {
        color: #888;
        font-size: 0.85rem;
    }

    .sidebar-footer {
        font-size: 0.7rem;
        color: #bbb;
        text-align: center;
        padding-top: 1rem;
        border-top: 0.5px solid #e0ddd6;
    }
</style>
