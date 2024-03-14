// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    async_runtime, AppHandle, EventTarget, Manager, Runtime, Url, WebviewUrl, WebviewWindowBuilder,
};

#[tauri::command]
fn hello<R: Runtime>(app: AppHandle<R>) {
    println!("Hello, world");

    if let Some(window) = app.get_webview_window("main") {
        _ = window.emit_to(
            EventTarget::WebviewWindow {
                label: "main".to_string(),
            },
            "hello-response",
            (),
        );
    }
}

const SCRIPT: &str = "\
window.addEventListener('load', () => {
  window.__TAURI__.event.listen('hello-response', (event) => {
    console.log('hello-response');
  });
});
";

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let app = app.app_handle().clone();
            async_runtime::spawn(async move {
                if let Ok(url) = Url::parse("https://jp.apps.gree.net/ja/97") {
                    let window = WebviewWindowBuilder::new(&app, "main", WebviewUrl::External(url))
                        .initialization_script(SCRIPT)
                        .build()
                        .unwrap();
                    window.open_devtools();
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![hello])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
