<p align="center">
  <img src="https://raw.githubusercontent.com/techton7/nodoxus/main/assets/icon.svg" alt="nodoxus logo" width="160" height="160" />
</p>

<h1 align="center">nodoxus</h1>

<p align="center">
  <strong>High-performance, reactive node graph and flow engine for Dioxus.</strong>
</p>

<p align="center">
  <a href="https://crates.io/crates/nodoxus"><img src="https://img.shields.io/crates/v/nodoxus.svg" alt="Crates.io" /></a>
  <a href="https://docs.rs/nodoxus"><img src="https://docs.rs/nodoxus/badge.svg" alt="docs.rs" /></a>
  <a href="LICENSE-MIT"><img src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg" alt="License" /></a>
</p>

---

**Nodoxus** is a next-generation, high-performance node-edge graph and workflow canvas engine purpose-built for the [Dioxus](https://dioxuslabs.com) ecosystem in Rust.

It blends the raw performance of a virtual scene graph with the reactive, composable ergonomics of Dioxus components, enabling fluid 60fps interaction across thousands of nodes.

## Vision & Architecture

- **High-Performance Virtual Scene**: Bypasses full-DOM overhead for intensive graph layouts and streaming data, maintaining smooth pan/zoom/drag physics.
- **Hybrid DOM Interactivity**: Seamlessly mounts rich Dioxus RSX components and interactive controls inside or alongside nodes whenever deep UI interaction is needed.
- **Reactive State Flow**: Native Dioxus signal integration for fine-grained node and edge updates without canvas-wide rerenders.
- **Deterministic Routing**: Smart edge routing, customizable port anchors, and crossing-minimized solver pipelines.

## Status

Early scaffold release (`v0.1.0`). Package metadata, repository scaffolding, and continuous release pipelines are configured. Core engine abstractions and canvas runtime are actively in progress.

## License

Dual-licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
