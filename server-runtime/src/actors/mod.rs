pub mod api;

pub enum ActorStatus {
    INITIALISING,
    RUNNING,
    STOPPING,
    STOPPED,
}