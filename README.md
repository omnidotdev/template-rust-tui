# 🖥️ Rust TUI Template

Terminal user interface application built with [Ratatui](https://ratatui.rs).

## Features

- 🖥️ **Interactive TUI**: Built with [ratatui](https://ratatui.rs) and crossterm
- ⚡ **Event-driven**: Async architecture with tokio
- ⌨️ **Full keyboard support**: Emacs-style line editing
- 🖱️ **Mouse support**: Scroll with mouse wheel
- 📜 **Auto-scroll**: With manual override
- 🔒 **Safety**:
  - Unsafe code forbidden via lint
  - Strict Clippy lints (pedantic, nursery, cargo)
- ⚡ **Optimized Builds**:
  - LTO (Link-Time Optimization) enabled
  - Single codegen unit for maximum optimization
  - Binary stripping for smaller size

## Prerequisites

- [Rust](https://rustup.rs) 1.85+

## Development

### Running

```sh
cargo run
```

### Testing

```sh
cargo test
```

### Linting

```sh
cargo clippy
```

### Formatting

```sh
cargo fmt
```

## Building

```sh
cargo build --release
```

## Key Bindings

| Key | Action |
|-----|--------|
| `Ctrl+C` / `Ctrl+D` / `Esc` | Quit |
| `Left` / `Right` | Move cursor |
| `Ctrl+A` | Move to start |
| `Ctrl+E` | Move to end |
| `Backspace` | Delete char |
| `Delete` | Delete char forward |
| `Ctrl+U` | Delete to start |
| `Ctrl+K` | Delete to end |
| `Ctrl+W` | Delete word |
| `Ctrl+L` | Clear input |
| `Enter` | Submit |
| `Up` / `Down` | Scroll |
| `PageUp` / `PageDown` | Scroll fast |
| `Home` / `End` | Scroll to top/bottom |
| Mouse wheel | Scroll |

## License

The code in this repository is licensed under Apache 2.0, &copy; [Omni LLC](https://omni.dev). See [LICENSE.md](LICENSE.md) for more information.
