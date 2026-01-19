use tauri::{
    App, Manager, WebviewUrl, WebviewWindowBuilder,
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
};

use crate::{AppState, error::ZapError};

pub fn init_tray(app: &mut App) -> Result<(), ZapError> {
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let show = MenuItem::with_id(app, "show", "Show Zap", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show, &quit])?;

    let _tray = TrayIconBuilder::new()
        .show_menu_on_left_click(false)
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .tooltip("Zap")
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => {
                let state = app.state::<AppState>();
                *state.is_quitting.lock().unwrap() = true;

                app.exit(0);
            }
            "show" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                } else {
                    let _ = WebviewWindowBuilder::new(
                        app,
                        "main",
                        WebviewUrl::App("index.html".into()),
                    )
                    .title("Zap")
                    .inner_size(630.0, 800.0)
                    .resizable(false)
                    .maximizable(false)
                    .build();
                }
            }
            _ => {}
        })
        .build(app)?;

    Ok(())
}
