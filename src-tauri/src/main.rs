#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{Manager, SystemTrayEvent, GlobalShortcutManager, Window};
use window_vibrancy::{apply_blur, apply_vibrancy, NSVisualEffectMaterial};
use window_shadows::set_shadow;
use tauri::SystemTray;

fn toggle_visibility(win: Window) {
    if win.is_visible().unwrap() {
        win.hide().expect("failed to hide window");
    }else {
        win.center().unwrap();
        win.show().expect("failed to show window");
        win.set_focus().expect("failed to set-focus to window");
    }
}

fn main() {
    let tray = SystemTray::new();

    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();

            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            #[cfg(target_os = "macos")]
            apply_vibrancy(&window, NSVisualEffectMaterial::UltraDark)
                .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

            #[cfg(target_os = "windows")]
            apply_blur(&window, Some((18, 18, 18, 125)))
                .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

            set_shadow(&window, true).expect("Unsupported platform!");

            window.hide().expect("failed to hide");
            Ok(())
        })
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
              position: _,
              size: _,
              ..
            } => {
                let window = app.get_window("main").unwrap();
                toggle_visibility(window);
            }
            _ => {}
        })
        .run(tauri::generate_context!()).expect("error while running tauri application");
}
