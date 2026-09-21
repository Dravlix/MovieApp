mod db;

use std::sync::Mutex;
use tauri::{State, Manager};
use db::{MediaItem, Category};

struct AppState {
    db: Mutex<rusqlite::Connection>,
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_dir = app.path().app_data_dir().expect("failed to get app data dir");
            let conn = db::init_db(app_dir).expect("failed to init db");
            app.manage(AppState { db: Mutex::new(conn) });
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
            get_all_collections
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
