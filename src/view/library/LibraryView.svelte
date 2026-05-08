<script lang="ts">
    import girlsUndPanzerImg from "../../assets/girls-und-panzer-portada.jpg";
    import cyberpunkImg from "../../assets/cyberpunk-edgerunners-portada.jpg";
    import ImportModal from "./ImportModal.svelte";

    import type { VideoMetadata } from "../../lib/types";

    import { invoke } from "@tauri-apps/api/core";

    interface Video {
        name: string;
        path: string;
        size: number;
        format: string;
        duration: number;
        image: string;
    }

    let videos: Video[] = [
        {
            name: "Girls und Panzer",
            path: "/path/to/video1",
            size: 100,
            format: "mp4",
            duration: 100,
            image: girlsUndPanzerImg,
        },
        {
            name: "Cyberpunk Edgerunners",
            path: "/path/to/video2",
            size: 200,
            format: "mp4",
            duration: 200,
            image: cyberpunkImg,
        },
    ];

    let showModal = $state(false);

    let searchQuery: string = $state("");

    let filteredVideos = $derived(
        videos.filter((video) => {
            const query = searchQuery.toLowerCase();

            const matchesName = video.name.toLowerCase().includes(query);
            const matchesFormat = video.format.toLowerCase().includes(query);
            const matchesPath = video.path.toLowerCase().includes(query);

            return matchesName || matchesFormat || matchesPath;
        }),
    );

    async function handleSave(
        video: VideoMetadata,
        name: string,
        type: string,
        year: string,
        coverPath: string | null,
    ) {
        await invoke("save_video", {
            video: $state.snapshot(video),
            name,
            type,
            year: parseInt(year),
            cover_path: coverPath,
        });
        showModal = false;
    }
</script>

{#if showModal}
    <ImportModal onCancel={() => (showModal = false)} onSave={handleSave} />
{/if}

<section class="collection">
    <div class="nav-top">
        <span class="section-title">My Collection</span>
        <input type="text" placeholder="Search..." bind:value={searchQuery} />
        <button class="import-btn" onclick={() => (showModal = true)}
            >+ Import</button
        >
    </div>

    <div class="grid">
        {#each filteredVideos as video}
            <div class="card">
                <img src={video.image} alt={video.name} />
                <div class="card-body">
                    <p class="card-title">{video.name}</p>
                    <div class="meta">
                        <div class="meta-row">
                            <span class="label">Tamaño</span>
                            <span class="value">{video.size} MB</span>
                        </div>
                        <div class="meta-row">
                            <span class="label">Duración</span>
                            <span class="value">{video.duration} min</span>
                        </div>
                        <div class="meta-row">
                            <span class="label">Formato</span>
                            <span class="badge">{video.format}</span>
                        </div>
                    </div>
                    <p class="path">{video.path}</p>
                </div>
            </div>
        {/each}
    </div>

    {#if filteredVideos.length === 0}
        <p style="text-align: center; color: #888; margin-top: 2rem;">
            No videos found.
        </p>
    {/if}
</section>

<style>
    .collection {
        padding: 1rem 0;
    }

    .nav-top {
        display: flex;
        align-items: center;
        gap: 0.75rem;
        margin-bottom: 1.25rem;
    }

    .section-title {
        font-size: 1rem;
        font-weight: 600;
        color: #1a1a1a;
        white-space: nowrap;
        margin-right: auto;
    }

    .nav-top input {
        padding: 0.4rem 0.75rem;
        font-size: 0.8rem;
        border: 0.5px solid #d0cdc6;
        border-radius: 8px;
        background: #ffffff;
        color: #1a1a1a;
        outline: none;
        width: 180px;
        font-family: inherit;
        transition: border-color 0.15s;
    }

    .nav-top input:focus {
        border-color: #e05a00;
    }

    .import-btn {
        padding: 0.4rem 0.85rem;
        font-size: 0.8rem;
        font-family: inherit;
        font-weight: 500;
        background: #e05a00;
        color: #ffffff;
        border: none;
        border-radius: 8px;
        cursor: pointer;
        white-space: nowrap;
        transition: background 0.15s;
    }

    .import-btn:hover {
        background: #c44f00;
    }

    /* ── grid ── */
    .grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
        gap: 1rem;
    }

    /* ── card ── */
    .card {
        background: #ffffff;
        border: 0.5px solid #e0ddd6;
        border-radius: 12px;
        overflow: hidden;
        box-shadow: 0 1px 3px rgba(0, 0, 0, 0.06);
        transition:
            transform 0.15s ease,
            box-shadow 0.15s ease,
            border-color 0.15s;
        cursor: pointer;
    }

    .card:hover {
        transform: translateY(-3px);
        box-shadow: 0 6px 16px rgba(0, 0, 0, 0.1);
        border-color: #c0bdb6;
    }

    .card img {
        width: 100%;
        aspect-ratio: 16 / 9;
        object-fit: cover;
        display: block;
    }

    .card-body {
        padding: 12px;
    }

    .card-title {
        font-size: 0.875rem;
        font-weight: 500;
        margin: 0 0 8px;
        color: #1a1a1a;
    }

    .meta {
        display: flex;
        flex-direction: column;
        gap: 4px;
    }

    .meta-row {
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .label {
        font-size: 0.75rem;
        color: #888;
    }

    .value {
        font-size: 0.75rem;
        font-weight: 500;
        color: #1a1a1a;
    }

    .badge {
        font-size: 0.6875rem;
        padding: 2px 8px;
        border-radius: 999px;
        background: #fff4ee;
        color: #e05a00;
        font-weight: 500;
    }

    .path {
        font-size: 0.6875rem;
        color: #aaa;
        margin: 6px 0 0;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
    }
</style>
