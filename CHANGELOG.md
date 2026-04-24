# Changelog

All notable changes to the **GamePiLot** project will be documented in this file.

## [Unreleased] - 2026-04-24

### Added
- **Real Wine/Winetricks Execution**: Replaced simulated thread sleeps with actual `std::process::Command` calls to `wine` and `winetricks`.
- **Live Output Logging**: The UI now includes a `ScrolledWindow` with a `TextView` to display live installation logs and errors from `winetricks` and `wine`.
- **Prefix Discovery**: Added automatic detection of Wine prefixes for Steam (`compatdata`), Heroic Games Launcher, Lutris, and default `~/.wine`.
- **Unified Game Manifests**: Migrated from individual game TOML files to a single, unified `games.toml` format mapping games by slug.
- **Internationalization (i18n)**: Built-in multi-language support (English, Portuguese, Spanish, Italian) reading directly from the system's `LANG` variable without heavy dependencies.
- **Steam App ID Matching**: Added `steam_app_id` scanning to prioritize exact match detections over simple string name inclusions.

### Fixed
- **Heroic Prefix Parsing**: Fixed a bug where GamePiLot failed to read Wine prefixes from Heroic's `GamesConfig` JSON files due to nested data structures.
- **Empty Prefix Protection**: The application now strictly checks and blocks dependency installations if no valid Wine prefix is detected or manually entered, preventing silent `winetricks` failures.
- **UI State Sync**: Fixed data synchronization between the selected game's detected prefix and the manual path entry widget.
- **Manifest Fallbacks**: Ensured tools like Simple Mod Framework (Hitman 3), SKSE, MO2, Vortex (Skyrim), Cheat Engine, and ReShade have the correct `download_url` and `executable_path` values.

### Removed
- Removed the heavy `tokio` (features = ["full"]) dependency from `Cargo.toml` as the project operates synchronously with standard threads, improving build times.
