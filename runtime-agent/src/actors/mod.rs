pub mod api;
pub mod connection_manager;
pub mod controller;

// Constants used by the agent controller
pub(crate) const ACTOR_AGENT_API_NAME: &str = "Agent Api";
pub const DATABASE_NAME: &str = "agent.db";
pub const API_SOURCE_NAME: &str = "api";
pub const CONNECTION_STRING_PENDING_STATUS: &str = "pending";
pub const CONNECTION_STRING_ACTIVE_STATUS: &str = "active";
