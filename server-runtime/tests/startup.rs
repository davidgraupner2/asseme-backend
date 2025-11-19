use server_runtime::RuntimeProperties;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

/// Integration test: initialise runtime properties and verify directories are created.
#[test]
fn runtime_init_creates_dirs() {
    // Unique app name to avoid collisions
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let app_name = format!("test_app_{}", ts);

    // Initialize runtime with a temporary base directory to avoid touching
    // system locations.
    let base = std::env::temp_dir().join(format!("server_runtime_test_{}", ts));
    std::fs::create_dir_all(&base).unwrap();

    // Initialize runtime (this will call Folders::ensure_exists()) using base
    RuntimeProperties::init_with_base(&app_name, &base);

    let props = RuntimeProperties::global();
    let logs = props.folders().logs().to_path_buf();
    let supp = props.folders().supplementary_files().to_path_buf();
    let jobs = props.folders().jobs().to_path_buf();

    assert!(logs.exists(), "logs folder was not created: {:?}", logs);
    assert!(
        supp.exists(),
        "supplementary folder was not created: {:?}",
        supp
    );
    assert!(jobs.exists(), "jobs folder was not created: {:?}", jobs);

    // Clean up only when these paths are inside the crate workspace to avoid
    // accidentally deleting system directories like /var/log.
    // Clean up the temporary base directory we created above.
    let _ = fs::remove_dir_all(&base);
}
