//! `YIS_HOME` override tests in an isolated binary so `yis_home()`'s
//! process-wide `OnceLock` initializes from the overridden env var.

use std::path::PathBuf;

#[test]
fn yis_home_override_path_helpers() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let yis_home = tmp.path().to_path_buf();
    unsafe {
        std::env::set_var("YIS_HOME", &yis_home);
    }

    assert_eq!(
        xai_yis_pager::util::pager_toml_path(),
        yis_home.join("pager.toml")
    );
    assert_eq!(
        xai_yis_pager::util::display_yis_home_prefix(),
        "$YIS_HOME"
    );
    assert_eq!(
        xai_yis_pager::util::display_user_yis_path("config.toml"),
        "$YIS_HOME/config.toml"
    );

    let memory_path = yis_home.join("memory/MEMORY.md");
    assert_eq!(
        xai_yis_pager::util::abbreviate_path(&memory_path.display().to_string()),
        "$YIS_HOME/memory/MEMORY.md"
    );

    assert!(xai_yis_pager::util::is_under_user_yis_home(&memory_path));
    assert!(!xai_yis_pager::util::is_under_user_yis_home(
        PathBuf::from("/tmp/other").as_path()
    ));
}
