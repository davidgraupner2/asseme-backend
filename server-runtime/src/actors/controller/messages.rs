use ractor::{Actor, RpcReplyPort};

use crate::actors::controller::Controller;

#[derive(Debug)]
pub enum ControllerMessage {
    Shutdown,
}
