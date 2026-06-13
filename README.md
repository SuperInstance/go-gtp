# go-gtp: Go Text Protocol Engine for the Game of Go

A Rust implementation of the **Go Text Protocol (GTP)** — the standard text-based interface between Go engines and graphical frontends. Includes a GTP command parser, a 19×19 board representation with stone placement, and the foundations for building a Go-playing agent.

## Why It Matters

The Go Text Protocol (GTP, version 2) is the universal interface for Go engines, defined by the GNU Go project. Every major Go engine (GnuGo, Leela Zero, KataGo, Pachi) speaks GTP. A correct GTP implementation is the prerequisite for:

- Connecting a custom engine to any Go GUI (GoGui, Sabaki, Lizzie)
- Participating in Computer Go tournaments (CGOS, KGS)
- Benchmarking against existing engines
- Building analysis pipelines (tsumego solvers, joseki databases)

The game of Go itself is famous in AI: with 10¹⁷⁰ legal positions, it has a state space larger than chess by 80 orders of magnitude. AlphaGo's 2016 victory over Lee Sedol was a landmark in deep reinforcement learning.

## How It Works

### GTP Command Parsing

GTP commands follow the format:

```
[id] command_name [args...]
```

Where `id` is an optional numeric identifier for request-response correlation. The parser:

1. Strips comments (everything after `#`)
2. Tokenizes by whitespace
3. Checks if the first token is numeric (→ id)
4. Remaining tokens form the command name + args

```
"1 play black D4"   →  GtpCommand { id: Some(1), name: "play", args: ["black", "D4"] }
"boardsize 19"      →  GtpCommand { id: None, name: "boardsize", args: ["19"] }
```

**Complexity**: O(n) where n = command string length.

### Board Representation

A fixed 19×19 grid using a 2D array of `Option<Stone>`:

```
grid: [[Option<Stone>; 19]; 19]

Stone = Black | White
```

Stone placement is O(1) — direct array index. The `place` method rejects moves on occupied points.

### Vertex Encoding

Board intersections use GTP vertex notation: column letters (skipping `I`) + row numbers:

```
A1 (bottom-left)  ...  T19 (top-right)
```

Column letters: A, B, C, D, ..., H, J, K, ..., T (19 columns, `I` is skipped to avoid confusion with `1`).

### Complexity

| Operation | Time | Space |
|-----------|------|-------|
| Parse command | O(n) | O(n) tokens |
| `place(Vertex, Stone)` | O(1) | O(1) |
| `get(Vertex)` | O(1) | O(1) |
| `clear()` | O(N²) | O(1) where N = 19 |

Board state: 19 × 19 × 1 byte = 361 bytes (compact).

## Quick Start

```rust
use go_gtp::{GtpCommand, GoBoard, Stone, Vertex};
use std::str::FromStr;

// Parse GTP commands
let cmd = GtpCommand::from_str("1 play black D4").unwrap();
assert_eq!(cmd.id, Some(1));
assert_eq!(cmd.name, "play");
assert_eq!(cmd.args, vec!["black", "D4"]);

// Manage the board
let mut board = GoBoard::new();
board.place(Vertex(3, 3), Stone::Black).unwrap(); // D4
assert_eq!(board.get(Vertex(3, 3)), Some(Stone::Black));
assert_eq!(board.get(Vertex(0, 0)), None);
```

## API

### `GtpCommand`

| Field | Type | Description |
|-------|------|-------------|
| `id` | `Option<u32>` | Optional numeric request ID |
| `name` | `String` | Command name (lowercased) |
| `args` | `Vec<String>` | Command arguments |

Implements `FromStr` for parsing from `&str`.

### `GoBoard`

| Method | Signature | Description |
|--------|-----------|-------------|
| `new()` | `() -> Self` | Empty 19×19 board |
| `place(Vertex, Stone)` | `() -> Result<(), &str>` | Place stone (errors if occupied) |
| `get(Vertex)` | `(Vertex) -> Option<Stone>` | Read stone at position |
| `clear()` | `(&mut self)` | Remove all stones |

### `Stone`

`Black | White`, with `opposite()` for alternating turns.

### `Vertex(pub u8, pub u8)`

Zero-indexed (column, row) pair. Column 0 = A, column 18 = T.

## Architecture Notes

This is a **γ (gamma)** module — the board state and protocol parsing are deterministic and side-effect-free. In the γ + η = C framework, this provides the game-state substrate; an **η** layer would add move generation, UCT/MCTS search, pattern matching, and engine logic. The GTP protocol is the interface contract between γ (game state) and η (strategy).

## References

- Bump, G. (2002). *Go Text Protocol Specification, Version 2, draft 2*. GNU Go documentation.
- Silver, D. et al. (2016). *Mastering the Game of Go with Deep Neural Networks and Tree Search*. Nature 529, 484–489.
- Müller, M. (2002). *Computer Go*. Artificial Intelligence 134(1–2), 145–179.

## License

MIT
