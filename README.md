# Anduran

Anduran is a Tauri desktop app for inspecting and working with fheroes2 save
data.

The repository is a Rust workspace with two main parts:

```text
.
|-- Cargo.toml          # Workspace definition
|-- app/                # Tauri + SvelteKit desktop app
|   |-- package.json    # Frontend and Tauri commands
|   |-- src/            # Svelte UI
|   `-- src-tauri/      # Rust desktop shell
`-- kastore/            # Rust library for reading/writing fheroes2 saves
```

## Running The App

Install frontend dependencies from the app directory:

```sh
cd app
pnpm install
```

Start the desktop app:

```sh
pnpm tauri dev
```

This starts the Svelte frontend on `http://localhost:1420` and opens the Tauri
desktop window.

To run only the browser frontend:

```sh
pnpm dev
```

## Rust Workspace

The root `Cargo.toml` is a workspace manifest:

```toml
[workspace]
members = [
    "app/src-tauri",
    "kastore",
]
resolver = "3"
```

Cargo build output is written to the root `target/` directory. This is normal
for a workspace and should not be committed.

## Project Roles

- `app/`: the user-facing desktop application.
- `app/src-tauri/`: the Rust backend for the Tauri app.
- `kastore/`: the reusable Rust crate that parses and writes fheroes2 save
  files.
