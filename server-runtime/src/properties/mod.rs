pub mod files;
pub mod folders;

use machineid_rs::{Encryption, HWIDComponent, IdBuilder};
use std::sync::OnceLock;
use sysinfo::System;

use crate::properties::{files::Files, folders::Folders};
static RUNTIME_PROPERTIES: OnceLock<RuntimeProperties> = OnceLock::new();

#[derive(Debug)]
pub struct RuntimeProperties {
    version: Box<str>,
    name: Box<str>,
    host_name: Box<str>,
    exe_name: Box<str>,
    id: Box<str>,
    folders: Box<Folders>,
    files: Box<Files>,
}

impl RuntimeProperties {
    /// Initialize the runtime properties globally. Must be called once at application startup.
    ///
    /// # Panics
    /// Panics if called more than once.
    ///
    /// # Example
    /// ```
    /// use server_runtime::RuntimeProperties;
    ///
    /// fn main() {
    ///     RuntimeProperties::init("Server App");
    /// }
    /// ```
    pub fn init(app_name: &str) {
        RUNTIME_PROPERTIES
            .set(RuntimeProperties::new(app_name))
            .expect("RUNTIME_PROPERTIES already initialised");
    }

    /// Initialize runtime properties with a custom base directory for folders.
    ///
    /// Useful for tests: callers can pass a temporary directory to avoid
    /// creating folders under system paths like `/var/log`.
    pub fn init_with_base(app_name: &str, base: &std::path::Path) {
        RUNTIME_PROPERTIES
            .set(RuntimeProperties::new_with_base(app_name, base))
            .expect("RUNTIME_PROPERTIES already initialised");
    }

    /// Get a reference to the global runtime properties.
    ///
    /// # Panics
    /// Panics if `RuntimeProperties::init()` hasn't been called yet.
    pub fn global() -> &'static RuntimeProperties {
        RUNTIME_PROPERTIES
            .get()
            .expect("RUNTIME_PROPERTIES not initialized. Call RuntimeProperties::init() first.")
    }

    pub fn new(app_name: &str) -> Self {
        let version = option_env!("CARGO_PKG_VERSION")
            .unwrap_or("0.0.0")
            .to_string()
            .into_boxed_str();
        let name = System::name().unwrap_or_default().into_boxed_str();
        let host_name = System::host_name().unwrap_or_default().into_boxed_str();
        let exe_name = std::env::current_exe()
            .unwrap()
            .with_extension("")
            .file_name()
            .unwrap()
            .to_str()
            .unwrap_or("default")
            .to_string()
            .into_boxed_str();
        let id = runtime_id().into_boxed_str();
        let folders = Box::new(Folders::new(app_name.to_lowercase().as_str()).ensure_exists());
        let files = Box::new(Files::new(app_name.to_lowercase().as_str(), folders.logs()));

        Self {
            version,
            name,
            host_name,
            exe_name,
            id,
            folders,
            files,
        }
    }

    /// Create a `RuntimeProperties` instance but use an explicit base path for
    /// folder creation. This mirrors `init_with_base` but returns the instance
    /// so callers (tests) can inspect it without initializing the global.
    pub fn new_with_base(app_name: &str, base: &std::path::Path) -> Self {
        let version = option_env!("CARGO_PKG_VERSION")
            .unwrap_or("0.0.0")
            .to_string()
            .into_boxed_str();
        let name = System::name().unwrap_or_default().into_boxed_str();
        let host_name = System::host_name().unwrap_or_default().into_boxed_str();
        let exe_name = std::env::current_exe()
            .unwrap()
            .with_extension("")
            .file_name()
            .unwrap()
            .to_str()
            .unwrap_or("default")
            .to_string()
            .into_boxed_str();
        let id = runtime_id().into_boxed_str();
        let folders = Box::new(
            Folders::new_with_base(app_name.to_lowercase().as_str(), base).ensure_exists(),
        );
        let files = Box::new(Files::new(app_name.to_lowercase().as_str(), folders.logs()));

        Self {
            version,
            name,
            host_name,
            exe_name,
            id,
            folders,
            files,
        }
    }

    // Accessor methods
    pub fn version(&self) -> &str {
        &self.version
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn host_name(&self) -> &str {
        &self.host_name
    }
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn exe_name(&self) -> &str {
        &self.exe_name
    }
    pub fn folders(&self) -> &Folders {
        &self.folders
    }
    pub fn files(&self) -> &Files {
        &self.files
    }
}

fn runtime_id() -> String {
    let mut hardware_id_builder = IdBuilder::new(Encryption::SHA256);
    hardware_id_builder
        .add_component(HWIDComponent::SystemID)
        .add_component(HWIDComponent::CPUID)
        .add_component(HWIDComponent::OSName);

    hardware_id_builder.build("id").unwrap()
}
