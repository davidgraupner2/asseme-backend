use std::{env::current_exe, path::PathBuf};

#[cfg(target_os = "windows")]
pub fn home_folder() -> PathBuf {
    current_exe().unwrap().parent().unwrap().to_path_buf()
}

// #[cfg(target_os = "windows")]
// pub fn config_folder() -> PathBuf {
//     home_folder().join("config")
// }

#[cfg(target_os = "windows")]
pub fn jobs_folder() -> PathBuf {
    home_folder().join("jobs")
}

#[cfg(target_os = "windows")]
pub fn additional_files_folder() -> PathBuf {
    home_folder().join("additional_files")
}

#[cfg(target_os = "windows")]
pub fn logs_folder() -> PathBuf {
    home_folder().join("logs")
}

#[cfg(target_os = "linux")]
pub fn home_folder() -> PathBuf {
    current_exe().unwrap().parent().unwrap().to_path_buf()
}

// #[cfg(target_os = "linux")]
// pub fn config_folder() -> PathBuf {
//     home_folder().join("config")
// }

#[cfg(target_os = "linux")]
pub fn jobs_folder() -> PathBuf {
    home_folder().join("jobs")
}

#[cfg(target_os = "linux")]
pub fn additional_files_folder() -> PathBuf {
    home_folder().join("additional_files")
}

#[cfg(target_os = "linux")]
pub fn logs_folder() -> PathBuf {
    home_folder().join("logs")
}

#[cfg(target_os = "macos")]
pub fn home_folder() -> PathBuf {
    current_exe().unwrap().parent().unwrap().to_path_buf()
}

// #[cfg(target_os = "macos")]
// pub fn config_folder() -> PathBuf {
//     home_folder().join("config")
// }

#[cfg(target_os = "macos")]
pub fn jobs_folder() -> PathBuf {
    home_folder().join("jobs")
}

#[cfg(target_os = "macos")]
pub fn additional_files_folder() -> PathBuf {
    home_folder().join("additional_files")
}

#[cfg(target_os = "macos")]
pub fn logs_folder() -> PathBuf {
    home_folder().join("logs")
}

pub fn folders() -> Vec<PathBuf> {
    vec![
        home_folder(),
        // config_folder(),
        jobs_folder(),
        additional_files_folder(),
    ]
}

pub fn ensure_required_folders_exist() {
    for folder in folders() {
        if !folder.exists() {
            std::fs::create_dir_all(folder).unwrap();
        }
    }
}
