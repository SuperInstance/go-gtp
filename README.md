# Go Text Protocol (GTP) Engine

**A Rust library implementing the Go Text Protocol** — the standard communication protocol for Go (Baduk/Weiqi) engine interfaces — with command parsing, a 19×19 board model, and stone placement logic.

## Why It Matters

The Go Text Protocol (GTP) is how Go engines (GNU Go, Leela Zero, KataGo) communicate with graphical frontends and online servers. Every competitive Go program implements GTP to receive commands like `play black D4`, `genmove white`, and `boardsize 19`. This library provides the parsing layer and board representation, handling the text protocol format (optional numeric IDs, command names, arguments) and maintaining board state (stone placement, occupancy checks). Whether you're building a Go engine, a GTP controller, or an analysis tool, this crate handles the protocol plumbing.

## How It Works

**Protocol parsing** implements `FromStr` for `GtpCommand`. A GTP line like `1 play black D4` is parsed by: (1) stripping comments after `#`, (2) splitting on whitespace, (3) checking if the first token is a numeric ID (e.g., `1`), and (4) extracting the command name (`play`) and arguments (`["black", "D4"]`). Both `play black D4` (no ID) and `10 play black D4` (with ID) are valid GTP.

**Board representation** uses a fixed `19×19` array of `Option<Stone>` where `Stone` is `Black` or `White`. The `GoBoard` struct supports `place(Vertex, Stone)` with occupancy checking, `get(Vertex)` queries, and `clear()`. The `Stone` enum has an `opposite()` method for alternating turns. Vertices are `(x, y)` coordinates (column, row) using 0-indexed `u8` values — the GTP vertex `D4` maps to `Vertex(3, 3)`.

## Quick Start

```rust
use go_gtp::*;
use std::str::FromStr;

fn main() {
    // Parse GTP commands
    let cmd: GtpCommand = "1 play black D4".parse().unwrap();
    println!("ID: {:?}, Command: {}, Args: {:?}", cmd.id, cmd.name, cmd.args);

    let cmd2: GtpCommand = "genmove white".parse().unwrap();
    println!("Command: {}", cmd2.name);

    // Board operations
    let mut board = GoBoard::new();
    board.place(Vertex(3, 3), Stone::Black).unwrap(); // D4
    board.place(Vertex(15, 15), Stone::White).unwrap(); // Q16

    println!("D4: {:?}", board.get(Vertex(3, 3)));  // Some(Black)
    println!("Q16: {:?}", board.get(Vertex(15, 15))); // Some(White)
    println!("K10: {:?}", board.get(Vertex(9, 9)));   // None
}
```

## API

| Type / Function | Description |
|---|---|
| `GtpCommand` (impl `FromStr`) | Parsed GTP command with optional ID, name, and args |
| `Stone::Black` / `Stone::White` | Stone colors with `opposite()` |
| `Vertex(x, y)` | Board coordinate (0-indexed column, row) |
| `GoBoard::new()` | Create an empty 19×19 board |
| `GoBoard::place(v, stone)` | Place a stone (errors if occupied) |
| `GoBoard::get(v)` | Query the stone at a vertex |
| `GoBoard::clear()` | Remove all stones |

## Architecture Notes

Part of the SuperInstance game engine collection. The GTP parser interfaces with the board model for move execution and game state tracking. See the [Architecture Guide](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md).

## License

MIT
