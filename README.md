<div align="center">

# 🦁 Leo Explorer

**A next-generation, AI-augmented file management platform built entirely in Rust.**  
Cross-platform. Intelligent. Open source.

<br/>

![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)
![Rust](https://img.shields.io/badge/built_with-Rust-orange?style=flat-square&logo=rust)
![Tauri](https://img.shields.io/badge/Tauri-2.0-blue?style=flat-square&logo=tauri)
![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey?style=flat-square)
![Status](https://img.shields.io/badge/status-active_development-green?style=flat-square)
![Award](https://img.shields.io/badge/🏆_Best_Design_Project-ACLC_Mandaue_2025-gold?style=flat-square)

<br/>

> *"File management reimagined — not just a tool, but an intelligent workspace."*

</div>

---

## Table of Contents

- [Story Behind The Project](#story-behind-the-project)
- [Thesis Background](#thesis-background)
- [Why Leo Explorer Exists](#why-leo-explorer-exists)
- [Features](#features)
- [Technology Stack](#technology-stack)
- [Project Roadmap](#project-roadmap)
- [Getting Started](#getting-started)
- [Open Source Vision](#open-source-vision)
- [Contributors](#contributors)
- [Acknowledgements](#acknowledgements)

---

## Story Behind The Project

Leo Explorer started as a personal side project by **Cedrick C. Alegsao** — born out of a single frustration: every file explorer available, across every major operating system, felt like it was designed for a different era.

Windows Explorer, macOS Finder, and the various Linux file managers all share the same fundamental DNA. They browse directories. They move files. They do little else. In an age where AI assistants can write code, summarize documents, and manage entire workflows, our file explorers remain stubbornly ordinary.

What began as a late-night experiment quickly grew into something more ambitious. As the scope expanded — AI-assisted organization, intelligent search, multithreaded operations, cross-platform support — it became clear that Leo Explorer was not just a personal tool. It was a thesis.

The project was formally proposed at **ACLC College of Mandaue** as a thesis under the Bachelor of Science in Computer Science program, jointly developed by Cedrick and his co-proponent, **Joshua Mark H. Playda**. After months of research, design, and development, the project was successfully defended and recognized with the **Best Design Project Award** at the ACLC College of Mandaue Graduation Ceremony on **May 22, 2025**.

Today, Leo Explorer lives on as an open-source project — rebuilt from the ground up, more powerful than ever, and open to the world.

---

## Thesis Background

| Detail | Information |
|---|---|
| **Thesis Title** | *Leo Explorer: An AI-Augmented, Multithreaded File Management Solution for Modern Operating Systems* |
| **Institution** | ACLC College of Mandaue |
| **Program** | Bachelor of Science in Computer Science |
| **Subjects** | Design Project 1 & Design Project 2 |
| **Proponents** | Cedrick C. Alegsao, Joshua Mark H. Playda |
| **Status** | Successfully Defended ✅ |
| **Recognition** | 🏆 Best Design Project Award — Graduation Ceremony, May 22, 2025 |

The thesis explored the intersection of AI-assisted workflows, modern systems programming, and human-computer interaction in the context of file management. The research identified critical gaps in existing solutions and proposed a new architecture built on performance, intelligence, and extensibility.

---

## Why Leo Explorer Exists

Traditional file explorers were designed when files were simple, storage was local, and workflows were linear. The modern reality is vastly different:

- **Files are everywhere** — local drives, cloud storage, network shares, and external devices all coexist.
- **Workflows are complex** — creative professionals, developers, and researchers work across hundreds of files simultaneously.
- **Search is still broken** — most native search tools are slow, shallow, and context-unaware.
- **AI is absent** — despite the explosion of AI tooling in every other productivity category, file management has been left behind.

Leo Explorer was created to answer these gaps directly. It treats the file system as a living, intelligent workspace — one that understands context, learns from usage patterns, and helps users find, organize, and act on their files faster than ever before.

---

## Features

### ✅ Current Features

| Feature | Description |
|---|---|
| 🖥️ **Modern UI** | Clean, responsive interface built with Dioxus — native performance, web-quality design |
| 🌍 **Cross-platform** | Runs natively on Windows, macOS, and Linux from a single codebase |
| ⚡ **Multithreaded Operations** | File operations run concurrently — copy, move, index, and search without blocking the UI |
| 🔍 **Fast Search** | Real-time, indexed search across file names, types, and metadata |

### 🔭 Planned Features

| Feature | Description |
|---|---|
| 🤖 **AI-Assisted File Management** | Smart tagging, automatic organization, and content-aware suggestions powered by AI |
| 📑 **File Indexing Engine** | Deep background indexing for instant access across large file systems |
| 🗂️ **Workspace Organization** | Save, switch between, and share project-scoped workspaces |
| 🔌 **Plugin Architecture** | Extend Leo Explorer with community-built plugins for custom workflows |
| ☁️ **Cloud Integrations** | First-class support for Google Drive, OneDrive, Dropbox, and S3-compatible storage |
| 🌐 **Extensible Ecosystem** | Open API for third-party developers to build on top of Leo Explorer |

---

## Technology Stack

| Layer | Technology | Purpose |
|---|---|---|
| **Systems Core** | [Rust](https://www.rust-lang.org/) | Performance-critical backend logic, file system operations, threading |
| **Desktop Runtime** | [Tauri 2.0](https://tauri.app/) | Cross-platform native window, OS integration, secure IPC bridge |
| **UI Framework** | [Dioxus](https://dioxuslabs.com/) | Reactive, Rust-native UI — no JavaScript runtime required |

> The entire stack is Rust. No Electron. No Node.js. No compromises on performance or binary size.

---

## Project Roadmap

```
Phase 1 ──── Phase 2 ──── Phase 3 ──── Phase 4 ──── Phase 5
  Core       Indexing      AI Core     Plugins       Cloud
```

### Phase 1 — Core File Explorer
> *Foundation*

- [ ] Directory navigation and browsing
- [ ] File operations: copy, move, rename, delete
- [ ] Dual-pane layout
- [ ] Keyboard-first navigation
- [ ] Bookmarks and pinned locations
- [ ] Cross-platform builds (Windows, macOS, Linux)

### Phase 2 — Performance and Indexing
> *Speed*

- [ ] Background file indexing engine
- [ ] Real-time search with instant results
- [ ] File metadata extraction and caching
- [ ] Multithreaded bulk operations with progress tracking
- [ ] Directory size calculation

### Phase 3 — AI Features
> *Intelligence*

- [ ] AI-powered file tagging and categorization
- [ ] Natural language search ("show me large videos from last month")
- [ ] Smart duplicate detection
- [ ] Context-aware file suggestions
- [ ] Document summarization preview

### Phase 4 — Plugin Ecosystem
> *Extensibility*

- [ ] Plugin API design and documentation
- [ ] Plugin manager with install/update/remove
- [ ] Community plugin registry
- [ ] First-party plugins: Git integration, image viewer, code preview

### Phase 5 — Cloud and Collaboration
> *Connectivity*

- [ ] Google Drive, OneDrive, Dropbox integration
- [ ] S3-compatible remote storage support
- [ ] Shared workspaces for teams
- [ ] File sync and conflict resolution

---

## Getting Started

> ⚠️ Leo Explorer is currently in active development. Pre-release builds are not yet available.

To build from source, ensure you have the prerequisites installed, then run:

```bash
# Clone the repository
git clone https://github.com/your-username/leo-explorer.git
cd leo-explorer

# Install all dependencies and tools
make init

# Start the development server
make dev
```

For a full list of available `make` commands, see [MAKE.md](./MAKE.md) or run:

```bash
make
```

---

## Open Source Vision

Leo Explorer is built in the open because the best tools are built together.

The long-term vision is to create a **community-driven file management platform** — one that grows with the needs of its users rather than the priorities of a corporation. Whether you are a developer who wants to contribute code, a designer who wants to improve the UX, a researcher pushing the boundaries of AI integration, or simply a power user with great ideas — there is a place for you here.

The project is committed to:

- **Transparency** — development happens publicly, decisions are documented, and the roadmap is open to feedback.
- **Inclusivity** — contributions of all kinds are welcome, from code to documentation to bug reports.
- **Quality** — we move deliberately. Every feature ships when it is right, not when it is rushed.
- **Longevity** — Leo Explorer is built to last. The architecture, the tooling, and the community are all designed with the long game in mind.

If you believe file management deserves better, star the repository, open an issue, or submit a pull request. Every contribution moves the project forward.

---

## Contributors

<table>
  <tr>
    <td align="center">
      <b>Cedrick C. Alegsao</b><br/>
      <sub>Project Founder · Lead Developer · Thesis Proponent</sub><br/><br/>
      <sub>Conceived and initiated Leo Explorer as a personal project. Led the overall architecture, systems design, and development of both the original thesis build and the current open-source rewrite.</sub>
    </td>
    <td align="center">
      <b>Joshua Mark H. Playda</b><br/>
      <sub>Thesis Proponent · Research & Development</sub><br/><br/>
      <sub>Co-developed the thesis project at ACLC College of Mandaue. Contributed to research, feature design, and the academic documentation underlying Leo Explorer's design principles.</sub>
    </td>
  </tr>
</table>

---

## Acknowledgements

Leo Explorer would not exist without the support of people and institutions who believed in the project from the very beginning.

**ACLC College of Mandaue** — for providing the academic environment and the platform through which this project was shaped into a formal thesis and recognized for its contribution to design and technology.

**Faculty and Thesis Advisers** — for the guidance, constructive feedback, and mentorship that pushed the project beyond a prototype into something worth defending.

**Thesis Panelists and Evaluators** — for the rigorous critique that made Leo Explorer stronger.

**Testers and Early Users** — for the honest feedback, the bug reports, and the encouragement to keep going when the work was hard.

**The Rust, Tauri, and Dioxus communities** — for building the exceptional open-source tooling that makes Leo Explorer possible in the first place.

And to everyone who stars, forks, opens an issue, or simply shares the project — thank you. You are part of the story now.

---

<div align="center">

Built with 🦀 Rust · Powered by community · Inspired by curiosity

<br/>

*Leo Explorer — because your files deserve a smarter home.*

</div>
