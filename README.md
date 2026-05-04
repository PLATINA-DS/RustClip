# RustClip

A lightweight clipboard manager for Linux written in Rust. Tracks text, links, files, and images with a clean rofi-based interface.

## Features

- **Text Tracking**: Automatically captures all text copied to clipboard
- **Link Detection**: Identifies and categorizes HTTP/HTTPS links
- **File Support**: Tracks file:// protocol paths
- **Image Storage**: Saves clipboard images to disk with timestamps
- **History Management**: Maintains up to 50 items in history
- **Rofi Integration**: Clean, icon-enhanced menu for selecting clipboard items
- **Autostart**: Can be configured to run automatically on login

## Requirements

- Rust (for building)
- rofi (for the clipboard menu)
- Linux with X11 or Wayland

## Dependencies

- `arboard` - Cross-platform clipboard access
- `serde` / `serde_json` - Serialization for history storage
- `image` - Image processing and saving

## Installation

### Manual Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd RustClip
```

2. Build the project:
```bash
cargo build --release
```

3. Run the daemon:
```bash
./target/release/RustClip daemon
```

4. Show clipboard history (in another terminal):
```bash
./target/release/RustClip show
```

### Automated Installation

Run the provided activation script:

```bash
chmod +x scripts/activate.sh
./scripts/activate.sh
```

This script will:
- Install rofi if not present
- Copy the binary to `~/.local/bin/rustclip`
- Create an autostart entry for automatic startup
- Start the daemon in the background

## Usage

### Running the Daemon

The daemon runs in the background and monitors clipboard changes:

```bash
rustclip daemon
```

### Showing Clipboard History

To display the clipboard history menu:

```bash
rustclip show
```

### Keyboard Shortcut

For quick access, bind the `rustclip show` command to a keyboard shortcut in your desktop environment.

## Data Storage

- **History file**: `/tmp/rustclip_data/history.json`
- **Images directory**: `/tmp/rustclip_data/images/`

Note: Data is stored in `/tmp` and will be cleared on system restart. This is intentional for privacy and simplicity.

## Project Structure

```
RustClip/
├── src/
│   └── main.rs          # Main application code
├── scripts/
│   └── activate.sh      # Installation and setup script
├── Cargo.toml           # Rust dependencies
└── README.md            # This file
```

## Building

```bash
cargo build --release
```

The compiled binary will be available at `target/release/RustClip`.

## License

This project is open source and available under the MIT License.
