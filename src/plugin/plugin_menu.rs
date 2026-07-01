use std::collections::HashMap;

use crate::plugin::Action;
use crate::utils::{ScriptItem, ScriptItemType, ScriptResult};
use tray_icon::menu::{Menu, MenuId, MenuItem, PredefinedMenuItem, Submenu};

pub fn set_menu(tray: &mut tray_icon::TrayIcon, result: &ScriptResult) -> HashMap<MenuId, Action> {
    tray.set_title(Some(&result.title));
    let menu = Menu::new();
    let mut actions = HashMap::new();
    for item in &result.items {
        MenuTarget::Menu(&menu).append_item(item, &mut actions);
    }
    menu.append(&PredefinedMenuItem::separator()).unwrap();
    let update = MenuItem::new("Update", true, None);
    actions.insert(update.id().clone(), Action::Refresh);
    menu.append(&update).unwrap();
    tray.set_menu(Some(Box::new(menu)));
    actions
}

enum MenuTarget<'a> {
    Menu(&'a Menu),
    Submenu(&'a Submenu),
}

impl MenuTarget<'_> {
    fn append_item(&self, item: &ScriptItem, actions: &mut HashMap<MenuId, Action>) {
        match item.item_type {
            ScriptItemType::Link => {
                if let (Some(title), Some(url)) = (
                    item.title.as_deref().filter(|s| !s.is_empty()),
                    item.path.as_deref().filter(|s| !s.is_empty()),
                ) {
                    let menu_item = MenuItem::new(title, true, None);
                    actions.insert(menu_item.id().clone(), Action::OpenUrl(url.to_string()));
                    self.append(&menu_item);
                }
            }
            ScriptItemType::Divider => self.append(&PredefinedMenuItem::separator()),
            ScriptItemType::Text => {
                if let Some(title) = item.title.as_deref().filter(|s| !s.is_empty()) {
                    self.append(&MenuItem::new(title, false, None));
                }
            }
            ScriptItemType::Submenu => {
                if let (Some(title), Some(children)) = (
                    item.title.as_deref().filter(|s| !s.is_empty()),
                    item.items.as_deref().filter(|i| !i.is_empty()),
                ) {
                    let sub = Submenu::new(title, true);
                    for child in children {
                        MenuTarget::Submenu(&sub).append_item(child, actions);
                    }
                    self.append(&sub);
                }
            }
        }
    }

    fn append<T: tray_icon::menu::IsMenuItem>(&self, item: &T) {
        match self {
            MenuTarget::Menu(menu) => menu.append(item).unwrap(),
            MenuTarget::Submenu(sub) => sub.append(item).unwrap(),
        }
    }
}
