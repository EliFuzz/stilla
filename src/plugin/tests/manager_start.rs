use crate::plugin::manager::scan_plugin_scripts;
use std::env::temp_dir;
use std::fs::{create_dir_all, remove_dir_all, rename, write};

#[test]
fn picks_up_script_from_dir() {
    let dir = temp_dir().join("stilla_picks_up_script");
    remove_dir_all(&dir).ok();
    create_dir_all(&dir).unwrap();
    write(dir.join("test.60s.sh"), "").unwrap();

    assert_eq!(scan_plugin_scripts(&dir).len(), 1);
}

#[test]
fn no_crash_without_scripts() {
    let dir = temp_dir().join("stilla_no_scripts");
    remove_dir_all(&dir).ok();
    create_dir_all(&dir).unwrap();

    assert!(scan_plugin_scripts(&dir).is_empty());
}

#[test]
fn update_all_removes_old_and_picks_up_renamed() {
    let dir = temp_dir().join("stilla_rename_test");
    remove_dir_all(&dir).ok();
    create_dir_all(&dir).unwrap();

    let old_path = dir.join("original.60s.sh");
    let new_path = dir.join("renamed.60s.sh");
    write(&old_path, "").unwrap();

    let before = scan_plugin_scripts(&dir);
    assert_eq!(before.len(), 1);
    assert_eq!(before[0].0, old_path);

    rename(&old_path, &new_path).unwrap();

    let after = scan_plugin_scripts(&dir);
    assert_eq!(after.len(), 1);
    assert_eq!(after[0].0, new_path);
    assert!(!after.iter().any(|(p, _, _)| p == &old_path));
}
