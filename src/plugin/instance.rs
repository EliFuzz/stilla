use crate::AppEvent;
use crate::plugin::{hash_output, notification_hash, set_menu};
use crate::utils::{ScriptResult, notify, run_script};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};
use tao::event_loop::EventLoopProxy;
use tokio::spawn;
use tokio::task::JoinHandle;
use tokio::time::sleep;
use tray_icon::TrayIconBuilder;
use tray_icon::menu::MenuId;
use xxhash_rust::xxh3::xxh3_64;

pub struct Plugin {
    pub id: u64,
    pub path: Arc<Path>,
    pub interval: Duration,
    pub notify: bool,
    tray: tray_icon::TrayIcon,
    cached: Option<u64>,
    output_hash: Option<u64>,
    pub actions: HashMap<MenuId, Action>,
    task: Option<JoinHandle<()>>,
    raw_output_hash: Arc<AtomicU64>,
}

#[derive(Clone)]
pub enum Action {
    OpenUrl(String),
    Refresh,
}

impl Plugin {
    pub fn new(path: PathBuf, interval: Duration, notify: bool) -> Self {
        let id = xxh3_64(path.as_os_str().as_encoded_bytes());
        Self {
            id,
            path: Arc::from(path),
            interval,
            notify,
            tray: TrayIconBuilder::new().with_title("").build().unwrap(),
            cached: None,
            output_hash: None,
            actions: HashMap::new(),
            task: None,
            raw_output_hash: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn start(&mut self, proxy: &EventLoopProxy<AppEvent>) {
        if let Some(h) = self.task.take() {
            h.abort();
        }
        let raw_output_hash = Arc::new(AtomicU64::new(0));
        self.raw_output_hash = raw_output_hash.clone();
        let id = self.id;
        let path = self.path.clone();
        let interval = self.interval;
        let mut next = Instant::now();
        let proxy = proxy.clone();
        self.task = Some(spawn(async move {
            loop {
                let now = Instant::now();
                if next > now {
                    sleep(next - now).await;
                }
                let output = match run_script(&path).await {
                    Ok(result) => result,
                    Err(e) => {
                        eprintln!("{}: {e}", path.display());
                        None
                    }
                };
                let remove = output.as_ref().is_none_or(|r| r.title.is_empty());
                let new_hash = output.as_ref().map_or(0, |r| r.raw_output_hash);
                if remove || raw_output_hash.swap(new_hash, Ordering::Relaxed) != new_hash {
                    proxy.send_event(AppEvent::PluginOutput { id, output }).ok();
                }
                if remove {
                    break;
                }
                next = Instant::now() + interval;
            }
        }));
    }

    pub fn terminate(&mut self) {
        if let Some(h) = self.task.take() {
            h.abort();
        }
    }

    pub fn apply_output(&mut self, output: Option<ScriptResult>) -> bool {
        let Some(result) = output else {
            return true;
        };
        if result.title.is_empty() {
            return true;
        }
        let output_hash = hash_output(&result);
        if self.output_hash == Some(output_hash) {
            return false;
        }
        self.output_hash = Some(output_hash);
        self.actions = set_menu(&mut self.tray, &result);
        if self.notify {
            self.notify_new_items(&result);
        }
        false
    }

    fn notify_new_items(&mut self, result: &ScriptResult) {
        let current = notification_hash(&result.items);
        let changed = self.cached.is_some() && current != self.cached;
        self.cached = current;
        if changed {
            notify(&result.title, "new notification");
        }
    }
}
