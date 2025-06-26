# gittop

A real-time Git repository monitoring tool inspired by `top` and `htop`, written in Rust.

## Features

- Real-time display of Git repository status
- Shows staged and unstaged files
- Displays recent commit history
- Shows branch information and remote status
- Terminal UI with keyboard navigation

## Installation

```bash
git clone https://github.com/engineers-hub-ltd/gittop.git
cd gittop
cargo build --release
```

## Usage

```bash
# Monitor current directory
gittop

# Monitor specific repository
gittop /path/to/repository
```

## Keyboard Shortcuts

- `q` - Quit
- `r` - Refresh
- `Tab` - Switch between tabs

## Development Status

This is Phase 1 (MVP) implementation with basic functionality:

- [x] Basic Git information retrieval
- [x] Simple TUI display
- [x] Exit functionality
- [ ] Real-time file system monitoring (Phase 2)
- [ ] Interactive features (Phase 3)
- [ ] Advanced features (Phase 4)

## Requirements

- Rust 1.70 or higher
- Git installed on your system

## License

MIT

## Testing

Added test line for demonstration.