# SpoolSync Desktop

**Desktop companion for managing 3D printer filament profiles and syncing to Bambu Lab printers via MQTT.**

Built with Tauri v2 + Svelte 5 for maximum performance and minimal footprint (~8MB binary).

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Tauri](https://img.shields.io/badge/Tauri-v2.1-24C8DB?logo=tauri)
![Svelte](https://img.shields.io/badge/Svelte-v5-FF3E00?logo=svelte)

## Features

- 🔍 **Browse filaments** from SpoolmanDB community database
- ⭐ **Manage favorites** and custom profiles locally
- 🎨 **Visual color browser** with high-quality swatch images
- 🔌 **Direct MQTT sync** to Bambu Lab AMS slots
- 📤 **Import/Export** profiles as JSON
- 🔐 **Google login** for cloud sync (optional)
- 🌙 **Dark/Light theme** support
- ⚡ **Lightweight** - Native binary, not Electron
- 🖥️ **Cross-platform** - Windows, macOS, Linux

## Screenshots

*Coming soon - Phase 2 in progress*

## Tech Stack

| Layer | Technology | Purpose |
|-------|-----------|----------|
| **Frontend** | Svelte 5 + SvelteKit | Reactive UI framework |
| **Styling** | Tailwind CSS | Utility-first CSS |
| **Backend** | Rust (Tauri v2) | Native performance, security |
| **Database** | SQLite (embedded) | Local storage |
| **MQTT** | rumqttc | Bambu Lab printer communication |
| **HTTP** | Tauri fetch API | SpoolmanDB API calls |

## Installation

### Prerequisites

#### All platforms
- **Node.js** 20+ and npm
- **Rust** 1.70+ ([rustup.rs](https://rustup.rs/))

#### Linux (Arch)
```bash
sudo pacman -S webkit2gtk-4.1 base-devel curl wget file openssl appmenu-gtk-module gtk3 libappindicator-gtk3 librsvg
```

#### macOS
```bash
xcode-select --install
```

#### Windows
- Install [Microsoft Visual C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
- Install [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/) (usually pre-installed on Win10/11)

### Setup

```bash
# Clone repository
git clone https://github.com/GaelicThunder/SpoolSync-Desktop.git
cd SpoolSync-Desktop

# Install dependencies
npm install

# Run in development mode
npm run tauri:dev

# Build for production
npm run tauri:build
```

### First Run

1. App opens with a simple greeting screen (Phase 1 complete)
2. Next phases will add filament browsing, MQTT sync, and more
3. Check [TODO.md](TODO.md) for implementation progress

## Development

### Project Structure

```
SpoolSync-Desktop/
├── src/                    # Svelte frontend
│   ├── routes/
│   │   ├── +page.svelte     # Home screen
│   │   └── +layout.svelte   # Root layout
│   ├── app.css            # Tailwind entry
│   └── app.html           # HTML template
├── src-tauri/             # Rust backend
│   ├── src/
│   │   └── main.rs        # Tauri commands
│   ├── Cargo.toml
│   └── tauri.conf.json  # App config
├── package.json
├── vite.config.ts
├── svelte.config.js
├── tailwind.config.js
└── TODO.md              # Roadmap
```

### Commands

```bash
# Development with hot reload
npm run tauri:dev

# Build optimized binary
npm run tauri:build

# Type checking
npm run check

# Frontend only (for UI work)
npm run dev
```

### Adding Tauri Commands

```rust
// src-tauri/src/main.rs
#[tauri::command]
fn my_command(arg: String) -> Result<String, String> {
    Ok(format!("Got: {}", arg))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![my_command])
        .run(tauri::generate_context!())
        .expect("error");
}
```

```typescript
// src/routes/+page.svelte
import { invoke } from '@tauri-apps/api/core';

const result = await invoke('my_command', { arg: 'test' });
```

## Roadmap

See [TODO.md](TODO.md) for the complete implementation checklist.

**Current Status**: Phase 1 Complete ✅  
**Next Up**: Phase 2 - Core UI & Navigation

### Completed
- [x] Tauri v2 + Svelte 5 base setup
- [x] Tailwind CSS configuration
- [x] Basic greeting command (proof of concept)
- [x] Git repository structure

### Next Steps
- [ ] Main layout with sidebar navigation
- [ ] Database schema and SQLite integration
- [ ] SpoolmanDB API client
- [ ] MQTT client for Bambu Lab sync

## Contributing

Pull requests welcome! This is an active development project.

### Development Workflow

1. Fork the repository
2. Create feature branch: `git checkout -b feature/my-feature`
3. Commit changes: `git commit -m 'Add feature'`
4. Push to branch: `git push origin feature/my-feature`
5. Open Pull Request

## Why Not Electron?

| Feature | Tauri | Electron |
|---------|-------|----------|
| Binary size | ~8MB | ~150MB |
| Memory usage | ~60MB | ~300MB |
| Startup time | ~0.3s | ~1.5s |
| Technology | Rust + native webview | Chromium + Node.js |
| Security | Process isolation, Rust safety | Node.js vulnerabilities |

## Relation to Other Projects

- **Spoolman**: Self-hosted inventory tracker with weight monitoring. SpoolSync is a lightweight client that doesn't require a server.
- **OpenSpool**: NFC hardware reader for tap-and-go. SpoolSync is software-only with direct MQTT control.
- **SpoolSync Android**: Mobile version. Desktop version shares the same API integrations but optimized for keyboard/mouse workflow.

## License

MIT License - see [LICENSE](LICENSE) for details.

## Support

- 🐛 **Issues**: [GitHub Issues](https://github.com/GaelicThunder/SpoolSync-Desktop/issues)
- 💬 **Discussions**: [GitHub Discussions](https://github.com/GaelicThunder/SpoolSync-Desktop/discussions)
- 📧 **Email**: [gaelicthunder@proton.me](mailto:gaelicthunder@proton.me)

---

**Status**: Active Development 🚧  
**Version**: 0.1.0-alpha  
**Author**: Gaël (GaelicThunder)
