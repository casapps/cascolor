# cascolor - Project Specification

## Overview
A beautiful, cross-platform color picker application written in Rust. Single static binary with GUI (X11/Wayland) and TUI fallback support.

## Repository
`github.com/casapps/cascolor`

## Core Features

### Color Picker
- **Formats Supported**: HEX, RGB, RGBA, HSL, HSLA, HSV, HSB, CMYK, Lab, LCh, Luv, XYZ, Oklab, Oklch, CSS Named Colors
- **Input Methods**:
  - Interactive sliders (per channel)
  - Drag support for quick adjustment
  - Direct text input for each format
  - Eyedropper tool (pick from screen)
- **Copy Buttons**: One-click copy for each format
- **Full Mouse Support**: Click, drag, scroll

### UI Modes
- **GUI Mode** (iced framework):
  - X11/Wayland support on Linux
  - Native on Windows/macOS
  - Auto-detect: Use GUI if display available
- **TUI Mode** (ratatui framework):
  - Fallback for remote sessions or no display
  - Similar UX to GUI mode
  - Full mouse support in terminal

### Display Detection Logic
```
if SSH_CONNECTION exists OR (no DISPLAY and no WAYLAND_DISPLAY):
    use TUI
else:
    use GUI
```

### Window Behavior (GUI)
- **Default**: Maximized (not fullscreen, respects panels/taskbars)
- **Standard Controls**: Minimize, Maximize, Close
- **OS-Native Menus**: File, Edit, View, Help (platform-dependent layout)
- **Remembers**: Window size and position on next launch

### Theming
- **Default**: Dark theme
- **Options**: Dark, Light, System, Auto (based on time of day)
- **Consistency**: Same theme across GUI and TUI

### Color History
- **Default Size**: 20 colors
- **Configurable**: User can adjust history size
- **Persistent**: Saved to config file
- **Display**: Show recent colors, click to reload

### System Tray
- **Default**: Enabled
- **Optional**: User can disable via config
- **Features**: Quick access, minimize to tray

## Configuration

### Location
- Linux/BSD: `~/.config/casapps/cascolor/config.toml`
- macOS: `~/Library/Application Support/casapps/cascolor/config.toml`
- Windows: `%APPDATA%\casapps\cascolor\config.toml`

### Format (TOML - Simple and Human-Readable)
```toml
[general]
theme = "dark"              # Options: dark, light, system, auto
history_size = 20           # Number of colors to remember

[updates]
channel = "stable"          # Options: stable, beta, daily
check_on_startup = true     # Check for updates when app starts
check_in_background = true  # Periodic background checks (every 24h)
prompt_before_update = true # false = auto-update without asking

[ui]
show_system_tray = true     # System tray icon (can disable)
remember_window_position = true
default_color_format = "hex" # Default format shown

# Color history (managed by app)
color_history = [
    "#FF5733",
    "#33FF57",
]
```

## Auto-Update System

### Update Channels
1. **stable**: Production releases (e.g., `1.2.3` or `v1.2.3`)
2. **beta**: Beta releases (e.g., `1.2.3-beta` or `v1.2.3-beta`)
3. **daily**: Nightly builds (e.g., `20251218122905`)

### Update Checks
- **On Startup**: Check GitHub API for latest version
- **Background**: Check every 24 hours
- **Manual**: `--update [channel]` flag

### Update Flow
1. Check GitHub Releases API for latest in selected channel
2. Compare with current version (semver for stable/beta, timestamp for daily)
3. If update available: Show notification
4. If `prompt_before_update = true`: Show dialog with changelog
5. If `prompt_before_update = false`: Auto-download and apply
6. Download appropriate binary for OS/arch
7. Replace current binary (using `self_update` crate)
8. Notify user and offer restart

### Version Handling
- Tags accept both `v1.2.3` and `1.2.3` (strip leading `v` in comparisons)
- `release.txt` format: `1.2.3` or `1.2.3-beta` (no `v` prefix)
- Daily version: `YYYYMMDDHHMMSS` (e.g., `20251218122905`)

