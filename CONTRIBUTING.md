# Contributing

Pull requests are welcome.

## Development

Build the project:

```bash
cargo build
```

Run the test suite:

```bash
cargo test
```

Run the standard checks before opening a PR:

```bash
cargo fmt --check
cargo clippy --all-targets
```

Run the TUI editor locally:

```bash
cargo run
```

Render a status line from JSON:

```bash
cat input.json | cargo run
```

## Guidelines

- Keep changes scoped to the problem being solved.
- Add or update tests when changing parsing, rendering, or config behavior.
- Update `README.md` when changing install steps, user-facing behavior, or configuration.
- For theme or TUI changes, screenshots are helpful but not required.

## Presets and Themes

- Keep preset names clear and user-facing.
- Prefer readable defaults across plain, Nerd Font, and powerline modes.
- Avoid introducing icons or colors that only work in one terminal setup unless clearly documented.
