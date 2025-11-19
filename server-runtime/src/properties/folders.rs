use std::path::Path;
use std::{env::current_exe, path::PathBuf};

#[derive(Debug, Clone)]
pub struct Folders {
    home_folder: PathBuf,
    supplementrary_files_folder: PathBuf,
    jobs_folder: PathBuf,
    logs_folder: PathBuf,
}

impl Folders {
    #[cfg(target_os = "windows")]
    pub fn new(app_name: &str) -> Self {
        let home_folder = current_exe().unwrap().parent().unwrap().to_path_buf();
        let supplementrary_files_folder = home_folder.join("supplementary_files");
        let logs_folder = home_folder.join("logs");
        let jobs_folder = home_folder.join("jobs");

        Self {
            home_folder,
            logs_folder,
            supplementrary_files_folder,
            jobs_folder,
        }
    }
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub fn new(#[cfg_attr(debug_assertions, allow(unused_variables))] app_name: &str) -> Self {
        let home_folder = current_exe().unwrap().parent().unwrap().to_path_buf();

        let supplementrary_files_folder;
        let logs_folder;
        let jobs_folder;

        #[cfg(debug_assertions)]
        {
            supplementrary_files_folder = home_folder.join("supplementary_files");
            logs_folder = home_folder.join("logs");
            jobs_folder = home_folder.join("jobs");
        }

        #[cfg(not(debug_assertions))]
        {
            supplementrary_files_folder = home_folder.join("supplementary_files");
            logs_folder = PathBuf::from("/").join("var").join("log").join(app_name);
            jobs_folder = PathBuf::from("/").join("opt").join(app_name).join("jobs");
        }

        Self {
            home_folder,
            logs_folder,
            supplementrary_files_folder,
            jobs_folder,
        }
    }

    /// Construct folders using an explicit base path. This is useful for tests
    /// where you want to control where folders are created.
    pub fn new_with_base(_app_name: &str, base: &Path) -> Self {
        let home_folder = base.to_path_buf();

        #[cfg(debug_assertions)]
        let supplementrary_files_folder = home_folder.join("supplementary_files");

        #[cfg(not(debug_assertions))]
        let supplementrary_files_folder = home_folder.join("supplementary_files");

        #[cfg(debug_assertions)]
        let logs_folder = home_folder.join("logs");

        #[cfg(not(debug_assertions))]
        let logs_folder = home_folder.join("logs");

        #[cfg(debug_assertions)]
        let jobs_folder = home_folder.join("jobs");

        #[cfg(not(debug_assertions))]
        let jobs_folder = home_folder.join("jobs");

        Self {
            home_folder,
            logs_folder,
            supplementrary_files_folder,
            jobs_folder,
        }
    }

    pub fn ensure_exists(self) -> Self {
        if !self.home_folder.exists() {
            std::fs::create_dir_all(&self.home_folder).unwrap();
        }

        if !self.supplementrary_files_folder.exists() {
            std::fs::create_dir_all(&self.supplementrary_files_folder).unwrap();
        }

        if !self.logs_folder.exists() {
            std::fs::create_dir_all(&self.logs_folder).unwrap();
        }

        if !self.jobs_folder.exists() {
            std::fs::create_dir_all(&self.jobs_folder).unwrap();
        }

        self
    }

    // Accessor methods
    pub fn home(&self) -> &PathBuf {
        &self.home_folder
    }
    pub fn supplementary_files(&self) -> &PathBuf {
        &self.supplementrary_files_folder
    }
    pub fn logs(&self) -> &PathBuf {
        &self.logs_folder
    }
    pub fn jobs(&self) -> &PathBuf {
        &self.jobs_folder
    }
}
