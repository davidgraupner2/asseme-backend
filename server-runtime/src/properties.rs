use crate::{
    folders::{config_folder, logs_folder},
};
use gethostname::gethostname;
use machineid_rs::{Encryption, HWIDComponent, IdBuilder};

pub fn runtime_version() -> String {
    option_env!("CARGO_PKG_VERSION").unwrap_or("0.0.0").into()
}

pub fn hostname() -> String {
    gethostname()
        .into_string()
        .unwrap_or("".to_string())
        .to_string()
}

pub fn config_file_name() -> String {
    config_folder()
        .join("config.toml")
        .into_os_string()
        .into_string()
        .unwrap()
}

pub fn log_file_name() -> String {
    logs_folder()
        .join("agent.log")
        .into_os_string()
        .into_string()
        .unwrap()
}

pub fn exe_file_name() -> String {
    std::env::current_exe()
        .unwrap()
        .with_extension("")
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

pub fn runtime_id() -> String {
    let mut hardware_id_builder = IdBuilder::new(Encryption::SHA256);
    hardware_id_builder
        .add_component(HWIDComponent::SystemID)
        .add_component(HWIDComponent::CPUID)
        .add_component(HWIDComponent::OSName);

    hardware_id_builder.build("agent_id").unwrap()
}

