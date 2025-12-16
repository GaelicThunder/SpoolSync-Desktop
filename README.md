# SpoolSync Desktop

**Desktop companion for managing 3D printer filament profiles and syncing to Bambu Lab printers via MQTT.**

Built with Tauri v2 + Svelte 5 for maximum performance and minimal footprint (~8MB binary).

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Tauri](https://img.shields.io/badge/Tauri-v2.1-24C8DB?logo=tauri)
![Svelte](https://img.shields.io/badge/Svelte-v5-FF3E00?logo=svelte)
![Status](https://img.shields.io/badge/status-Alpha-orange)
![Platforms](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-informational)

---

## Overview

SpoolSync Desktop is a native desktop app that lets you:

- 🔍 **Browse filament profiles** from your SpoolmanDB instance
- ⭐ **Manage favorites** and local custom profiles
- 🧵 **Configure Bambu Lab AMS** slots from the desktop
- 🔌 **Sync profiles via MQTT** directly to your printer
- 🧩 **Work fully offline** for local profiles and settings

Color browsing via FilamentColors is planned but not implemented yet and is tracked in the roadmap.

---

## Current Feature Status

| Area | Status | Notes |
|------|--------|-------|
| Core layout & routing | ✅ | Sidebar, pages, loading & error handling |
| SQLite storage | ✅ | Profiles, favorites, custom profiles, settings |
| Favorites & custom profiles | ✅ | List, create, delete, basic validation |
| AMS sync page | ✅ | Basic AMS page and settings wiring |
| AMS settings in Settings | ✅ | AMS defaults stored in settings DB |
| MQTT client | 🚧 | rumqttc integration in progress |
| SpoolmanDB integration | 🚧 | Client, pagination, filters in progress |
| FilamentColors integration | ⏸ | Deferred to later phase |
| Theme switcher | 🚧 | UI present, persistence WIP |
| Import/Export | 🔜 | JSON import/export not yet implemented |
| Google auth & Drive sync | 🔜 | Planned for later phases |

For full implementation details, see the [roadmap](TODO.md).

---

## Screenshots

Screenshots will be added once the AMS sync and Spoolman browser flows are fully polished.

---

## Tech Stack

| Layer | Technology | Purpose |
|-------|-----------|---------|
| **Frontend** | Svelte 5 + SvelteKit | Reactive UI, routing |
| **Styling** | Tailwind CSS | Utility-first styling |
| **Backend** | Rust (Tauri v2) | Native shell, MQTT, DB |
| **Database** | SQLite (rusqlite) | Local persistent storage |
| **MQTT** | rumqttc | Bambu Lab communication |
| **HTTP** | Tauri HTTP APIs | SpoolmanDB integration |

---

## Installation

### Prerequisites

#### All platforms

- **Node.js** 20+ and npm
- **Rust** 1.70+ ([rustup.rs](https://rustup.rs/))

#### Arch Linux

```bash
sudo pacman -S webkit2gtk-4.1 base-devel curl wget file openssl appmenu-gtk-module gtk3 libappindicator-gtk3 librsvg
```

#### macOS

```bash
xcode-select --install
```

#### Windows

- Install [Microsoft Visual C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
- Install [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)

---

### Setup & Run

```bash
# Clone repository
git clone https://github.com/GaelicThunder/SpoolSync-Desktop.git
cd SpoolSync-Desktop

# Install JS dependencies
npm install

# Development with hot reload
npm run tauri:dev

# Build optimized binary
npm run tauri:build
```

On first run, configure your printer and AMS under **Settings → Printer & AMS**.

---

## Usage Overview

### 1. Configure Printer & AMS

- Open **Settings → Printer & AMS**.
- Set printer IP, serial, access code.
- Configure default AMS slot and AMS behavior.
- Save and optionally use the **Test connection** button (once implemented in Phase 9/14).

### 2. Browse & Manage Profiles

- Use the **Browse / Spoolman** page to query filament profiles from SpoolmanDB (when enabled).
- Mark profiles as **favorite** for quick access.
- Create **custom profiles** under the Custom section for filaments that are not present in Spoolman.

### 3. Sync to Bambu AMS

- Go to the **AMS Sync** page.
- Select:
  - Target printer
  - AMS unit and slot
  - Filament profile (favorite or custom)
- Trigger sync to send the configuration via MQTT to your Bambu Lab printer.

FilamentColors-based color browsing will be added later and is not required for AMS sync.

---

## Development

### Project Structure

```text
SpoolSync-Desktop/
├── src/                      # Svelte frontend
│   ├── routes/
│   │   ├── +layout.svelte     # Root layout & shell
│   │   ├── browse/            # Spoolman browser
│   │   ├── favorites/         # Favorites listing
│   │   ├── custom/            # Custom profiles CRUD
│   │   ├── ams/               # AMS sync page
│   │   └── settings/          # Printer & AMS settings
│   ├── app.css                # Tailwind entry
│   └── app.html               # HTML template
├── src-tauri/                 # Rust backend
│   ├── src/
│   │   └── main.rs            # Tauri commands & MQTT/DB glue
│   ├── Cargo.toml
│   └── tauri.conf.json        # App config
├── package.json
├── vite.config.ts
├── svelte.config.js
├── tailwind.config.js
└── TODO.md                    # Detailed roadmap
```

### Commands

```bash
# Dev with Tauri shell
npm run tauri:dev

# Build
npm run tauri:build

# Type checking / linting
npm run check

# Frontend-only dev (no Tauri shell)
npm run dev
```

---

## Roadmap

The full roadmap lives in [TODO.md](TODO.md). High-level phases:

- **Phase 4–7**: SpoolmanDB integration, MQTT, AMS UX
- **Phase 8–9**: Import/Export, Settings polish, theme persistence
- **Phase 10–11**: Google auth and Drive sync
- **Phase 12–13**: Desktop niceties, animations, notifications
- **Phase 14**: Testing, CI, release tooling

Current internal status is updated directly in `TODO.md`.

FilamentColors support and advanced color tools are explicitly tracked under **Phase 5: Color Browser** and are not yet implemented.

---

## Contributing

Pull requests are welcome. For larger changes, please open an issue first to discuss the design.

Recommended workflow:

1. Create a feature branch: `git checkout -b feature/my-feature`
2. Implement and add tests where applicable
3. Run `npm run check` and `npm run tauri:build`
4. Open a Pull Request with a clear description

---

## License & Contact

This project is licensed under the MIT License. See [LICENSE](LICENSE).

- 🐛 Issues: <https://github.com/GaelicThunder/SpoolSync-Desktop/issues>
- 💬 Discussions: <https://github.com/GaelicThunder/SpoolSync-Desktop/discussions>
- 📧 Email: [gaelicthunder@proton.me](mailto:gaelicthunder@proton.me)

---

**Status**: Active development (Phase 7 – MQTT & AMS polish)

