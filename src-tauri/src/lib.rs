mod commands;
mod monitoring;
mod state;

use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .manage(AppState::new())
    .invoke_handler(tauri::generate_handler![commands::get_info])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
