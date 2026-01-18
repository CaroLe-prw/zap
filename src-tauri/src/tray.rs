use tauri::{
    App, Manager,
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
};

use crate::error::ZapError;

pub fn init_tray(app: &mut App) -> Result<(), ZapError> {
    let quit = MenuItem::with_id(app, "quit", "quit", true, None::<&str>)?;
    let show = MenuItem::with_id(app, "show", "show", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show, &quit])?;

    let _tray = TrayIconBuilder::new()
        .show_menu_on_left_click(false)
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .tooltip("Zap")
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => {
                app.exit(0);
            }
            "show" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.unminimize();
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            _ => {
                println!("menu item {:?} not handled", event.id);
            }
        })
        .build(app)?;

    Ok(())
}
