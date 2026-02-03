use tauri::tray::{MouseButton, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Manager};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Shortcut, ShortcutState};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn hide_windows(app: AppHandle) {
    if let Some(main) = app.get_webview_window("main") {
        if let Some(toolbar) = app.get_webview_window("toolbar") {
            let _ = main.hide();
            let _ = toolbar.hide();
        }
    }
}

fn toggle_windows(app: &AppHandle) {
    if let Some(main) = app.get_webview_window("main") {
        if let Some(toolbar) = app.get_webview_window("toolbar") {
            let is_visible = main.is_visible().unwrap_or(false);
            if is_visible {
                let _ = main.hide();
                let _ = toolbar.hide();
            } else {
                let _ = main.show();
                let _ = main.set_focus();
                let _ = toolbar.show();
                let _ = toolbar.set_focus();
            }
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, shortcut, event| {
                    if event.state() == ShortcutState::Pressed {
                        if shortcut.id() == Shortcut::new(None, Code::F9).id() {
                            toggle_windows(app);
                        }
                    }
                })
                .build(),
        )
        .invoke_handler(tauri::generate_handler![greet, hide_windows])
        .setup(|app| {
            // 1. 設置工具列位置到螢幕右側
            if let Some(main_window) = app.get_webview_window("main") {
                let _ = main_window.maximize();

                if let Some(toolbar_window) = app.get_webview_window("toolbar") {
                    // 獲取主螢幕解析度
                    if let Ok(Some(monitor)) = main_window.current_monitor() {
                        let size = monitor.size();
                        let scale_factor = monitor.scale_factor();

                        // 計算右側位置 (視窗寬度約 55)
                        let window_width = (55.0 * scale_factor) as u32;
                        let x = size.width - window_width - (10.0 * scale_factor) as u32; // 距離右邊 10px
                        let y = (size.height / 2) - (275.0 * scale_factor) as u32; // 垂直到中間

                        let _ = toolbar_window.set_position(tauri::Position::Physical(
                            tauri::PhysicalPosition {
                                x: x as i32,
                                y: y as i32,
                            },
                        ));
                    }

                    #[cfg(any(target_os = "windows", target_os = "macos"))]
                    if let Ok(handle) = main_window.as_ref().window_handle() {
                        let _ = toolbar_window.set_parent_window(handle);
                    }
                }
            }

            // 2. 建立 Tray Icon 菜單
            let toggle_item =
                tauri::menu::MenuItem::with_id(app, "toggle", "顯示/隱藏視窗", true, None::<&str>)?;
            let quit_item =
                tauri::menu::MenuItem::with_id(app, "quit", "退出程式", true, None::<&str>)?;
            let menu = tauri::menu::Menu::with_items(app, &[&toggle_item, &quit_item])?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false)
                .tooltip("繪圖工具")
                .on_tray_icon_event(|app, event| {
                    println!("Tray Event: {:?}", event);
                    match event {
                        // 處理單擊或雙擊 (不同 Linux 桌面行為不一)
                        TrayIconEvent::Click {
                            button: MouseButton::Left,
                            ..
                        }
                        | TrayIconEvent::DoubleClick {
                            button: MouseButton::Left,
                            ..
                        } => {
                            println!("Tray: Activaton detected via Click/DoubleClick");
                            toggle_windows(app.app_handle());
                        }
                        _ => {}
                    }
                })
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "toggle" => {
                        toggle_windows(app.app_handle());
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .build(app)?;

            // 3. 註冊 F9 快捷鍵
            let f9_shortcut = Shortcut::new(None, Code::F9);
            app.global_shortcut().register(f9_shortcut)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
