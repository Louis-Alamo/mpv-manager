<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import * as fileDiaog from "@tauri-apps/plugin-dialog";
    import type { VideoMetadata } from "../../lib/types";

    let {
        onCancel,
        onSave,
    }: {
        onCancel: () => void;
        onSave: (
            video: VideoMetadata,
            name: string,
            type: string,
            year: string,
            coverPath: string | null,
        ) => Promise<void>;
    } = $props();

    let name = $state("");
    let type = $state("");
    let year = $state("");
    let videoData = $state<VideoMetadata | null>(null);
    let pathImage = $state("");

    let canSave = $derived(
        name != "" && type != "" && year != "" && videoData != null,
    );

    async function getVideoMetadata(): Promise<void> {
        try {
            let selected = await fileDiaog.open({
                multiple: false,
                directory: false,
                title: "Select file",
                filters: [
                    {
                        name: "videos",
                        extensions: ["mp4", "mkv", "avi", "mov"],
                    },
                ],
            });

            if (!selected || Array.isArray(selected)) {
                return;
            }

            videoData = await invoke<VideoMetadata>("get_video_metadata", {
                path: selected,
            });

            name = videoData.filename;
        } catch (err) {
            console.error("Error al invocar Rust:", err);
        }
    }

    async function getPathImage(): Promise<void> {
        try {
            let selected = await fileDiaog.open({
                multiple: false,
                directory: false,
                title: "Select imagen...",
                filters: [
                    {
                        name: "images",
                        extensions: ["jpg", "png"],
                    },
                ],
            });

            if (!selected || Array.isArray(selected)) {
                return;
            }

            pathImage = selected;
        } catch (err) {
            console.log("ImportModal -> Error -> ", err);
        }
    }

    async function handleSave(): Promise<void> {
        if (!videoData) return;
        await onSave(videoData, name, type, year, pathImage || null);
    }
</script>

<div class="overlay">
    <div class="modal">
        <div class="modal-header">
            <h2>Import video</h2>
            <button class="close-btn" onclick={onCancel}>✕</button>
        </div>

        <div class="modal-body">
            <div class="field">
                <label for="vname">Name</label>
                <input
                    id="vname"
                    type="text"
                    bind:value={name}
                    placeholder="Video name..."
                />
            </div>

            <fieldset class="field fieldset">
                <legend>Type</legend>
                <div class="radio-group">
                    <label class="radio-opt">
                        <input
                            type="radio"
                            name="type"
                            value="movie"
                            bind:group={type}
                        /> Movie
                    </label>
                    <label class="radio-opt">
                        <input
                            type="radio"
                            name="type"
                            value="series"
                            bind:group={type}
                        /> Series
                    </label>
                    <label class="radio-opt">
                        <input
                            type="radio"
                            name="type"
                            value="other"
                            bind:group={type}
                        /> Other
                    </label>
                </div>
            </fieldset>

            <div class="field">
                <label for="vyear">Year</label>
                <input
                    id="vyear"
                    type="number"
                    placeholder="2024"
                    min="1900"
                    max="2099"
                    bind:value={year}
                />
            </div>

            <div class="field">
                <label for="vcover">Cover image</label>
                <button
                    id="vcover"
                    type="button"
                    class="file-btn"
                    onclick={getPathImage}
                >
                    {pathImage ? pathImage : "Select file..."}
                </button>
            </div>

            <div class="field">
                <label for="vfile">File</label>
                <button
                    id="vfile"
                    type="button"
                    class="file-btn"
                    onclick={getVideoMetadata}
                >
                    {videoData ? videoData.filename : "Select file..."}
                </button>
            </div>
        </div>

        <div class="modal-footer">
            <button class="btn-cancel" onclick={onCancel}>Cancel</button>
            <button class="btn-save" disabled={!canSave} onclick={handleSave}
                >Save</button
            >
        </div>
    </div>
</div>

<style>
    .overlay {
        position: fixed;
        inset: 0;
        background: rgba(0, 0, 0, 0.35);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 50;
    }

    .modal {
        background: #fff;
        border-radius: 14px;
        width: 100%;
        max-width: 400px;
        border: 0.5px solid #e0ddd6;
    }

    .modal-header {
        padding: 1rem 1.25rem;
        border-bottom: 0.5px solid #e0ddd6;
        display: flex;
        align-items: center;
        justify-content: space-between;
    }

    .modal-header h2 {
        font-size: 0.9rem;
        font-weight: 600;
        color: #1a1a1a;
        margin: 0;
    }

    .close-btn {
        background: none;
        border: none;
        font-size: 1rem;
        color: #aaa;
        cursor: pointer;
    }

    .close-btn:hover {
        color: #555;
    }

    .modal-body {
        padding: 1.25rem;
        display: flex;
        flex-direction: column;
        gap: 1rem;
    }

    .field {
        display: flex;
        flex-direction: column;
        gap: 5px;
    }

    label,
    legend {
        font-size: 0.75rem;
        font-weight: 500;
        color: #555;
    }

    .fieldset {
        border: 0;
        padding: 0;
        margin: 0;
    }

    input[type="text"],
    input[type="number"] {
        padding: 0.4rem 0.75rem;
        font-size: 0.8rem;
        border: 0.5px solid #d0cdc6;
        border-radius: 8px;
        background: #fff;
        color: #1a1a1a;
        font-family: inherit;
        outline: none;
        transition: border-color 0.15s;
    }

    input:focus {
        border-color: #e05a00;
    }

    .radio-group {
        display: flex;
        gap: 8px;
    }

    .radio-opt {
        display: flex;
        align-items: center;
        gap: 5px;
        font-size: 0.8rem;
        color: #555;
        cursor: pointer;
    }

    .file-btn {
        padding: 0.4rem 0.85rem;
        font-size: 0.8rem;
        font-family: inherit;
        background: #fff;
        border: 0.5px solid #d0cdc6;
        border-radius: 8px;
        color: #555;
        cursor: pointer;
        text-align: left;
        transition: background 0.15s;
    }

    .file-btn:hover {
        background: #f5f5f3;
    }

    .modal-footer {
        padding: 1rem 1.25rem;
        border-top: 0.5px solid #e0ddd6;
        display: flex;
        gap: 8px;
        justify-content: flex-end;
    }

    .btn-cancel {
        padding: 0.4rem 1rem;
        font-size: 0.8rem;
        font-family: inherit;
        background: #fff;
        border: 0.5px solid #d0cdc6;
        border-radius: 8px;
        color: #555;
        cursor: pointer;
        transition: background 0.15s;
    }

    .btn-cancel:hover {
        background: #f5f5f3;
    }

    .btn-save {
        padding: 0.4rem 1rem;
        font-size: 0.8rem;
        font-family: inherit;
        font-weight: 500;
        background: #e05a00;
        border: none;
        border-radius: 8px;
        color: #fff;
        cursor: pointer;
        opacity: 0.4;
    }

    .btn-save:not([disabled]) {
        opacity: 1;
        cursor: pointer;
    }
</style>
