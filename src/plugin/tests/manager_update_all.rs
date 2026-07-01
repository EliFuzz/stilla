use crate::plugin::manager::scan_plugin_scripts;
use std::env::temp_dir;
use std::fs::{create_dir_all, remove_dir_all, remove_file, write};

#[test]
fn empty_dir_then_add_script_update_all_picks_up() {
    let dir = temp_dir().join("stilla_empty_then_add");
    remove_dir_all(&dir).ok();
    create_dir_all(&dir).unwrap();

    assert!(scan_plugin_scripts(&dir).is_empty());

    write(dir.join("status.60s.sh"), "#!/bin/sh\necho 'hello'").unwrap();

    assert_eq!(scan_plugin_scripts(&dir).len(), 1);
}

#[test]
fn script_on_start_then_removed_update_all_drops_plugin() {
    let dir = temp_dir().join("stilla_remove_after_start");
    remove_dir_all(&dir).ok();
    create_dir_all(&dir).unwrap();

    let script = dir.join("status.60s.sh");
    write(&script, "#!/bin/sh\necho 'hello'").unwrap();

    assert_eq!(scan_plugin_scripts(&dir).len(), 1);

    remove_file(&script).unwrap();

    assert!(scan_plugin_scripts(&dir).is_empty());
}
