# MPV Manager App - Project Information

## 📋 Overview

**MPV Manager App** is a cross-platform desktop application built with **Tauri**, **Svelte**, and **TypeScript**. It provides a modern interface to manage and configure the MPV media player, including library management, GPU upscaling (shaders), and MPV settings configuration.

**Author**: ElPanadero  
**Version**: 0.1.0 (v1.0.0-dev)  
**Type**: Desktop Application  
**License**: (Not specified)

---

## 🏗️ Project Architecture

### Tech Stack

#### Frontend
- **Framework**: Svelte 5.55.1
- **Language**: TypeScript 6.0.2
- **Build Tool**: Vite 8.0.4
- **Plugin**: @sveltejs/vite-plugin-svelte 7.0.0
- **Package Manager**: npm 11.12.1
- **Node Version**: v25.9.0

#### Backend
- **Desktop Framework**: Tauri 2.10.3
- **Language**: Rust 1.94.1 (MSRV: 1.77.2)
- **Cargo**: 1.94.1
- **Logging**: tauri-plugin-log 2 (debug mode only)
- **Serialization**: serde + serde_json

### Directory Structure

```
mpv-manager-app/
├── src/                          # Frontend (Svelte + TypeScript)
│   ├── components/               # Reusable Svelte components
│   ├── view/                     # View components (Library, Shaders, Settings)
│   ├── lib/                      # Utility functions & stores
│   ├── assets/                   # Static assets
│   ├── App.svelte                # Main app component with routing
│   ├── app.css                   # Global styles
│   └── main.ts                   # Entry point
├── src-tauri/                    # Backend (Rust + Tauri)
│   ├── src/
│   │   ├── main.rs               # Tauri app entry point
│   │   ├── lib.rs                # Tauri setup & command handlers
│   │   └── video_handler.rs      # Video metadata commands
│   ├── tauri.conf.json           # Tauri configuration
│   ├── Cargo.toml                # Rust dependencies
│   ├── Cargo.lock
│   ├── build.rs
│   └── icons/                    # App icons
├── public/                       # Static public assets
├── .vscode/                      # VS Code configuration
├── index.html                    # HTML entry point
├── package.json                  # npm dependencies
├── package-lock.json
├── vite.config.ts                # Vite configuration
├── svelte.config.js              # Svelte configuration
├── tsconfig.json                 # TypeScript root config
├── tsconfig.app.json             # Frontend TypeScript config
├── tsconfig.node.json            # Build tools TypeScript config
└── README.md                     # Original template README

```

---

## 🎨 Frontend Architecture

### Main Views (Navigation)
The app uses a sidebar navigation with three main views:

1. **Library** (📁)
   - Component: `Library.svelte`
   - Purpose: Manage and browse video library

2. **Upscaling** (🚀)
   - Purpose: Configure GPU shaders for AI upscaling

3. **MPV Settings** (⚙️)
   - Purpose: Configure MPV player settings

### Design System
- **Color Scheme**: 
  - Primary: `#e05a00` (Orange)
  - Background: `#f5f5f3` (Beige)
  - Text: `#1a1a1a` (Dark)
  - Accents: `#ffffff`, `#777`, `#aaa`
- **Typography**: Inter, system sans-serif
- **Layout**: Flexbox-based responsive design
- **Sidebar Width**: 220px fixed

### Current State
- ✅ Navigation structure implemented
- ✅ Sidebar styling complete
- ❌ Library view not yet detailed
- ❌ Upscaling configuration placeholder
- ❌ MPV Settings placeholder
- ⚠️ Frontend-Backend integration not yet active

---

## 🦀 Backend Architecture (Tauri + Rust)

### Tauri Configuration (`tauri.conf.json`)

| Setting | Value |
|---------|-------|
| Product Name | mpv-manager-app |
| Window Title | mpv-manager-app |
| Window Size | 800x600 (resizable) |
| Frontend Dist | `../public` |
| Dev URL | `http://localhost:5174` |
| Frontend Dev Command | `npm run dev` |
| Frontend Build Command | `npm run build` |
| Security CSP | null (disabled) |

### Current Commands

#### `get_video_metadata()`
- **Module**: `video_handler.rs`
- **Return Type**: `String`
- **Current Implementation**: Returns dummy string "Hola desde rust"
- **Purpose**: Fetch video metadata (intended for real implementation)

### Logging
- **Plugin**: tauri-plugin-log v2
- **Active In**: Debug builds only
- **Level**: Info

