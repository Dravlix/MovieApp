mod db;
mod mpv;

use std::sync::Mutex;
use tauri::{State, Manager};
use db::{MediaItem, Category};

struct AppState {
    db: Mutex<rusqlite::Connection>,
    mpv: mpv::MpvManager,
}

#[tauri::command]
fn get_movies(state: State<AppState>) -> Result<Vec<MediaItem>, String> {
    let conn = state.db.lock().unwrap();
    db::get_media(&conn, Some("movie")).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_series(state: State<AppState>) -> Result<Vec<MediaItem>, String> {
    let conn = state.db.lock().unwrap();
    db::get_media(&conn, Some("series")).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_wishlist(state: State<AppState>) -> Result<Vec<MediaItem>, String> {
    let conn = state.db.lock().unwrap();
    db::get_wishlist(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
fn toggle_wishlist(state: State<AppState>, id: String) -> Result<bool, String> {
    let conn = state.db.lock().unwrap();
    db::toggle_wishlist(&conn, &id).map_err(|e| e.to_string())
}

#[tauri::command]
fn check_wishlist(state: State<AppState>, id: String) -> Result<bool, String> {
    let conn = state.db.lock().unwrap();
    db::is_in_wishlist(&conn, &id).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_collection(state: State<AppState>, id: String) -> Result<Vec<MediaItem>, String> {
    let conn = state.db.lock().unwrap();
    db::get_collection(&conn, &id).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_home_data(state: State<AppState>) -> Result<Vec<Category>, String> {
    let conn = state.db.lock().unwrap();
    let movies = db::get_media(&conn, Some("movie")).unwrap_or_default();
    let series = db::get_media(&conn, Some("series")).unwrap_or_default();
    
    let mut categories = Vec::new();
    
    if movies.len() > 6 {
        categories.push(Category {
            title: "Nedávno přidáno".to_string(),
            items: movies[0..6].to_vec(),
        });
        categories.push(Category {
            title: "Akční nářez".to_string(),
            items: movies[6..].to_vec(),
        });
    } else {
        categories.push(Category {
            title: "Všechny filmy".to_string(),
            items: movies,
        });
    }
    
    categories.push(Category {
        title: "Populární seriály".to_string(),
        items: series,
    });
    
    Ok(categories)
}

#[tauri::command]
fn get_all_collections(state: State<AppState>) -> Result<Vec<Category>, String> {
    let conn = state.db.lock().unwrap();
    
    let spiderman = db::get_collection(&conn, "c_spider").unwrap_or_default();
    let matrix = db::get_collection(&conn, "c_matrix").unwrap_or_default();
    let batman = db::get_collection(&conn, "c_batman").unwrap_or_default();
    
    let mut cols = Vec::new();
    if !spiderman.is_empty() { cols.push(Category { title: "Kolekce: Spider-Man (Popořadě)".to_string(), items: spiderman }); }
    if !matrix.is_empty() { cols.push(Category { title: "Kolekce: The Matrix".to_string(), items: matrix }); }
    if !batman.is_empty() { cols.push(Category { title: "Kolekce: Temný rytíř (Batman)".to_string(), items: batman }); }
    
    Ok(cols)
}

#[tauri::command]
fn get_scan_folders(state: State<AppState>) -> Result<Vec<String>, String> {
    let conn = state.db.lock().unwrap();
    db::get_scan_folders(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
fn add_scan_folder(state: State<AppState>, path: String) -> Result<(), String> {
    let conn = state.db.lock().unwrap();
    db::add_scan_folder(&conn, &path).map_err(|e| e.to_string())
}

#[tauri::command]
fn remove_scan_folder(state: State<AppState>, path: String) -> Result<(), String> {
    let conn = state.db.lock().unwrap();
    db::remove_scan_folder(&conn, &path).map_err(|e| e.to_string())
}

#[tauri::command]
fn scan_library(state: State<AppState>) -> Result<db::ScanSummary, String> {
    let conn = state.db.lock().unwrap();
    db::scan_all_folders(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
fn open_media_file(path: String) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(["/C", "start", "", &path])
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn check_mpv_status(state: State<AppState>) -> Result<mpv::MpvStatus, String> {
    Ok(mpv::MpvStatus {
        is_installed: mpv::MpvManager::is_installed(),
        is_running: state.mpv.is_running(),
        path: mpv::MpvManager::find_mpv_binary(),
    })
}

#[tauri::command]
fn mpv_play(state: State<AppState>, options: mpv::MpvLaunchOptions) -> Result<(), String> {
    state.mpv.play(options)
}

#[tauri::command]
fn mpv_command(state: State<AppState>, json_cmd: String) -> Result<(), String> {
    state.mpv.send_ipc_command(&json_cmd)
}

#[tauri::command]
fn mpv_stop(state: State<AppState>) -> Result<(), String> {
    state.mpv.kill_current();
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_dir = app.path().app_data_dir().expect("failed to get app data dir");
            let conn = db::init_db(app_dir).expect("failed to init db");
            app.manage(AppState { 
                db: Mutex::new(conn),
                mpv: mpv::MpvManager::new(),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_movies, 
            get_series, 
            get_wishlist, 
            toggle_wishlist, 
            check_wishlist,
            get_collection, 
            get_home_data,
            get_all_collections,
            get_scan_folders,
            add_scan_folder,
            remove_scan_folder,
            scan_library,
            open_media_file,
            check_mpv_status,
            mpv_play,
            mpv_command,
            mpv_stop
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
