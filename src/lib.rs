//! # Nodoxus
//!
//! High-performance, reactive node graph and flow engine for Dioxus.
//! Blends virtual canvas performance with DOM-like interactivity.

#![warn(missing_docs)]

/// Early scaffold version of nodoxus.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}
