use serde_json::Value as JsonValue;
#[cfg(target_os = "windows")]
use std::sync::atomic::{AtomicBool, Ordering};
#[cfg(target_os = "windows")]
use std::thread;
#[cfg(target_os = "windows")]
use std::time::Duration;
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Shortcut, ShortcutState};

// 追蹤視窗是否已經首次顯示過
#[cfg(target_os = "windows")]
static WINDOWS_SHOWN_ONCE: AtomicBool = AtomicBool::new(false);

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
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
                // Windows 上使用延遲顯示來消除閃動
                #[cfg(target_os = "windows")]
                {
                    // 記錄工具列當前位置（如果已經顯示過）
                    let toolbar_pos = toolbar.outer_position().ok();

                    // 將視窗移到螢幕外
                    let _ = main.set_position(tauri::Position::Physical(tauri::PhysicalPosition {
                        x: -10000,
                        y: -10000,
                    }));
                    let _ =
                        toolbar.set_position(tauri::Position::Physical(tauri::PhysicalPosition {
                            x: -10000,
                            y: -10000,
                        }));

                    // 顯示視窗（在螢幕外，用戶看不到）
                    let _ = main.show();
                    let _ = toolbar.show();

                    // 等待 WebView 渲染完成，然後移回正確位置
                    let main_clone = main.clone();
                    let toolbar_clone = toolbar.clone();
                    let is_first_show = !WINDOWS_SHOWN_ONCE.swap(true, Ordering::SeqCst);

                    thread::spawn(move || {
                        // 等待 WebView 初始化透明背景
                        // 首次顯示需要更長時間，後續顯示較快
                        let delay = if is_first_show { 150 } else { 50 };
                        thread::sleep(Duration::from_millis(delay));

                        // 獲取螢幕尺寸並移回正確位置
                        if let Ok(Some(monitor)) = main_clone.current_monitor() {
                            let size = monitor.size();
                            let scale_factor = monitor.scale_factor();

                            // 主視窗最大化
                            let _ = main_clone.set_position(tauri::Position::Physical(
                                tauri::PhysicalPosition { x: 0, y: 0 },
                            ));
                            let _ = main_clone.maximize();

                            // 工具列視窗：如果之前有位置就恢復，否則移到螢幕右側
                            if let Some(pos) = toolbar_pos {
                                let _ = toolbar_clone.set_position(tauri::Position::Physical(pos));
                            } else {
                                let window_width = toolbar_clone
                                    .outer_size()
                                    .ok()
                                    .map(|s| s.width)
                                    .unwrap_or((55.0 * scale_factor) as u32);
                                let x = size.width - window_width - (10.0 * scale_factor) as u32;
                                let y = (size.height / 2) - (275.0 * scale_factor) as u32;

                                let _ = toolbar_clone.set_position(tauri::Position::Physical(
                                    tauri::PhysicalPosition {
                                        x: x as i32,
                                        y: y as i32,
                                    },
                                ));
                            }
                        }

                        let _ = main_clone.set_focus();
                        let _ = toolbar_clone.set_focus();
                    });
                    return;
                }

                // 非 Windows 平台：直接顯示
                #[cfg(not(target_os = "windows"))]
                {
                    let _ = main.show();
                    let _ = main.set_focus();
                    let _ = toolbar.show();
                    let _ = toolbar.set_focus();
                }
            }
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
struct AppSettings {
    pen1_color: String,
    trace_color: String,
    rect_color: String,
    line_width: u32,
}

#[tauri::command]
fn save_settings(app: AppHandle, settings: AppSettings) -> Result<(), String> {
    let store = tauri_plugin_store::StoreBuilder::new(&app, "settings.json")
        .build()
        .map_err(|e| format!("Failed to create store: {:?}", e))?;

    store.set("pen1_color", settings.pen1_color);
    store.set("trace_color", settings.trace_color);
    store.set("rect_color", settings.rect_color);
    store.set("line_width", settings.line_width);

    store
        .save()
        .map_err(|e| format!("Failed to save settings: {:?}", e))?;

    Ok(())
}

