use ractor::RpcReplyPort;

#[derive(Debug)]
pub enum ControllerMessage {
    // REPL Commands
    // GetStatus(RpcReplyPort<String>),
    GetConfig(RpcReplyPort<String>),
    // GetDatabaseStatus(RpcReplyPort<String>),

    // Control Commands
    ReloadApiServer,
    RestartApiServer,
    ReloadCertificates,
    ReloadConfig,
    RecreateDatabase,
    RunMigrations,
    SetLogLevel(tracing::Level),
}
