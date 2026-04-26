<script lang="ts">
    import modelo3dImagen from "../assets/modelado-3d.png";

    interface UpscalingModels {
        id: string;
        displayName: string;
        category: "Upscaling" | "Denoise" | "Deband" | "Sharpening";
        gpuWeight: "low" | "medium" | "high";
        contentType: "Anime" | "Live Action" | "CGI";
        path: string;
    }

    const testModels: UpscalingModels[] = [
        {
            id: "anime4k-v4-glsl",
            displayName: "Anime4K: Upscale (HQ)",
            category: "Upscaling",
            gpuWeight: "medium",
            contentType: "Anime",
            path: "/path/to/model1",
        },
        {
            id: "fsr-ultra-fast",
            displayName: "AMD FSR: Edge Sharpening",
            category: "Sharpening",
            gpuWeight: "low",
            contentType: "Live Action",
            path: "/path/to/model2",
        },
        {
            id: "nnedi3-heavy-resample",
            displayName: "NNEDI3: Neuronal Scaling",
            category: "Upscaling",
            gpuWeight: "high",
            contentType: "CGI",
            path: "/path/to/model3",
        },
    ];

    let searchQuery: string = $state("");

    let filteredModels = $derived(
        testModels.filter((model) => {
            const query = searchQuery.toLowerCase();

            const matchesName = model.displayName.toLowerCase().includes(query);

            const matchesCategory = model.category
                .toLowerCase()
                .includes(query);

            const matchesContentType = model.contentType
                .toLowerCase()
                .includes(query);

            return matchesName || matchesCategory || matchesContentType;
        }),
    );
</script>

<section class="collection">
    <div class="nav-top">
        <span class="section-title">Models</span>

        <input
            type="text"
            placeholder="Search by name, category, or type..."
            bind:value={searchQuery}
        />
        <button class="import-btn">+ Import</button>
    </div>

    <div class="grid">
        {#each filteredModels as model}
            <div class="card">
                <img src={modelo3dImagen} alt="Modelo" />
                <div class="card-body">
                    <p class="card-title">{model.displayName}</p>
                    <div class="meta">
                        <div class="meta-row">
                            <span class="label">Category</span>
                            <span class="value">{model.category}</span>
                        </div>
                        <div class="meta-row">
                            <span class="label">GPU Weight</span>
                            <span class="value weight-{model.gpuWeight}"
                                >{model.gpuWeight}</span
                            >
                        </div>
                        <div class="meta-row">
                            <span class="label">Content type</span>
                            <span class="value">{model.contentType}</span>
                        </div>
                    </div>
                    <p class="path">{model.path}</p>
                </div>
            </div>
        {/each}
    </div>

    {#if filteredModels.length === 0}
        <div class="no-results">
            <p>No models found matching "{searchQuery}"</p>
        </div>
    {/if}
</section>

<style>
    section {
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
        font-size: 0.65rem;
        color: #aaa;
        margin: 0;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
    }

    /* ── GPU Weight Colors ── */
    .weight-low {
        color: #22c55e;
        font-weight: 600;
    }

    .weight-medium {
        color: #eab308;
        font-weight: 600;
    }

    .weight-high {
        color: #ef4444;
        font-weight: 600;
    }

    /* ── No Results Message ── */
    .no-results {
        display: flex;
        justify-content: center;
        align-items: center;
        min-height: 200px;
        text-align: center;
    }

    .no-results p {
        font-size: 0.875rem;
        color: #888;
        margin: 0;
    }
</style>
