# IC10LSP

A language server for IC10, the MIPS-like assembly language used in the game
Stationeers. Talks LSP over stdio or TCP, so it works with any LSP-capable
editor.

![Demo](demo.gif)

## Features

- Completions for instructions, defines/aliases/labels, logic types, slot
  logic types, and batch modes (raw register/device literals and
  reagent-mode names aren't completed)
- Hover information for instructions, defines/aliases/labels, and logic
  types
- Signature help
- Goto definition
- Document symbols (an outline of defines, aliases, and labels)
- Diagnostics: syntax errors, unsupported instructions, type mismatches,
  wrong argument counts, duplicate definitions, line/column length limits,
  and a few style lints
- Quick fixes for some of those lints
- Inlay hints resolving hashed Stationpedia item/logic-type literals to
  their names
- Semantic highlighting

## Installation

Prebuilt binaries for Linux and Windows are attached to each
[GitHub release](https://github.com/Xandaros/ic10lsp/releases).

To build from source:

```sh
git clone https://github.com/Xandaros/ic10lsp.git
cd ic10lsp
cargo build --release
```

The binary ends up at `target/release/ic10lsp` (`ic10lsp.exe` on Windows).

## Usage

By default the server communicates over stdio, which is what most editors
expect when they spawn a language server themselves:

```sh
ic10lsp
```

Two TCP modes are also available, for setups that talk to a language server
over a socket instead:

```sh
ic10lsp --listen [host] [port]   # bind and wait for one client (default 127.0.0.1:9257)
ic10lsp <host> <port>            # connect out to a listening client
```

ic10lsp doesn't ship editor-specific plugins; point your editor's LSP client
at the binary (or the socket, in TCP mode) yourself.

## Configuration

The language server reads the following configuration options via
`workspace/didChangeConfiguration`:

| Key                         | Description                                      | Default |
| --------------------------- | ------------------------------------------------ | ------- |
| max_lines                   | Maximum number of lines                          | 128     |
| max_columns                 | Maximum number of columns                        | 90      |
| warnings.overline_comment   | Emit a warning on comments past the line limit   | true    |
| warnings.overcolumn_comment | Emit a warning on comments past the column limit | false   |

## Commands

The language server exposes the following commands via
`workspace/executeCommand`:

| Command | Description                                            |
| ------- | ------------------------------------------------------ |
| version | Show a message with the version of the language server |

## Related

- [tree-sitter-ic10](https://crates.io/crates/tree-sitter-ic10) -- the IC10
  grammar this server parses with.

## License

MIT. See [LICENSE](LICENSE).
