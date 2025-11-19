use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Files {
    log_file_name: PathBuf,
}

impl Files {
    /// Construct a `Files` instance where the log file is the joined path of
    /// `log_folder` and `<app_name>.log`.
    pub fn new(app_name: &str, log_folder: &Path) -> Self {
        let log_file_name = log_folder.join(format!("{}.log", app_name));

        Self { log_file_name }
    }

    // Accessor methods
    /// Return the path to the log file.
    pub fn log_file_name(&self) -> &Path {
        &self.log_file_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::temp_dir;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn log_file_path_is_joined() {
        // Use timestamp to avoid collisions when tests run in parallel
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let base = temp_dir().join(format!("server_runtime_test_{}", ts));
        let files = Files::new("asseme", &base);
        let expected = base.join("asseme.log");

        assert_eq!(files.log_file_name(), expected.as_path());
    }
}
