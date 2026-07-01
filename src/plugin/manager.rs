use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

use fs::read_dir;
use tao::event_loop::EventLoopProxy;
use tray_icon::menu::MenuId;

use crate::AppEvent;
use crate::plugin::{Action, Plugin};
use crate::utils::{ScriptResult, plugins_dir, time_parse};

pub fn scan_plugin_scripts(dir: &Path) -> Vec<(PathBuf, Duration, bool)> {
    let Some(entries) = read_dir(dir).ok() else {
        return vec![];
    };
    entries
        .filter_map(|e| e.ok())
        .filter_map(|entry| {
            let file_name = entry.file_name();
            let name = file_name.to_string_lossy();
            let mut parts = name.splitn(3, '.');
            let _ = parts.next()?;
            let interval_part = parts.next()?;
            if parts.next()? != "sh" || parts.next().is_some() {
                return None;
            }
            let notify = interval_part.ends_with('n');
            let interval = time_parse(interval_part);
            Some((entry.path(), interval, notify))
        })
        .collect()
}

pub struct PluginManager {
    plugins: HashMap<u64, Plugin>,
    menu_to_plugin: HashMap<MenuId, u64>,
    proxy: EventLoopProxy<AppEvent>,
}

impl PluginManager {
    pub fn new(proxy: EventLoopProxy<AppEvent>) -> Self {
        Self {
            plugins: HashMap::new(),
            menu_to_plugin: HashMap::new(),
            proxy,
        }
    }

    pub fn start(&mut self) {
        self.terminate();
        for (path, interval, notify) in scan_plugin_scripts(plugins_dir()) {
            let mut p = Plugin::new(path, interval, notify);
            p.start(&self.proxy);
            self.plugins.insert(p.id, p);
        }
    }

    pub fn terminate(&mut self) {
        for p in self.plugins.values_mut() {
            p.terminate();
        }
        self.plugins.clear();
        self.menu_to_plugin.clear();
    }

    pub fn handle_click(&mut self, id: &MenuId) {
        let Some(&plugin_id) = self.menu_to_plugin.get(id) else {
            return;
        };
        let Some(plugin) = self.plugins.get_mut(&plugin_id) else {
            return;
        };
        if matches!(plugin.actions.get(id), Some(Action::Refresh)) {
            plugin.start(&self.proxy);
            return;
        }
        if let Some(Action::OpenUrl(url)) = plugin.actions.get(id) {
            open::that(url).ok();
        }
    }

    pub fn apply_output(&mut self, id: u64, output: Option<ScriptResult>) {
        let Some(plugin) = self.plugins.get_mut(&id) else {
            return;
        };
        for menu_id in plugin.actions.keys() {
            self.menu_to_plugin.remove(menu_id);
        }
        if plugin.apply_output(output) {
            plugin.terminate();
            self.plugins.remove(&id);
            return;
        }
        let plugin = self.plugins.get(&id).unwrap();
        for menu_id in plugin.actions.keys() {
            self.menu_to_plugin.insert(menu_id.clone(), id);
        }
    }
}
