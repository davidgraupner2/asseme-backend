use server_runtime::RuntimeProperties;
use std::time::{SystemTime, UNIX_EPOCH};

/// Verify `RuntimeProperties::new_with_base` produces sensible values and paths.
#[test]
fn new_with_base_populates_fields_and_paths() {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let app_name = format!("unit_test_app_{}", ts);

    let base = std::env::temp_dir().join(format!("server_runtime_props_{}", ts));
    std::fs::create_dir_all(&base).unwrap();

    // Create instance without touching global OnceLock
    let props = RuntimeProperties::new_with_base(&app_name, &base);

    // version should match the compile-time package version or default
    let expected_version = option_env!("CARGO_PKG_VERSION").unwrap_or("0.0.0");
    assert_eq!(props.version(), expected_version);

    // exe_name and id should be present
    assert!(!props.exe_name().is_empty(), "exe_name should not be empty");
    assert!(!props.id().is_empty(), "id should not be empty");

    // Folder paths should be under base
    let logs = props.folders().logs();
    let supp = props.folders().supplementary_files();
    let jobs = props.folders().jobs();

    assert_eq!(logs, &base.join("logs"));
    assert_eq!(supp, &base.join("supplementary_files"));
    assert_eq!(jobs, &base.join("jobs"));

    // ensure_exists should have been called by new_with_base, so directories exist
    assert!(logs.exists(), "logs dir should exist");
    assert!(supp.exists(), "supp dir should exist");
    assert!(jobs.exists(), "jobs dir should exist");

    // Files: log filename should be base.join("<app>.log")
    let expected_log = base.join("logs").join(format!("{}.log", app_name));
    assert_eq!(props.files().log_file_name(), expected_log.as_path());

    // Cleanup
    let _ = std::fs::remove_dir_all(&base);
}
