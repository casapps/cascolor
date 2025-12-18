# cascolor

A beautiful, cross-platform color picker with GUI and TUI support. Single static binary with comprehensive color format support.

## About

**cascolor** is a modern color picker application written in Rust that automatically adapts to your environment. Whether you're working in a graphical desktop environment or over SSH, cascolor provides a consistent and intuitive interface for selecting, converting, and managing colors.

### Features

- **Multiple Color Formats**: HEX, RGB, RGBA, HSL, HSLA, HSV, HSB, CMYK, Lab, LCh, Luv, XYZ, Oklab, Oklch, and CSS Named Colors
- **Dual Interface**: 
  - **GUI Mode**: Beautiful native interface with X11/Wayland support (Linux), native Windows and macOS support
  - **TUI Mode**: Terminal UI for remote sessions or systems without display
- **Interactive Controls**: Sliders with drag support, direct text input, and one-click copy buttons
- **Eyedropper Tool**: Pick colors directly from your screen
- **Color History**: Automatically saves your last 20 colors (configurable)
- **Auto-Update**: Built-in update system with stable, beta, and daily channels
- **Themeable**: Dark (default), Light, System, and Auto (time-based) themes
- **Cross-Platform**: Linux, Windows, macOS, and FreeBSD support
- **System Tray**: Optional system tray integration (can be disabled)

## Installation

### Download Pre-built Binaries

Download the latest release for your platform from the [releases page](https://github.com/casapps/cascolor/releases):

- **Linux**: `cascolor-linux-x86_64` or `cascolor-linux-aarch64`
- **Windows**: `cascolor-windows-x86_64.exe`
- **macOS**: `cascolor-macos-x86_64` or `cascolor-macos-aarch64`
- **FreeBSD**: `cascolor-freebsd-x86_64` or `cascolor-freebsd-aarch64`

### Quick Install (Linux/macOS)

```bash
curl -sSL https://raw.githubusercontent.com/casapps/cascolor/main/scripts/install.sh | bash
```

### Manual Installation

1. Download the appropriate binary for your system
2. Make it executable: `chmod +x cascolor-*`
3. Move to your PATH: `sudo mv cascolor-* /usr/local/bin/cascolor`

## Usage

### Basic Usage

```bash
# Launch the application (auto-detects GUI or TUI mode)
cascolor

# Show version information
cascolor --version

# Check for updates
cascolor --update

# Update from a specific channel
cascolor --update beta
cascolor --update daily
```

### Configuration

Configuration file location:
- **Linux/BSD**: `~/.config/casapps/cascolor/config.toml`
- **macOS**: `~/Library/Application Support/casapps/cascolor/config.toml`
- **Windows**: `%APPDATA%\casapps\cascolor\config.toml`

Example configuration:

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
```

### Update Channels

- **stable**: Production releases (recommended)
- **beta**: Pre-release versions with new features
- **daily**: Nightly builds (bleeding edge)

## Development

### Prerequisites

- Docker (for building)
- Git

### Building from Source

```bash
# Clone the repository
git clone https://github.com/casapps/cascolor.git
cd cascolor

# Build for all platforms
make build

# Build for current platform only (faster)
make build-local

# Run tests
make test
```

### Project Structure

```
cascolor/
├── src/           # Source code
├── tests/         # Integration tests
├── scripts/       # Installation scripts
├── .github/       # GitHub Actions workflows
├── .gitea/        # Gitea Actions workflows
├── Cargo.toml     # Rust dependencies
├── Makefile       # Build automation
└── release.txt    # Current version
```

### Release Process

```bash
# Create a release (automatically increments version)
make release

# Or specify a version
VERSION=1.2.3 make release
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Workflow

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Make your changes
4. Run tests: `make test`
5. Build: `make build`
6. Commit: `git commit -am 'Add new feature'`
7. Push: `git push origin feature/my-feature`
8. Submit a Pull Request

## License

MIT License - see [LICENSE.md](LICENSE.md) for details.

## Repository

- GitHub: https://github.com/casapps/cascolor
- Issues: https://github.com/casapps/cascolor/issues

## Support

For bugs, feature requests, or questions, please open an issue on GitHub.
