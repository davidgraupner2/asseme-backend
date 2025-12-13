pub mod actors;

// Global Constants
pub const DATABASE_NAME: &str = "agent.db";

// Constants used by the agent controller
pub(crate) const ACTOR_AGENT_API_NAME: &str = "Agent Api";
pub(crate) const CONNECTION_STRING_PENDING_STATUS: &str = "pending";
pub(crate) const CONNECTION_STRING_ACTIVE_STATUS: &str = "active";

// Default Property names used for configuration
pub(crate) const PROPERTY_API_PORT: &str = "api_port";
pub(crate) const PROPERTY_LOGGING_FORMAT: &str = "logging::format";
pub(crate) const PROPERTY_LOGGING_LEVEL: &str = "logging::level";

// Property defaults, if property names not loaded into the database
pub(crate) const DEFAULT_PROPERTY_API_PORT: i32 = 8174;
pub(crate) const DEFAULT_PROPERTY_LOGGING_FORMAT: &str = "pretty";
pub(crate) const DEFAULT_PROPERTY_LOGGING_LEVEL: &str = "error";

pub use crate::actors::controller::actor::Controller as AgentRuntimeController;
pub use crate::actors::controller::arguments::AgentControllerArguments;