#[tauri::command]
fn load_settings(app: AppHandle) -> Result<AppSettings, String> {
    let store = tauri_plugin_store::StoreBuilder::new(&app, "settings.json")
        .build()
        .map_err(|e| format!("Failed to create store: {:?}", e))?;

    let pen1_color = match store.get("pen1_color") {
        Some(JsonValue::String(s)) => s.clone(),
        _ => "#ff0000".to_string(),
    };

    let trace_color = match store.get("trace_color") {
        Some(JsonValue::String(s)) => s.clone(),
        _ => "#ff8800".to_string(),
    };

    let rect_color = match store.get("rect_color") {
        Some(JsonValue::String(s)) => s.clone(),
        _ => "#0000ff".to_string(),
    };

    let line_width = match store.get("line_width") {
        Some(JsonValue::Number(n)) => n.as_u64().unwrap_or(5) as u32,
        _ => 5,
    };

    Ok(AppSettings {
        pen1_color,
        trace_color,
        rect_color,
        line_width,
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::new().build())
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
        .invoke_handler(tauri::generate_handler![
            greet,
            hide_windows,
            set_click_through,
            save_settings,
            load_settings
        ])
        .setup(|app| {
            // 1. 設置工具列位置到螢幕右側
            if let Some(main_window) = app.get_webview_window("main") {
                let _ = main_window.maximize();

                if let Some(toolbar_window) = app.get_webview_window("toolbar") {
                    // 獲取主螢幕解析度
                    if let Ok(Some(monitor)) = main_window.current_monitor() {
                        let size = monitor.size();
                        let scale_factor = monitor.scale_factor();

                        // 獲取實際視窗寬度 (Windows 可能有最小寬度限制)
                        let window_width = toolbar_window
                            .outer_size()
                            .ok()
                            .map(|s| s.width)
                            .unwrap_or((55.0 * scale_factor) as u32);
                        let x = size.width - window_width - (10.0 * scale_factor) as u32; // 距離右邊 10px
                        let y = (size.height / 2) - (275.0 * scale_factor) as u32; // 垂直到中間

                        let _ = toolbar_window.set_position(tauri::Position::Physical(
                            tauri::PhysicalPosition {
                                x: x as i32,
                                y: y as i32,
                            },
                        ));
                    }

                    // #[cfg(any(target_os = "windows", target_os = "macos"))]
                    // if let Ok(handle) = main_window.as_ref().window_handle() {
                    //     let _ = toolbar_window.set_parent_window(handle);
                    // }
                }
            }

            // 2. 建立 Tray Icon 菜單
            let toggle_item =
                tauri::menu::MenuItem::with_id(app, "toggle", "顯示/隱藏視窗", true, None::<&str>)?;
            let settings_item =
                tauri::menu::MenuItem::with_id(app, "settings", "設定", true, None::<&str>)?;
            let quit_item =
                tauri::menu::MenuItem::with_id(app, "quit", "退出程式", true, None::<&str>)?;
            let menu =
                tauri::menu::Menu::with_items(app, &[&toggle_item, &settings_item, &quit_item])?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false)
                .tooltip("繪圖工具")
                .on_tray_icon_event(|app, event| {
                    println!("Tray Event: {:?}", event);
                    match event {
                        // 只處理左鍵單擊
                        TrayIconEvent::Click {
                            button: MouseButton::Left,
                            button_state,
                            ..
                        } => {
                            // 在 Windows 上，Click 事件會觸發多次 (Down 和 Up)
                            // 我們只在 Up (放開) 時執行切換，以避免閃爍 (連續切換兩次)
                            // #[cfg(target_os = "windows")]
                            if button_state == MouseButtonState::Down {
                                return;
                            }

                            println!("Tray: Activation detected via Click");
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
                        // 先顯示 main 窗口，確保設定對話框可見
                        if let Some(main) = app.app_handle().get_webview_window("main") {
                            let _ = main.show();
                            let _ = main.set_focus();
                        }
                        if let Some(toolbar) = app.app_handle().get_webview_window("toolbar") {
                            let _ = toolbar.show();
                        }

                        // 發送事件到前端開啟設定對話框
                        if let Err(e) = app.app_handle().emit("open-settings", ()) {
                            println!("Failed to emit open-settings event: {:?}", e);
                        }
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
