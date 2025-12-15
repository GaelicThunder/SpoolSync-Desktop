# SpoolSync Desktop - Implementation Roadmap

## Phase 1: Base Setup ✅
- [x] Initialize Tauri v2 project
- [x] Configure Svelte 5 + SvelteKit
- [x] Setup Tailwind CSS
- [x] Create project structure
- [x] Git repository initialization

## Phase 2: Core UI & Navigation ✅
- [x] Main window layout with sidebar
- [x] Navigation routing (Home, Colors, Custom, Settings)
- [ ] Theme switcher (dark/light)
- [x] Responsive layout system
- [x] Loading states and error boundaries

## Phase 3: Database Layer ✅
- [x] SQLite integration via rusqlite
- [x] Filament profile schema
- [x] Favorites table
- [x] Custom profiles table
- [x] Settings storage
- [x] CRUD operations for profiles

## Phase 4: SpoolmanDB Integration
- [ ] API client for SpoolmanDB
- [ ] Search/filter filaments
- [ ] Pagination handling
- [ ] Filament detail view
- [ ] Cache responses locally
- [ ] Brand/material filters

## Phase 5: Color Browser
- [ ] Filament Colors API integration
- [ ] Grid layout with color swatches
- [ ] Image loading optimization
- [ ] Filter by brand/material
- [ ] Add to favorites from browser

## Phase 6: Favorites & Custom Profiles ✅
- [x] Favorites list with search
- [x] Star/unstar functionality
- [x] Custom profile creation form
- [ ] Edit custom profiles
- [x] Delete custom profiles
- [x] Validation (temps, colors, etc)

## Phase 7: MQTT Integration 🚧 IN PROGRESS
- [ ] Rust MQTT client (rumqttc)
- [ ] Connect to Bambu Lab printer
- [ ] TLS/SSL configuration
- [ ] AMS slot selection UI
- [ ] Send filament config command
- [ ] Connection status indicator
- [ ] Error handling and retries

## Phase 8: Import/Export
- [ ] Export profile as JSON
- [ ] Import profile from JSON file
- [ ] Drag & drop file support
- [ ] Bulk export all favorites
- [ ] Bulk import validation

## Phase 9: Settings & Configuration ✅
- [x] Printer settings form (IP, serial, access code)
- [x] Save/load settings from DB
- [ ] Test connection button (working)
- [x] Default AMS slot selection
- [ ] Theme preference persistence
- [x] Auto-sync on startup option

## Phase 10: Google Authentication
- [ ] OAuth flow via Tauri
- [ ] Google Sign-In button
- [ ] Store auth tokens securely
- [ ] User profile display
- [ ] Sign out functionality

## Phase 11: Cloud Sync (Google Drive)
- [ ] Google Drive API integration
- [ ] Backup favorites to Drive
- [ ] Restore from Drive
- [ ] Auto-sync on changes
- [ ] Conflict resolution
- [ ] Sync status indicator

## Phase 12: Desktop-Specific Features
- [ ] Keyboard shortcuts (Ctrl+F, Ctrl+S, etc)
- [ ] System tray icon
- [ ] Minimize to tray
- [ ] Desktop notifications
- [ ] Auto-updater
- [ ] Window state persistence

## Phase 13: Polish & UX
- [ ] Animations and transitions
- [ ] Empty states with illustrations
- [ ] Tooltips and help text
- [x] Confirmation dialogs
- [ ] Toast notifications
- [ ] About/Help screen

## Phase 14: Testing & Deployment
- [ ] Unit tests (Rust backend)
- [ ] Integration tests
- [ ] Build scripts for all platforms
- [ ] Code signing (macOS/Windows)
- [ ] Release workflow (GitHub Actions)
- [ ] Documentation

## Future Enhancements
- [ ] Multiple printer support
- [ ] Batch sync operations
- [ ] Usage tracking per spool
- [ ] Print history integration
- [ ] Community profile repository
- [ ] Plugin system for other printers (Prusa, Klipper)
- [ ] CLI for automation
- [ ] i18n support

---

**Current Status**: Phase 7 (MQTT) in progress 🚧  
**Completed**: Phases 1, 2, 3, 6, 9
**Next**: Complete MQTT integration for Bambu Lab sync
