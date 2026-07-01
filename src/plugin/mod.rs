mod instance;
mod manager;
mod plugin_menu;
mod plugin_output;

use instance::{Action, Plugin};
pub use manager::PluginManager;
use plugin_menu::set_menu;
use plugin_output::{hash_output, notification_hash};

#[cfg(test)]
mod tests;
