pub mod api;
pub mod logging;
pub mod settings;

// Some agent wide constants
pub const DEFAULT_API_PORT: u16 = 8173;
pub const DEFAULT_LOG_LEVEL: &str = "error";
pub const DEFAULT_LOG_FORMAT: &str = "pretty";

// Public Re-Exports
pub use settings::AgentSettings;
