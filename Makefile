# ─── Tauri + Dioxus Makefile ───────────────────────────────────────────────
# Works on Linux, macOS, and Windows (with make installed)
# Usage:
#   make dev           → Start development server
#   make build         → Clean & build the app (debug)
#   make release       → Clean & build the app (release)
#   make clean         → Remove all build artifacts
#   make test          → Run Rust tests (backend)
#   make lint          → Check code with clippy
# ────────────────────────────────────────────────────────────────────────────

# ── Phony targets (not real files) ─────────────────────────────────────────
.PHONY: dev build release clean test lint help

# ── Variables ──────────────────────────────────────────────────────────────
TAURI_CLI  = cargo tauri
BACKEND_DIR = src-tauri

# Detect OS to use the right commands (optional – here only for show)
ifeq ($(OS),Windows_NT)
    RM = del /Q /S
    # On Windows we avoid shell commands that don't exist
else
    RM = rm -rf
endif

# ── Default target (first one listed) ─────────────────────────────────────
help:
	@echo "Available targets:"
	@echo "  add-icon  Add icons to the project form dioxus-conify"
	@echo "  dev       Run Tauri development server"
	@echo "  build     Clean backend, then build (debug)"
	@echo "  release   Clean backend, then build (release)"
	@echo "  clean     Remove all Rust build artifacts"
	@echo "  test      Run cargo test in the backend"
	@echo "  lint      Run clippy lints on the backend"
	@echo "  help      Show this help"

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

# ── Testing and linting ────────────────────────────────────────────────────
test:
	cd $(BACKEND_DIR) && cargo test

lint:
	cd $(BACKEND_DIR) && cargo clippy --all-targets --all-features

# ── Icon management ────────────────────────────────────────────────────────
# Usage: make add-icon ICONS="material-symbols:home-outline"
#        make add-icon ICONS="mdi:home mdi:account"
ICON ?=                       # default empty if not provided

add-icon:
	@if [ -z "$(ICONS)" ]; then \
		echo "Usage: make add-icon ICONS=\"<icon-name> [icon-name ...]\""; \
		exit 1; \
	fi
	dioxus-iconify add $(ICONS)

.PHONY: add-icon
