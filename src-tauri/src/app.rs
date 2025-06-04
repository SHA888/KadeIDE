//! Main application module for KadeIDE

use tauri::{
    Manager, Runtime, WebviewWindowBuilder, WebviewUrl
};
use tauri::tray::TrayIconBuilder;
use tauri::menu::{Menu, MenuItem};
use log::info;
use anyhow::{Result, Context};

/// Initialize the system tray with menu
fn setup_tray<R: Runtime>(app: &mut tauri::App<R>) -> Result<()> {
    let app_handle = app.handle();
    
    // Create menu items
    let show = MenuItem::with_id(app_handle, "show", "Show", true, None::<&str>)?;
    let hide = MenuItem::with_id(app_handle, "hide", "Hide", true, None::<&str>)?;
    let quit = MenuItem::with_id(app_handle, "quit", "Quit", true, None::<&str>)?;
    
    // Create the menu with items
    let menu = Menu::with_items(app_handle, &[&show, &hide, &quit])?;
    
    // Build and set up the tray icon
    TrayIconBuilder::new()
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event(move |app_handle, event| {
            match event.id.as_ref() {
                "show" => {
                    if let Some(window) = app_handle.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
                "hide" => {
                    if let Some(window) = app_handle.get_webview_window("main") {
                        let _ = window.hide();
                    }
                }
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            }
        })
        .build(app_handle)?;
    
    Ok(())
}

/// Initialize application directories and configuration
fn init_app() -> Result<()> {
    use crate::utils::{get_config_dir, get_data_dir, get_cache_dir, get_local_data_dir, ensure_dir_exists};
    
    // Get application directories
    let app_data_dir = get_data_dir()?;
    let config_dir = get_config_dir()?;
    let cache_dir = get_cache_dir()?;
    let logs_dir = get_local_data_dir()?.join("logs");
    
    // Create directories if they don't exist
    ensure_dir_exists(&app_data_dir)
        .context("Failed to create app data directory")?;
    
    ensure_dir_exists(&config_dir)
        .context("Failed to create config directory")?;
    
    ensure_dir_exists(&cache_dir)
        .context("Failed to create cache directory")?;
    
    ensure_dir_exists(&logs_dir)
        .context("Failed to create logs directory")?;
    
    info!("App data directory: {:?}", app_data_dir);
    info!("Config directory: {:?}", config_dir);
    info!("Cache directory: {:?}", cache_dir);
    info!("Logs directory: {:?}", logs_dir);
    
    Ok(())
}

/// Main entry point for the Tauri application
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<()> {
    info!("Starting Kade IDE");
    
    // Initialize the app directories and configuration
    init_app()?;
    
    // Create and run the Tauri application
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .level(log::LevelFilter::Debug)
                .build()
        )
        .plugin(tauri_plugin_http::init())
        .setup(|app| {
            // Set up the system tray
            setup_tray(app)?;
            
            // Create the main window
            let window = WebviewWindowBuilder::new(
                app,
                "main",
                WebviewUrl::App("index.html".into())
            )
            .title("Kade IDE")
            .inner_size(1280.0, 800.0)
            .min_inner_size(800.0, 600.0)
            .build()?;
            
            #[cfg(debug_assertions)]
            window.open_devtools();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    
    Ok(())
}
