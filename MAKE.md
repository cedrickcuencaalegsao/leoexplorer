# Tauri + Dioxus — Makefile Guide

This document explains the project `Makefile` — what it does, how each target works,
and how to use it day-to-day. The Makefile wraps common `cargo` and `cargo tauri`
commands so you don't have to remember long chains of flags.

---

## Prerequisites

| Tool | Why it's needed |
|---|---|
| `make` | Runs the Makefile (built-in on Linux/macOS; install via [chocolatey](https://chocolatey.org/) or [GnuWin32](http://gnuwin32.sourceforge.net/) on Windows) |
| Rust + Cargo | The Tauri backend is written in Rust |
| `cargo-tauri` (`cargo install tauri-cli`) | The Tauri CLI used for `dev`, `build`, and `release` |
| `dioxus-iconify` (`cargo install dioxus-iconify`) | Only needed for the `add-icon` target |

---

## Makefile — Annotated

```makefile
# ─── Tauri + Dioxus Makefile ───────────────────────────────────────────────
# Works on Linux, macOS, and Windows (with make installed)
# Usage:
#   make init          → Install all dependencies and set up the project
#   make dev           → Start development server
#   make build         → Clean & build the app (debug)
#   make release       → Clean & build the app (release)
#   make clean         → Remove all build artifacts
#   make test          → Run Rust tests (backend)
#   make lint          → Check code with clippy
# ────────────────────────────────────────────────────────────────────────────
```

> The header comment is purely informational — it appears when you open the file
> and acts as quick-start documentation for anyone new to the project.

---

### `.PHONY` Declaration

```makefile
.PHONY: init dev build release clean test lint help
```

`make` normally checks for a *file* with the same name as the target. Declaring
targets as `.PHONY` tells `make` to **always run** them, even if a file named
`dev`, `build`, etc. happens to exist in the directory. Every target in this
Makefile is phony because none of them produce a file directly.

---

### Variables

```makefile
TAURI_CLI   = cargo tauri
BACKEND_DIR = src-tauri
```

| Variable | Value | Purpose |
|---|---|---|
| `TAURI_CLI` | `cargo tauri` | The full CLI command used to invoke Tauri. Change this to `npx tauri` if you use the JS wrapper instead. |
| `BACKEND_DIR` | `src-tauri` | Path to the Rust backend directory. Every `cargo` command `cd`s here first. |

Using variables means you only need to update one line if these paths ever change.

---

### OS Detection

```makefile
ifeq ($(OS),Windows_NT)
    RM = del /Q /S
else
    RM = rm -rf
endif
```

`make` exposes the `$(OS)` environment variable. On Windows it equals `Windows_NT`;
on Linux/macOS it is unset. This block picks the right delete command for each
platform:

- **Linux / macOS** → `rm -rf` (standard Unix recursive delete)
- **Windows** → `del /Q /S` (quiet, recursive delete in cmd.exe)

> **Note:** `$(RM)` is defined here but the targets below use `cargo clean`
> instead, which is cross-platform by itself. The `$(RM)` variable is available
> if you ever want to add a target that deletes non-Cargo files.

---

### Colors

```makefile
# ── Colors (ANSI escape codes — only used in init) ─────────────────────────
RESET  = \033[0m
BOLD   = \033[1m
GREEN  = \033[0;32m
CYAN   = \033[0;36m
YELLOW = \033[0;33m
```

ANSI escape codes are special byte sequences that terminals interpret as
formatting instructions rather than printable characters. `\033` is the escape
character (octal 33, hex 1B); what follows selects the color or style.

| Variable | Code | Appearance |
|---|---|---|
| `RESET` | `\033[0m` | Clears all active colors and styles back to the terminal default |
| `BOLD` | `\033[1m` | Makes text bold / bright |
| `GREEN` | `\033[0;32m` | Standard green — used for the success message |
| `CYAN` | `\033[0;36m` | Cyan — used for step descriptions and the `make dev` hint |
| `YELLOW` | `\033[0;33m` | Yellow — used for the `[  0%]` … `[100%]` progress badge |

These are intentionally scoped to `init` only. The `help` target and all other
targets keep plain `@echo` so they stay readable when piped to a file or run
in a CI log that strips escape codes.

---

### `help` — Default Target

```makefile
help:
	@echo "Available targets:"
	@echo "  init      Install all dependencies and set up the project"
	@echo "  add-icon  Add icons to the project from dioxus-iconify"
	@echo "  dev       Run Tauri development server"
	@echo "  build     Clean backend, then build (debug)"
	@echo "  release   Clean backend, then build (release)"
	@echo "  clean     Remove all Rust build artifacts"
	@echo "  test      Run cargo test in the backend"
	@echo "  lint      Run clippy lints on the backend"
	@echo "  help      Show this help"
```

**Usage:** `make` or `make help`

Because `help` is the *first target* defined, running bare `make` with no
arguments will print the usage menu. The `@` prefix suppresses echoing the
`echo` command itself — you see only the output, not the command.

---

### `init` — Install Dependencies & Set Up the Project

```makefile
init:
	@printf "$(YELLOW)[  0%%]$(RESET) $(CYAN)→ Installing Tauri CLI...$(RESET)\n"
	cargo install tauri-cli
	@printf "$(YELLOW)[ 33%%]$(RESET) $(CYAN)→ Installing dioxus-iconify...$(RESET)\n"
	cargo install dioxus-iconify
	@printf "$(YELLOW)[ 66%%]$(RESET) $(CYAN)→ Fetching Rust dependencies...$(RESET)\n"
	cd $(BACKEND_DIR) && cargo fetch
	@printf "$(GREEN)[100%%]$(RESET) $(BOLD)✓ Done!$(RESET) Run '$(CYAN)make dev$(RESET)' to start the development server.\n"
```

**Usage:** `make init`

Run this **once** after cloning the repository. It installs every tool the other
`make` targets depend on, then pre-downloads all Rust crate dependencies so the
first `make dev` or `make build` doesn't have to fetch them from the network.

**Terminal output preview:**

```
[  0%] → Installing Tauri CLI...         ← yellow badge, cyan text
[ 33%] → Installing dioxus-iconify...    ← yellow badge, cyan text
[ 66%] → Fetching Rust dependencies...   ← yellow badge, cyan text
[100%] ✓ Done! Run 'make dev' to start the development server.
  ↑ green + bold         ↑ cyan
```

**Why `printf` instead of `echo`?**

`echo -e` (which interprets `\033` escape codes) is not portable — it behaves
differently across shells and is not available in all environments. `printf` is
POSIX-standard and works identically on Linux, macOS, and Git Bash on Windows,
making it the safe choice for colored output in Makefiles.

**Why `%%` for the percent sign?**

`printf` treats `%` as the start of a format specifier (`%s`, `%d`, etc.).
Writing `%%` tells `printf` to output a literal `%` character instead.

**Why `$(RESET)` after every colored segment?**

Without `$(RESET)`, the color would bleed into the `cargo install` output that
follows each `printf` line. Resetting after each segment keeps only the intended
text colored.

What each step does:

1. `cargo install tauri-cli` — installs the `cargo tauri` CLI globally into
   `~/.cargo/bin/`. This is the tool behind `make dev`, `make build`, and
   `make release`. If it is already installed, Cargo will reinstall the latest
   version; add `--locked` to pin to the version in `Cargo.lock` instead.

2. `cargo install dioxus-iconify` — installs the `dioxus-iconify` binary
   globally, required by `make add-icon`. Safe to skip if you won't be adding
   icons.

3. `cd $(BACKEND_DIR) && cargo fetch` — downloads every crate listed in
   `src-tauri/Cargo.lock` into the local Cargo cache (`~/.cargo/registry`).
   The source code is never compiled here, so it is fast. Subsequent builds will
   use the cache and won't need a network connection.

> **First-time setup on Linux** — Tauri also needs a few system libraries
> (e.g. `webkit2gtk`, `libgtk-3-dev`). Install them with your package manager
> before running `make init`. See the
> [Tauri prerequisites guide](https://tauri.app/start/prerequisites/) for the
> exact packages per distro.

---

### `dev` — Development Server

```makefile
dev:
	$(TAURI_CLI) dev
```

**Usage:** `make dev`

Expands to `cargo tauri dev`. This starts the Tauri development server with:

- Hot-reload for the **frontend** (Dioxus/HTML)
- Automatic recompile of the **Rust backend** on file changes
- A native window that opens your app

Leave this running while you work. Stop it with `Ctrl+C`.

---

### `build` — Debug Build

```makefile
build:
	cd $(BACKEND_DIR) && cargo clean && cd .. && $(TAURI_CLI) build --debug
```

**Usage:** `make build`

This runs three steps in sequence:

1. `cd src-tauri` — move into the Rust backend directory
2. `cargo clean` — wipe all previous build artifacts so you get a completely
   fresh compile (avoids stale cache issues)
3. `cd ..` then `cargo tauri build --debug` — build the full Tauri app in
   **debug** mode (larger binary, faster compile, includes debug symbols)

> Use this when you need a real installable binary for testing but don't need
> the optimisations of a release build.

---

### `release` — Release Build

```makefile
release:
	cd $(BACKEND_DIR) && cargo clean && cd .. && $(TAURI_CLI) build --release
```

**Usage:** `make release`

Identical flow to `build` but passes `--release` instead of `--debug`. Cargo
compiles with full optimisations (`opt-level = 3`). The result is a smaller,
faster binary suitable for distribution. Compile time will be significantly
longer.

> This is what you run before packaging or shipping to users.

---

### `clean` — Remove Build Artifacts

```makefile
clean:
	cd $(BACKEND_DIR) && cargo clean
```

**Usage:** `make clean`

Deletes the `src-tauri/target/` directory — all compiled Rust objects,
libraries, and binaries. Useful when:

- Switching Rust toolchain versions
- Resolving mysterious compile errors
- Freeing disk space (`target/` can reach several GB)

After cleaning, the next `build` or `release` will start from scratch.

---

### `test` — Run Rust Tests

```makefile
test:
	cd $(BACKEND_DIR) && cargo test
```

**Usage:** `make test`

Runs all unit and integration tests found in `src-tauri/` via `cargo test`.
Output shows which tests passed, failed, or were ignored.

> Add `-- --nocapture` to see `println!` output from tests:
> ```
> make test ARGS="-- --nocapture"
> ```
> (Requires updating the recipe to `cargo test $(ARGS)`)

---

### `lint` — Clippy Static Analysis

```makefile
lint:
	cd $(BACKEND_DIR) && cargo clippy --all-targets --all-features
```

**Usage:** `make lint`

Runs [Clippy](https://github.com/rust-lang/rust-clippy), Rust's official linter,
across every target (bins, libs, tests, examples) and with every feature flag
enabled. Clippy catches common mistakes, style issues, and performance
anti-patterns that the compiler itself lets through.

Fix all warnings before merging to keep the codebase clean. To treat warnings as
errors in CI, you can add `-- -D warnings` to the clippy invocation.

---

### `add-icon` — Add Dioxus Icons

```makefile
ICON ?=

add-icon:
	@if [ -z "$(ICONS)" ]; then \
		echo "Usage: make add-icon ICONS=\"<icon-name> [icon-name ...]\""; \
		exit 1; \
	fi
	dioxus-iconify add $(ICONS)

.PHONY: add-icon
```

**Usage:**

```bash
# Add a single icon
make add-icon ICONS="material-symbols:home-outline"

# Add multiple icons at once
make add-icon ICONS="mdi:home mdi:account mdi:settings"
```

`ICONS` is an **overridable variable** — you pass it on the command line.
The `?=` operator means the variable defaults to empty if not supplied.

The shell `if` guard (`-z "$(ICONS)"`) checks that `ICONS` isn't blank and
prints a helpful message before exiting with an error code, rather than running
`dioxus-iconify add` with no arguments.

`dioxus-iconify add` downloads and registers the requested icons from
[Iconify](https://iconify.design/) so you can use them as Dioxus components.

> `add-icon` has its own `.PHONY` declaration at the bottom. This is equivalent
> to including it in the top-level `.PHONY` list — both styles are valid.

---

## Quick Reference

| Command | What it does |
|---|---|
| `make` | Print the help menu |
| `make init` | Install all tools and fetch Rust dependencies |
| `make dev` | Start hot-reloading dev server |
| `make build` | Fresh debug build |
| `make release` | Fresh optimised release build |
| `make clean` | Delete all Rust build artifacts |
| `make test` | Run all Rust tests |
| `make lint` | Run Clippy on all targets and features |
| `make add-icon ICONS="mdi:home"` | Add one or more Iconify icons |

---

## The Full Makefile

```makefile
# ─── Tauri + Dioxus Makefile ───────────────────────────────────────────────
# Works on Linux, macOS, and Windows (with make installed)
# Usage:
#   make init          → Install all dependencies and set up the project
#   make dev           → Start development server
#   make build         → Clean & build the app (debug)
#   make release       → Clean & build the app (release)
#   make clean         → Remove all build artifacts
#   make test          → Run Rust tests (backend)
#   make lint          → Check code with clippy
# ────────────────────────────────────────────────────────────────────────────

# ── Phony targets (not real files) ─────────────────────────────────────────
.PHONY: init dev build release clean test lint help

# ── Variables ──────────────────────────────────────────────────────────────
TAURI_CLI   = cargo tauri
BACKEND_DIR = src-tauri

# Detect OS to use the right commands
ifeq ($(OS),Windows_NT)
    RM = del /Q /S
else
    RM = rm -rf
endif

# ── Colors (ANSI escape codes — only used in init) ─────────────────────────
RESET  = \033[0m
BOLD   = \033[1m
GREEN  = \033[0;32m
CYAN   = \033[0;36m
YELLOW = \033[0;33m

# ── Default target (first one listed) ─────────────────────────────────────
help:
	@echo "Available targets:"
	@echo "  init      Install all dependencies and set up the project"
	@echo "  add-icon  Add icons to the project from dioxus-iconify"
	@echo "  dev       Run Tauri development server"
	@echo "  build     Clean backend, then build (debug)"
	@echo "  release   Clean backend, then build (release)"
	@echo "  clean     Remove all Rust build artifacts"
	@echo "  test      Run cargo test in the backend"
	@echo "  lint      Run clippy lints on the backend"
	@echo "  help      Show this help"

# ── Project initialisation ─────────────────────────────────────────────────
init:
	@printf "$(YELLOW)[  0%%]$(RESET) $(CYAN)→ Installing Tauri CLI...$(RESET)\n"
	cargo install tauri-cli
	@printf "$(YELLOW)[ 33%%]$(RESET) $(CYAN)→ Installing dioxus-iconify...$(RESET)\n"
	cargo install dioxus-iconify
	@printf "$(YELLOW)[ 66%%]$(RESET) $(CYAN)→ Fetching Rust dependencies...$(RESET)\n"
	cd $(BACKEND_DIR) && cargo fetch
	@printf "$(GREEN)[100%%]$(RESET) $(BOLD)Done!!!$(RESET) Run '$(CYAN)make dev$(RESET)' to start the development server.\n"

# ── Development ────────────────────────────────────────────────────────────
dev:
	$(TAURI_CLI) dev

# ── Build (debug) with cleaning ────────────────────────────────────────────
build:
	cd $(BACKEND_DIR) && cargo clean && cd .. && $(TAURI_CLI) build --debug

# ── Release build with cleaning ────────────────────────────────────────────
release:
	cd $(BACKEND_DIR) && cargo clean && cd .. && $(TAURI_CLI) build --release

# ── Clean (just the Rust part) ─────────────────────────────────────────────
clean:
	cd $(BACKEND_DIR) && cargo clean

# ── Testing and linting ───────────────────────────────────────────────────
test:
	cd $(BACKEND_DIR) && cargo test

lint:
	cd $(BACKEND_DIR) && cargo clippy --all-targets --all-features

# ── Icon management ────────────────────────────────────────────────────────
# Usage: make add-icon ICONS="material-symbols:home-outline"
#        make add-icon ICONS="mdi:home mdi:account"
ICONS ?=

add-icon:
	@if [ -z "$(ICONS)" ]; then \
		echo "Usage: make add-icon ICONS=\"<icon-name> [icon-name ...]\""; \
		exit 1; \
	fi
	dioxus-iconify add $(ICONS)

.PHONY: add-icon
```
