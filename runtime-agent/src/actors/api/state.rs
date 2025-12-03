use runtime_shared::RuntimeProperties;

#[derive(Clone, Debug)]
pub(crate) struct ApiState {
    pub id: String,
}

impl ApiState {
    pub fn new() -> Self {
        let runtime_properties = RuntimeProperties::global();
        Self {
            id: format!("agent:{}", runtime_properties.id()),
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct V1ApiState {
    pub id: String,
}

impl V1ApiState {
    pub fn new() -> Self {
        let runtime_properties = RuntimeProperties::global();

        Self {
            id: format!("api:v1:{}", runtime_properties.id()),
        }
    }
}

#[derive(Debug)]
pub struct ApiActorState {
    pub server_handle: Option<axum_server::Handle>,
}

impl ApiActorState {
    pub fn new() -> Self {
        Self {
            server_handle: None,
        }
    }
}