---

## 📦 Dependencies

### Frontend Dependencies
```json
{
  "@tauri-apps/api": "^2.10.1"
}
```

### Frontend Dev Dependencies
```json
{
  "@sveltejs/vite-plugin-svelte": "^7.0.0",
  "@tauri-apps/cli": "^2.10.1",
  "@tsconfig/svelte": "^5.0.8",
  "@types/node": "^24.12.2",
  "svelte": "^5.55.1",
  "svelte-check": "^4.4.6",
  "typescript": "~6.0.2",
  "vite": "^8.0.4"
}
```

### Backend Dependencies (Rust)
```toml
serde_json = "1.0"
serde = { version = "1.0", features = ["derive"] }
log = "0.4"
tauri = { version = "2.10.3" }
tauri-plugin-log = "2"
```

### Build Dependencies (Rust)
```toml
tauri-build = { version = "2.5.6" }
```

---

## 🚀 Available Scripts

| Script | Command | Purpose |
|--------|---------|---------|
| `dev` | `vite` | Start frontend dev server (HMR enabled) |
| `build` | `vite build` | Build frontend for production |
| `preview` | `vite preview` | Preview production build locally |
| `check` | `svelte-check && tsc -p tsconfig.node.json` | Type checking & linting |

### Running the Full App
To run the complete desktop app in development:
```bash
npm run tauri dev  # (via Tauri CLI, not shown in package.json)
```

This will:
1. Start the frontend dev server on `http://localhost:5174`
2. Launch the Tauri desktop window pointing to that URL
3. Enable hot reload for frontend changes

---

## ⚠️ Known Issues & Notes

### Build Configuration
1. **Frontend Dist Path**: Currently set to `../public` in `tauri.conf.json`
   - ⚠️ Vite typically outputs to `dist/`, not `public/`
   - This may cause issues on release builds
   - **Action Needed**: Verify actual build output location

2. **Dev Port Mismatch**: 
   - `tauri.conf.json` specifies `devUrl: "http://localhost:5174"`
   - Default Vite port is 5173
   - **Action Needed**: Confirm Vite configuration forces port 5174 or update config

### Integration
- ⚠️ Frontend is not yet invoking Rust commands
- ⚠️ `@tauri-apps/api` is installed but not imported/used
- ⚠️ Video metadata function returns hardcoded string

### Git
- 🔍 `node_modules/` directory present in repo (should be in `.gitignore`)

---

## 📋 Development Checklist

### Short Term
- [ ] Verify Vite dev server port configuration
- [ ] Fix `frontendDist` path in `tauri.conf.json`
- [ ] Test Tauri dev build workflow
- [ ] Implement Tauri command invocation in `Library.svelte`
- [ ] Replace dummy `get_video_metadata()` implementation

### Medium Term
- [ ] Implement video library scanning & management
- [ ] Implement GPU shader configuration UI
- [ ] Implement MPV settings management
- [ ] Add error handling & logging throughout
- [ ] Create test suite for Rust commands

### Long Term
- [ ] MPV integration (IPC/socket communication)
- [ ] FFprobe integration for metadata extraction
- [ ] User preferences persistence
- [ ] Release builds for macOS, Windows, Linux
- [ ] Auto-update mechanism

---

## 🛠️ Development Setup

### Prerequisites
- **Node.js**: v20+ (tested with v25.9.0)
- **npm**: v11.12.1+
- **Rust**: 1.77.2+ (tested with 1.94.1)
- **Cargo**: Latest stable

### Getting Started
```bash
# 1. Install dependencies
npm install

# 2. Verify TypeScript compilation
npm run check

# 3. Run in development mode
npm run tauri dev

# 4. Build for release
npm run tauri build
```

---

## 📝 IDE Recommendations

- **IDE**: VS Code
- **Extensions**:
  - [Svelte for VS Code](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode)
  - Rust Analyzer (for Tauri backend)
  - TypeScript Vue Plugin

---

## 📄 License & Attribution

**Created by**: ElPanadero  
**License**: (Not specified - recommend adding LICENSE file)

---

## 📚 References

- [Tauri v2 Documentation](https://tauri.app/)
- [Svelte Documentation](https://svelte.dev/)
- [Vite Documentation](https://vitejs.dev/)
- [Rust Book](https://doc.rust-lang.org/book/)

---

**Last Updated**: 2025-01-XX  
**Status**: Early Development (v0.1.0)