mod plugin;
mod utils;

use tao::event::Event;
use tao::event_loop::{ControlFlow, EventLoopBuilder};
use tray_icon::{
    TrayIconBuilder,
    menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem},
};

#[cfg(target_os = "macos")]
use tao::platform::macos::{ActivationPolicy, EventLoopExtMacOS};

use plugin::PluginManager;
use utils::{ScriptResult, ensure_plugins_dir, plugins_dir};

pub enum AppEvent {
    PluginOutput {
        id: u64,
        output: Option<ScriptResult>,
    },
    MenuClick(tray_icon::menu::MenuId),
}

#[tokio::main]
async fn main() {
    let mut event_loop = EventLoopBuilder::<AppEvent>::with_user_event().build();

    #[cfg(target_os = "macos")]
    event_loop.set_activation_policy(ActivationPolicy::Accessory);

    let proxy = event_loop.create_proxy();

    let update_item = MenuItem::new("Update all", true, None);
    let folder_item = MenuItem::new("Open plugins folder", true, None);
    let quit_item = MenuItem::new("Quit", true, None);

    let main_menu = Menu::new();
    main_menu
        .append_items(&[
            &update_item,
            &folder_item,
            &PredefinedMenuItem::separator(),
            &quit_item,
        ])
        .unwrap();

    let _main_tray = TrayIconBuilder::new()
        .with_title("⛩️")
        .with_menu(Box::new(main_menu))
        .build()
        .unwrap();

    let update_id = update_item.id().clone();
    let folder_id = folder_item.id().clone();
    let quit_id = quit_item.id().clone();

    let proxy_clone = proxy.clone();
    MenuEvent::set_event_handler(Some(move |e: MenuEvent| {
        proxy_clone.send_event(AppEvent::MenuClick(e.id)).ok();
    }));

    ensure_plugins_dir();

    let mut pm = PluginManager::new(proxy);
    pm.start();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        let Event::UserEvent(app_event) = event else {
            return;
        };
        match app_event {
            AppEvent::MenuClick(id) if id == update_id => pm.start(),
            AppEvent::MenuClick(id) if id == folder_id => {
                open::that(plugins_dir()).ok();
            }
            AppEvent::MenuClick(id) if id == quit_id => {
                pm.terminate();
                *control_flow = ControlFlow::Exit;
            }
            AppEvent::MenuClick(id) => pm.handle_click(&id),
            AppEvent::PluginOutput { id, output } => pm.apply_output(id, output),
        }
    });
}