## Command-Line Interface

### Flags
```bash
cascolor               # Launch app (GUI or TUI auto-detect)
cascolor --help        # Show usage (kept in sync with README.md)
cascolor --version     # Show version info
cascolor --update [channel]  # Check/apply updates (optional channel, default: stable)
```

### --version Output Format
```
cascolor 1.2.3
Commit: abc123def456
Built: Wed Dec 18, 2024 at 12:29:05 EST
```
- Timezone priority: ENV `TZ` → System TZ → UTC (fallback)

### --update Behavior
- Respects `prompt_before_update` config
- HTTP 404 from GitHub API → Exit 0 (no update available)
- Network/API errors → Exit with appropriate error code
- Successful update → Exit 0

## Build System

### Binary Distribution
- **Single static binary** per platform
- Assumes system has display libraries (libwayland, libX11, etc.)
- All other dependencies statically linked

### Build Method
- **Docker-based**: Uses `rust:latest` official image
- **No local Rust required**: All builds in containers
- **Cross-compilation**: Target all platforms from Linux host

### Supported Platforms
| OS      | Architecture | Target Triple                | Binary Name                    |
|---------|--------------|------------------------------|--------------------------------|
| Linux   | x86_64       | x86_64-unknown-linux-musl    | cascolor-linux-x86_64          |
| Linux   | aarch64      | aarch64-unknown-linux-musl   | cascolor-linux-aarch64         |
| Windows | x86_64       | x86_64-pc-windows-gnu        | cascolor-windows-x86_64.exe    |
| Windows | aarch64      | aarch64-pc-windows-gnullvm   | cascolor-windows-aarch64.exe   |
| macOS   | x86_64       | x86_64-apple-darwin          | cascolor-macos-x86_64          |
| macOS   | aarch64      | aarch64-apple-darwin         | cascolor-macos-aarch64         |
| FreeBSD | x86_64       | x86_64-unknown-freebsd       | cascolor-freebsd-x86_64        |
| FreeBSD | aarch64      | aarch64-unknown-freebsd      | cascolor-freebsd-aarch64       |

### Binary Processing
- **musl binaries**: Strip before release (rename target triple)
  - `x86_64-unknown-linux-musl` → `cascolor-linux-x86_64`
- **Naming scheme**: `{projectname}-{os}-{arch}[.exe]`

### Build Outputs
- `./binaries/`: All platform binaries + host binary named `cascolor`
- `./releases/`: Production releases with source archive (no VCS files)

## Makefile Targets

### build
- Build for all platforms using Docker
- Output to `./binaries/{projectname}-{os}-{arch}`
- Also create host binary: `./binaries/cascolor`
- Strip musl binaries before output

### release
- Create GitHub release using `gh` CLI
- If tag exists, delete first
- Use `VERSION` env var if set, else auto-increment `release.txt`
- Upload all binaries from `./releases/`
- Include source archive (exclude VCS files)

### test
- Run all Rust tests (`cargo test`)
- Run integration tests from `./tests/`

