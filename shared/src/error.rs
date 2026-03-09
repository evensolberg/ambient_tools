//! Typed error variants for the `shared` crate.

use thiserror::Error;

/// Errors that can occur when reading, writing, or serializing configuration.
#[derive(Debug, Error)]
pub enum ConfigError {
    /// A config file could not be read from disk.
    #[error("Cannot read config file '{path}': {source}")]
    Read {
        path: String,
        #[source]
        source: std::io::Error,
    },

    /// A config file could not be written to disk.
    #[error("Cannot write config file '{path}': {source}")]
    Write {
        path: String,
        #[source]
        source: std::io::Error,
    },

    /// A config file exists but its TOML content is invalid.
    #[error("Invalid config file '{path}': {source}. Run 'newconfig' to create a template.")]
    Parse {
        path: String,
        #[source]
        source: toml::de::Error,
    },

    /// The in-memory config could not be serialized to TOML.
    #[error("Failed to serialize config: {0}")]
    Serialize(String),
}
