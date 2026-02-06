use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::Mutex;
use tauri::tray::{MouseButton, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Emitter, Manager, State};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppSettings {
    pub pen1_color: String,
    pub trace_color: String,
    pub rect_color: String,
    pub line_width: u32,
    pub hotkey: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            pen1_color: "#000000".to_string(),
            trace_color: "#ff0000".to_string(),
            rect_color: "#0000ff".to_string(),
            line_width: 3,
            hotkey: "F9".to_string(),
        }
    }
}

struct AppState {
    pub settings: Mutex<AppSettings>,
}

fn get_settings_path(app: &AppHandle) -> std::path::PathBuf {
    let mut path = app.path().app_config_dir().unwrap_or_default();
    if !path.exists() {
        let _ = fs::create_dir_all(&path);
    }
    path.push("settings.json");
    path
}

fn load_settings_internal(app: &AppHandle) -> AppSettings {
    let path = get_settings_path(app);
    if path.exists() {
        if let Ok(content) = fs::read_to_string(path) {
            if let Ok(settings) = serde_json::from_str(&content) {
                return settings;
            }
        }
    }
    AppSettings::default()
}

fn save_settings_internal(app: &AppHandle, settings: &AppSettings) {
    let path = get_settings_path(app);
    if let Ok(content) = serde_json::to_string_pretty(settings) {
        let _ = fs::write(path, content);
    }
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_settings(state: State<'_, AppState>) -> AppSettings {
    state.settings.lock().unwrap().clone()
}

#[tauri::command]
fn update_settings(app: AppHandle, state: State<'_, AppState>, new_settings: AppSettings) -> Result<(), String> {
    let old_hotkey = {
        let mut settings = state.settings.lock().unwrap();
        let old = settings.hotkey.clone();
        *settings = new_settings.clone();
        old
    };

    save_settings_internal(&app, &new_settings);

    // Update shortcut if changed
    if old_hotkey != new_settings.hotkey {
        if let Ok(old_shortcut) = old_hotkey.parse::<Shortcut>() {
            let _ = app.global_shortcut().unregister(old_shortcut);
        }
        if let Ok(new_shortcut) = new_settings.hotkey.parse::<Shortcut>() {
            let _ = app.global_shortcut().register(new_shortcut);
        }
    }

    // Notify all windows
    let _ = app.emit("settings-updated", new_settings);

    Ok(())
}

#[tauri::command]
fn set_click_through(app: AppHandle, ignore: bool) {
    if let Some(main) = app.get_webview_window("main") {
        println!("Rust: Setting click-through to {} for main window", ignore);
        let _ = main.set_ignore_cursor_events(ignore);
        if !ignore {
            let _ = main.set_focus();
        }
    }
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

fn open_settings_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("settings") {
        let _ = window.show();
        let _ = window.set_focus();
    } else {
        let _ = tauri::WebviewWindowBuilder::new(app, "settings", tauri::WebviewUrl::App("/#settings".into()))
            .title("設定")
            .inner_size(450.0, 650.0)
            .resizable(false)
            .build();
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
                        let state = app.state::<AppState>();
                        let hotkey_str = state.settings.lock().unwrap().hotkey.clone();
                        if let Ok(registered_shortcut) = hotkey_str.parse::<Shortcut>() {
                            if shortcut.id() == registered_shortcut.id() {
                                toggle_windows(app);
                            }
                        }
                    }
                })
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            greet,
            hide_windows,
            set_click_through,
            get_settings,
            update_settings
        ])
        .setup(|app| {
            let app_handle = app.app_handle().clone();
            let settings = load_settings_internal(&app_handle);
            app.manage(AppState {
                settings: Mutex::new(settings.clone()),
            });

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
            let settings_item =
                tauri::menu::MenuItem::with_id(app, "settings", "設定", true, None::<&str>)?;
            let quit_item =
                tauri::menu::MenuItem::with_id(app, "quit", "退出程式", true, None::<&str>)?;
            let menu = tauri::menu::Menu::with_items(app, &[&toggle_item, &settings_item, &quit_item])?;

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
                    "settings" => {
                        open_settings_window(app.app_handle());
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .build(app)?;

            // 3. 註冊快捷鍵
            if let Ok(shortcut) = settings.hotkey.parse::<Shortcut>() {
                let _ = app.global_shortcut().register(shortcut);
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
