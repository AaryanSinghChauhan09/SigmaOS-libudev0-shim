//! libudev.so.0 compatibility shim for SigmaOS.
//!
//! Thin compatibility layer that maps old libudev0 symbols to libudev1.
//! Provides symbol resolution hints and version detection. Dependency-free.

use std::fmt;

/// Errors for shim operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShimError {
    UnsupportedSymbol(String),
    VersionMismatch(String),
}

impl fmt::Display for ShimError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSymbol(s) => write!(f, "unsupported symbol: {s}"),
            Self::VersionMismatch(v) => write!(f, "version mismatch: {v}"),
        }
    }
}

impl std::error::Error for ShimError {}

/// Symbol compatibility mapping.
#[derive(Debug, Clone, PartialEq)]
pub struct SymbolMap {
    version: String,
}

impl SymbolMap {
    pub fn new(version: impl Into<String>) -> Self {
        Self { version: version.into() }
    }

    pub fn resolve(&self, symbol: &str) -> Result<&'static str, ShimError> {
        match symbol {
            "udev_device_new_from_syspath" => Ok("udev_device_new_from_devnum"),
            "udev_device_get_syspath" => Ok("udev_device_get_devnode"),
            "udev_enumerate_add_match_subsystem" => Ok("udev_enumerate_add_match_subsystem"),
            "udev_list_entry_get_name" => Ok("udev_list_entry_get_name"),
            _ => Err(ShimError::UnsupportedSymbol(symbol.to_string())),
        }
    }

    pub fn version(&self) -> &str {
        &self.version
    }
}

/// Compatibility check for libudev0 consumers.
#[derive(Debug, Clone, PartialEq)]
pub struct Libudev0Shim {
    symbol_map: SymbolMap,
}

impl Libudev0Shim {
    pub fn new() -> Self {
        Self { symbol_map: SymbolMap::new("1.0") }
    }

    pub fn check_symbol(&self, symbol: &str) -> Result<bool, ShimError> {
        self.symbol_map.resolve(symbol).map(|_| true)
    }

    pub fn list_mappings(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("udev_device_new_from_syspath", "udev_device_new_from_devnum"),
            ("udev_device_get_syspath", "udev_device_get_devnode"),
            ("udev_enumerate_add_match_subsystem", "udev_enumerate_add_match_subsystem"),
            ("udev_list_entry_get_name", "udev_list_entry_get_name"),
        ]
    }
}

impl Default for Libudev0Shim {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_resolution_aaa() {
        // Arrange
        let shim = Libudev0Shim::new();

        // Act
        let res = shim.check_symbol("udev_device_new_from_syspath");

        // Assert
        assert!(res.is_ok());
        assert!(shim.check_symbol("unknown_func").is_err());
    }

    #[test]
    fn test_mappings_aaa() {
        // Arrange
        let shim = Libudev0Shim::new();

        // Act
        let maps = shim.list_mappings();

        // Assert
        assert_eq!(maps.len(), 4);
        assert_eq!(maps[0].0, "udev_device_new_from_syspath");
    }
}