### Version Management
- Read from `release.txt` (create if doesn't exist with app version)
- Semver format: `{major}.{minor}.{patch}` or `{major}.{minor}.{patch}-beta`
- Auto-increment patch version on release
- Respect `VERSION` env var if set

## CI/CD Workflows (GitHub Actions + Gitea Actions)

Both GitHub (.github/workflows/) and Gitea (.gitea/workflows/) use identical workflow logic with platform-specific adaptations.

### 1. daily.yml
**Triggers**:
- Push to `main` branch
- Cron: `0 3 * * *` (3:00 AM UTC daily)

**Version**: `date +%Y%m%d%H%M%S` (e.g., `20251218124551`)

**Tag**: `daily` (force push, overwrites previous)

**Steps**:
1. Checkout code
2. Set version to current timestamp
3. Build all platforms using Docker
4. Strip musl binaries
5. Create/update `daily` release
6. Upload all binaries

### 2. beta.yml
**Triggers**:
- Manual dispatch
- Tag push matching `*beta*` pattern

**Version**: Read from `release.txt` (must contain `-beta`)

**Tag**: `v?{version}` (accepts with/without `v` prefix)

**Steps**:
1. Checkout code
2. Read version from `release.txt`
3. Validate version contains `-beta`
4. Build all platforms
5. Create release with tag
6. Upload binaries

### 3. release.yml
**Triggers**:
- Manual dispatch
- Tag push (not containing `beta`)

**Version**: Read from `release.txt`

**Tag**: `v?{version}` (accepts with/without `v` prefix)

**Steps**:
1. Checkout code
2. Read version from `release.txt`
3. Build all platforms
4. Create release with tag
5. Upload binaries
6. Auto-increment patch version in `release.txt`
7. Commit updated `release.txt` back to repo

### Platform Differences
- **GitHub**: Uses `actions/checkout@v4`, GitHub-specific release APIs
- **Gitea**: Uses Gitea-compatible actions, Gitea release APIs

## Dependencies

### Core
- `serde` + `toml` - Configuration management
- `dirs` - Cross-platform config paths
- `palette` - Comprehensive color conversions
- `arboard` - Cross-platform clipboard
- `self_update` - Auto-update from GitHub releases
- `semver` - Version comparison

### GUI (iced)
- `iced` - Main GUI framework (X11/Wayland/Windows/macOS)
- `screenshots` - Screen capture for eyedropper

### TUI (ratatui)
- `ratatui` - Terminal UI framework
- `crossterm` - Cross-platform terminal manipulation

### Build Info
- `built` - Embed build metadata (commit, date, etc.)

## Project Structure

```
cascolor/
├── src/
│   ├── main.rs           # Entry point, display detection, CLI args
│   ├── lib.rs            # Shared library code
│   ├── config/
│   │   ├── mod.rs        # Config management
│   │   └── defaults.rs   # Default configuration
│   ├── color/
│   │   ├── mod.rs        # Color conversion logic
│   │   └── formats.rs    # All supported formats
│   ├── gui/
│   │   ├── mod.rs        # GUI entry point
│   │   ├── app.rs        # Main application state
│   │   ├── theme.rs      # Theme management
│   │   ├── widgets/      # Custom widgets
│   │   └── eyedropper.rs # Screen color picker
│   ├── tui/
│   │   ├── mod.rs        # TUI entry point
│   │   ├── app.rs        # TUI application state
│   │   └── ui.rs         # UI rendering
│   ├── clipboard.rs      # Clipboard operations
│   ├── update.rs         # Auto-update logic
│   └── version.rs        # Version info (built.rs generated)
├── tests/                # Integration tests
├── scripts/              # Installation scripts
│   ├── install.sh        # Universal installer
│   ├── linux.sh          # Linux-specific
│   ├── macos.sh          # macOS-specific
│   ├── windows.ps1       # Windows PowerShell
│   └── windows.cmd       # Windows batch
├── binaries/             # Build outputs
├── releases/             # Production releases
├── .github/
│   └── workflows/
│       ├── daily.yml
│       ├── beta.yml
│       └── release.yml
├── .gitea/
│   └── workflows/
│       ├── daily.yml
│       ├── beta.yml
│       └── release.yml
├── Cargo.toml            # Rust dependencies
├── build.rs              # Build script (embed version info)
├── release.txt           # Current version (e.g., "0.1.0")
├── Makefile              # Build automation
├── README.md             # Documentation
├── LICENSE.md            # MIT + embedded licenses
├── AI.md                 # This file
├── TODO.AI.md            # Task tracking
├── .gitignore
└── .dockerignore
```

## License
MIT License with embedded dependency licenses appended to LICENSE.md

## Notes
- All variables in documentation use `{variable}` notation
- Non-braced paths are literals (e.g., `/etc/letsencrypt/live/domain`)
- Responses should be short, concise, descriptive, and helpful
- Never assume - ask questions when unclear
