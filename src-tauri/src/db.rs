use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MediaItem {
    pub id: String,
    pub title: String,
    pub description: String,
    pub year: i32,
    pub genre: String,
    pub poster_url: String,
    pub backdrop_url: String,
    pub r#type: Option<String>,
    pub duration: Option<String>,
    pub age_rating: Option<String>,
    pub content_warnings: Option<String>,
    pub cast: Option<Vec<String>>,
    pub genres: Option<Vec<String>>,
    pub moods: Option<Vec<String>>,
    pub local_file_path: Option<String>,
    pub rating: Option<f64>,
    pub audio_languages: Option<Vec<String>>,
    pub subtitle_languages: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ScanSummary {
    pub scanned_folders: usize,
    pub files_found: usize,
    pub new_added: usize,
    pub new_titles: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Category {
    pub title: String,
    pub items: Vec<MediaItem>,
}

pub fn init_db(app_dir: PathBuf) -> Result<Connection> {
    fs::create_dir_all(&app_dir).unwrap_or_default();
    let db_path = app_dir.join("movies.db");
    
    let conn = Connection::open(&db_path)?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS media (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT,
            year INTEGER,
            genre TEXT,
            posterUrl TEXT,
            backdropUrl TEXT,
            type TEXT,
            duration TEXT,
            ageRating TEXT,
            contentWarnings TEXT,
            cast_json TEXT,
            genres_json TEXT,
            moods_json TEXT,
            local_file_path TEXT,
            audio_languages_json TEXT,
            subtitle_languages_json TEXT,
            rating REAL DEFAULT 8.2
        )",
        [],
    )?;

    // Migrace pro případnou existující databázi
    let _ = conn.execute("ALTER TABLE media ADD COLUMN audio_languages_json TEXT", []);
    let _ = conn.execute("ALTER TABLE media ADD COLUMN subtitle_languages_json TEXT", []);
    let _ = conn.execute("ALTER TABLE media ADD COLUMN rating REAL DEFAULT 8.2", []);
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS collections (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS media_collections (
            media_id TEXT,
            collection_id TEXT,
            PRIMARY KEY(media_id, collection_id)
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS wishlist (
            media_id TEXT PRIMARY KEY,
            added_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS scan_folders (
            path TEXT PRIMARY KEY,
            added_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;
    
    seed_db_if_empty(&conn)?;
    
    Ok(conn)
}

fn create_mock(
    id: &str,
    title: &str,
    desc: &str,
    year: i32,
    genre: &str,
    poster: &str,
    backdrop: &str,
    m_type: &str,
    duration: Option<&str>,
    rating: f64,
    audio: Vec<&str>,
    subs: Vec<&str>,
) -> MediaItem {
    MediaItem {
        id: id.to_string(),
        title: title.to_string(),
        description: desc.to_string(),
        year,
        genre: genre.to_string(),
        poster_url: poster.to_string(),
        backdrop_url: backdrop.to_string(),
        r#type: Some(m_type.to_string()),
        duration: duration.map(|s| s.to_string()),
        age_rating: Some("12+".to_string()),
        content_warnings: None,
        cast: None,
        genres: None,
        moods: None,
        local_file_path: None,
        rating: Some(rating),
        audio_languages: Some(audio.into_iter().map(String::from).collect()),
        subtitle_languages: Some(subs.into_iter().map(String::from).collect()),
    }
}

fn seed_db_if_empty(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM media")?;
    let count: i64 = stmt.query_row([], |row| row.get(0))?;
    
    if count == 0 {
        let mut m1 = create_mock("m1", "THE SHIMMER", "Skupina vědců vstupuje do anomální zóny zvané Lesk, kde neplatí zákony přírody, biologie ani fyziky. Co najdou uvnitř, navždy změní lidstvo.", 2026, "Sci-Fi / Thriller", "https://images.unsplash.com/photo-1618331835717-801e976710b2?q=80&w=800&auto=format&fit=crop", "https://images.unsplash.com/photo-1618331835717-801e976710b2?q=80&w=1920&auto=format&fit=crop", "movie", Some("1h 55m"), 8.6, vec!["CZ", "EN"], vec!["CZ", "EN", "SK"]);
        m1.age_rating = Some("18+".to_string());
        m1.content_warnings = Some("násilí, hrubý jazyk".to_string());
        m1.cast = Some(vec!["Natalie Portman".to_string(), "Oscar Isaac".to_string(), "Jennifer Jason Leigh".to_string()]);
        m1.genres = Some(vec!["Sci-Fi".to_string(), "Thriller".to_string(), "Mysteriózní".to_string()]);
        m1.moods = Some(vec!["Temné".to_string(), "Napínavé".to_string(), "Mind-bending".to_string()]);

        let mut s1 = create_mock("s1", "CYBER CITY", "V neónem ozářeném městě budoucnosti se detektiv snaží rozplést síť korporátních konspirací.", 2025, "Sci-Fi / Akční", "https://images.unsplash.com/photo-1518773553398-650c184e0bb3?q=80&w=800&auto=format&fit=crop", "https://images.unsplash.com/photo-1518773553398-650c184e0bb3?q=80&w=1920&auto=format&fit=crop", "series", Some("3 Série"), 8.9, vec!["CZ", "EN"], vec!["CZ", "EN"]);
        s1.age_rating = Some("15+".to_string());
        s1.content_warnings = Some("násilí, drogy".to_string());
        s1.cast = Some(vec!["Keanu Reeves".to_string(), "Carrie-Anne Moss".to_string()]);
        s1.genres = Some(vec!["Sci-Fi".to_string(), "Akční".to_string(), "Kyberpunk".to_string()]);
        s1.moods = Some(vec!["Akční".to_string(), "Drsné".to_string()]);

        let mock_items = vec![
            m1,
            s1,
            create_mock("m2", "Blade Runner 2049", "Mladý Blade Runner odhaluje dlouho pohřbené tajemství.", 2017, "Sci-Fi", "https://images.unsplash.com/photo-1534447677768-be436bb09401?q=80&w=500&auto=format&fit=crop", "", "movie", Some("2h 44m"), 8.0, vec!["CZ", "EN"], vec!["CZ", "EN"]),
            create_mock("m3", "Interstellar", "Tým výzkumníků cestuje červí dírou v prostoru ve snaze zajistit přežití lidstva.", 2014, "Sci-Fi", "https://images.unsplash.com/photo-1444703686981-a3abbc4d4fe3?q=80&w=500&auto=format&fit=crop", "", "movie", Some("2h 49m"), 8.7, vec!["CZ", "EN"], vec!["CZ", "EN", "SK"]),
            create_mock("m4", "Drive", "Tajemný hollywoodský kaskadér a noční řidič pro zločince se zaplete do nebezpečné loupeže.", 2011, "Akční", "https://images.unsplash.com/photo-1492144534655-ae79c964c9d7?q=80&w=500&auto=format&fit=crop", "", "movie", Some("1h 40m"), 7.8, vec!["EN"], vec!["CZ", "EN"]),
            create_mock("m5", "Joker", "Zkrachovalý komik Arthur Fleck se propadá do šílenství a mění se v ikonu kriminálního podsvětí.", 2019, "Drama", "https://images.unsplash.com/photo-1620336655055-088d06e36bf0?q=80&w=500&auto=format&fit=crop", "", "movie", Some("2h 02m"), 8.4, vec!["CZ", "EN"], vec!["CZ"]),
            create_mock("m6", "Mad Max: Fury Road", "V postapokalyptické pustině se Max spojuje s Furiosou na útěku před tyranem.", 2015, "Akční", "https://images.unsplash.com/photo-1542362567-b07e54358753?q=80&w=500&auto=format&fit=crop", "", "movie", Some("2h 00m"), 8.1, vec!["CZ", "EN"], vec!["CZ", "EN"]),
            create_mock("m7", "Dune", "Paul Atreides musí čelit svému osudu na nejnebezpečnější planetě vesmíru.", 2021, "Sci-Fi", "https://images.unsplash.com/photo-1547333590-478e7279b940?q=80&w=500&auto=format&fit=crop", "", "movie", Some("2h 35m"), 8.0, vec!["CZ", "EN"], vec!["CZ", "EN"]),
            create_mock("m8", "John Wick", "Legendární nájemný zabiják se vrací z důchodu, aby pomstil smrt svého psa.", 2014, "Akční", "https://images.unsplash.com/photo-1566373767864-16a3a7891825?q=80&w=500&auto=format&fit=crop", "", "movie", Some("1h 41m"), 7.4, vec!["CZ", "EN"], vec!["CZ", "EN"]),
            create_mock("m9", "The Matrix", "Počítačový hacker Neo zjišťuje šokující pravdu o své realitě.", 1999, "Sci-Fi", "https://images.unsplash.com/photo-1526304640581-d334cdbbf45e?q=80&w=500&auto=format&fit=crop", "", "movie", Some("2h 16m"), 8.7, vec!["CZ", "EN"], vec!["CZ", "EN"]),
            create_mock("m10", "Inception", "Zloděj kradoucí firemní tajemství skrze technologii sdílení snů dostává obrácený úkol.", 2010, "Sci-Fi", "https://images.unsplash.com/photo-1618042164219-62c820f10723?q=80&w=500&auto=format&fit=crop", "", "movie", Some("2h 28m"), 8.8, vec!["CZ", "EN"], vec!["CZ", "EN"]),
            create_mock("m11", "Terminator 2", "Kyborg T-800 je poslán zpět v čase, aby ochránil mladého Johna Connora.", 1991, "Akční", "https://images.unsplash.com/photo-1535295972055-1c762f4483e5?q=80&w=500&auto=format&fit=crop", "", "movie", Some("2h 17m"), 8.6, vec!["CZ", "EN"], vec!["CZ"]),
            
            // Spider-Man
            create_mock("sp1", "Spider-Man", "Peter Parker získá pavoučí schopnosti.", 2002, "Akční", "https://images.unsplash.com/photo-1635805737707-575885ab0820?q=80&w=500&auto=format&fit=crop", "", "movie", Some("2h 01m"), 7.4, vec!["CZ", "EN"], vec!["CZ", "EN"]),
            create_mock("sp2", "Spider-Man 2", "Peter Parker čelí Dr. Octopusovi.", 2004, "Akční", "https://images.unsplash.com/photo-1635805737707-575885ab0820?q=80&w=500&auto=format&fit=crop", "", "movie", Some("2h 07m"), 7.5, vec!["CZ", "EN"], vec!["CZ", "EN"]),
            create_mock("sp3", "Spider-Man 3", "Peter bojuje s temnotou v sobě a s Venomem.", 2007, "Akční", "https://images.unsplash.com/photo-1635805737707-575885ab0820?q=80&w=500&auto=format&fit=crop", "", "movie", Some("2h 19m"), 6.3, vec!["CZ", "EN"], vec!["CZ"]),
            create_mock("sp4", "The Amazing Spider-Man", "Reboot s Andrewem Garfieldem.", 2012, "Akční", "https://images.unsplash.com/photo-1635805737707-575885ab0820?q=80&w=500&auto=format&fit=crop", "", "movie", Some("2h 16m"), 6.9, vec!["CZ", "EN"], vec!["CZ", "EN"]),
            create_mock("sp5", "Spider-Man: Homecoming", "Peter Parker v Marvel Cinematic Universe.", 2017, "Akční", "https://images.unsplash.com/photo-1635805737707-575885ab0820?q=80&w=500&auto=format&fit=crop", "", "movie", Some("2h 13m"), 7.4, vec!["CZ", "EN"], vec!["CZ", "EN"]),
            create_mock("sp6", "Spider-Man: Far From Home", "Peter na školním výletě v Evropě.", 2019, "Akční", "https://images.unsplash.com/photo-1635805737707-575885ab0820?q=80&w=500&auto=format&fit=crop", "", "movie", Some("2h 09m"), 7.4, vec!["CZ", "EN"], vec!["CZ", "EN"]),
            create_mock("sp7", "Spider-Man: No Way Home", "Multivesmír se otevírá.", 2021, "Akční", "https://images.unsplash.com/photo-1635805737707-575885ab0820?q=80&w=500&auto=format&fit=crop", "", "movie", Some("2h 28m"), 8.2, vec!["CZ", "EN"], vec!["CZ", "EN"]),
            
            // Matrix
            create_mock("mx2", "The Matrix Reloaded", "Neo se učí ovládat své schopnosti.", 2003, "Sci-Fi", "https://images.unsplash.com/photo-1526304640581-d334cdbbf45e?q=80&w=500&auto=format&fit=crop", "", "movie", Some("2h 18m"), 7.2, vec!["CZ", "EN"], vec!["CZ"]),
            create_mock("mx3", "The Matrix Revolutions", "Závěrečná bitva o Zion.", 2003, "Sci-Fi", "https://images.unsplash.com/photo-1526304640581-d334cdbbf45e?q=80&w=500&auto=format&fit=crop", "", "movie", Some("2h 09m"), 6.7, vec!["CZ", "EN"], vec!["CZ"]),
            create_mock("mx4", "The Matrix Resurrections", "Návrat ke kořenům v nové formě.", 2021, "Sci-Fi", "https://images.unsplash.com/photo-1526304640581-d334cdbbf45e?q=80&w=500&auto=format&fit=crop", "", "movie", Some("2h 28m"), 5.7, vec!["CZ", "EN"], vec!["CZ"]),
            
            // Batman
            create_mock("bm1", "Batman Begins", "Zrození temného rytíře.", 2005, "Akční", "https://images.unsplash.com/photo-1509347528160-9a9e33742cdb?q=80&w=500&auto=format&fit=crop", "", "movie", Some("2h 20m"), 8.2, vec!["CZ", "EN"], vec!["CZ", "EN"]),
            create_mock("bm2", "The Dark Knight", "Joker rozpoutává chaos.", 2008, "Akční", "https://images.unsplash.com/photo-1509347528160-9a9e33742cdb?q=80&w=500&auto=format&fit=crop", "", "movie", Some("2h 32m"), 9.0, vec!["CZ", "EN"], vec!["CZ", "EN", "SK"]),
            create_mock("bm3", "The Dark Knight Rises", "Poslední vzdor.", 2012, "Akční", "https://images.unsplash.com/photo-1509347528160-9a9e33742cdb?q=80&w=500&auto=format&fit=crop", "", "movie", Some("2h 45m"), 8.4, vec!["CZ", "EN"], vec!["CZ"]),
            
            // Seriály
            create_mock("s2", "Stranger Things", "Chlapec zmizí a ve městě se objeví dívka s telekinetickými schopnostmi.", 2016, "Sci-Fi", "https://images.unsplash.com/photo-1614145121029-83a9f7b68bf4?q=80&w=500&auto=format&fit=crop", "", "series", Some("4 Série"), 8.7, vec!["CZ", "EN"], vec!["CZ", "EN"]),
            create_mock("s3", "The Last of Us", "Po pádu civilizace je otrlý přeživší najat, aby propašoval 14letou dívku.", 2023, "Drama", "https://images.unsplash.com/photo-1605806616949-1e87b487cb2a?q=80&w=500&auto=format&fit=crop", "", "series", Some("1 Série"), 8.8, vec!["CZ", "EN"], vec!["CZ", "EN"]),
            create_mock("s4", "Breaking Bad", "Učitel chemie s rakovinou začíná vařit pervitin.", 2008, "Krimi", "https://images.unsplash.com/photo-1587314645258-202d08a0d4c8?q=80&w=500&auto=format&fit=crop", "", "series", Some("5 Sérií"), 9.5, vec!["CZ", "EN"], vec!["CZ", "EN"]),
            create_mock("s5", "Dark", "Zmizení dvou dětí v německém městečku odkrývá hříchy a tajemství čtyř rodin.", 2017, "Mysteriózní", "https://images.unsplash.com/photo-1509347528160-9a9e33742cdb?q=80&w=500&auto=format&fit=crop", "", "series", Some("3 Série"), 8.7, vec!["DE", "CZ", "EN"], vec!["CZ", "EN"]),
            create_mock("s6", "The Mandalorian", "Bounty hunter v předaleké galaxii.", 2019, "Sci-Fi", "https://images.unsplash.com/photo-1534447677768-be436bb09401?q=80&w=500&auto=format&fit=crop", "", "series", Some("3 Série"), 8.7, vec!["CZ", "EN"], vec!["CZ", "EN"]),
            create_mock("s7", "Arcane", "Dvě sestry, dvě města, jedna válka.", 2021, "Animovaný", "https://images.unsplash.com/photo-1618042164219-62c820f10723?q=80&w=500&auto=format&fit=crop", "", "series", Some("2 Série"), 9.0, vec!["CZ", "EN"], vec!["CZ", "EN"])
        ];
        
        for item in mock_items {
            let cast_json = item.cast.map(|v| serde_json::to_string(&v).unwrap_or_default());
            let genres_json = item.genres.map(|v| serde_json::to_string(&v).unwrap_or_default());
            let moods_json = item.moods.map(|v| serde_json::to_string(&v).unwrap_or_default());
            let audio_json = item.audio_languages.map(|v| serde_json::to_string(&v).unwrap_or_default());
            let subs_json = item.subtitle_languages.map(|v| serde_json::to_string(&v).unwrap_or_default());
            
            conn.execute(
                "INSERT INTO media (id, title, description, year, genre, posterUrl, backdropUrl, type, duration, ageRating, contentWarnings, cast_json, genres_json, moods_json, local_file_path, audio_languages_json, subtitle_languages_json, rating)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18)",
                params![
                    item.id,
                    item.title,
                    item.description,
                    item.year,
                    item.genre,
                    item.poster_url,
                    item.backdrop_url,
                    item.r#type,
                    item.duration,
                    item.age_rating,
                    item.content_warnings,
                    cast_json,
                    genres_json,
                    moods_json,
                    item.local_file_path,
                    audio_json,
                    subs_json,
                    item.rating.unwrap_or(8.0)
                ],
            )?;
        }
        
        let collections = vec![
            ("c_spider", "Kolekce: Spider-Man (Popořadě)"),
            ("c_matrix", "Kolekce: The Matrix"),
            ("c_batman", "Kolekce: Temný rytíř (Batman)"),
        ];
        
        for (id, title) in collections {
            conn.execute("INSERT INTO collections (id, title) VALUES (?1, ?2)", params![id, title])?;
        }
        
        let spiderman_ids = vec!["sp1", "sp2", "sp3", "sp4", "sp5", "sp6", "sp7"];
        for s_id in spiderman_ids {
            conn.execute("INSERT INTO media_collections (media_id, collection_id) VALUES (?1, ?2)", params![s_id, "c_spider"])?;
        }
        
        let matrix_ids = vec!["m9", "mx2", "mx3", "mx4"];
        for m_id in matrix_ids {
            conn.execute("INSERT INTO media_collections (media_id, collection_id) VALUES (?1, ?2)", params![m_id, "c_matrix"])?;
        }

        let batman_ids = vec!["bm1", "bm2", "bm3"];
        for b_id in batman_ids {
            conn.execute("INSERT INTO media_collections (media_id, collection_id) VALUES (?1, ?2)", params![b_id, "c_batman"])?;
        }
        
        let wishlist_ids = vec!["m1", "s3", "m5", "sp1", "sp7"];
        for w_id in wishlist_ids {
            conn.execute("INSERT INTO wishlist (media_id) VALUES (?1)", params![w_id])?;
        }
    }
    
    Ok(())
}

fn map_row_to_media(row: &rusqlite::Row) -> Result<MediaItem> {
    let cast_str: Option<String> = row.get(11)?;
    let genres_str: Option<String> = row.get(12)?;
    let moods_str: Option<String> = row.get(13)?;
    let audio_str: Option<String> = row.get(15).ok();
    let sub_str: Option<String> = row.get(16).ok();
    let rating: Option<f64> = row.get(17).ok();
    
    let cast = cast_str.and_then(|s| serde_json::from_str(&s).ok());
    let genres = genres_str.and_then(|s| serde_json::from_str(&s).ok());
    let moods = moods_str.and_then(|s| serde_json::from_str(&s).ok());
    let audio_languages = audio_str.and_then(|s| serde_json::from_str(&s).ok());
    let subtitle_languages = sub_str.and_then(|s| serde_json::from_str(&s).ok());
    
    Ok(MediaItem {
        id: row.get(0)?,
        title: row.get(1)?,
        description: row.get(2)?,
        year: row.get(3)?,
        genre: row.get(4)?,
        poster_url: row.get(5)?,
        backdrop_url: row.get(6)?,
        r#type: row.get(7)?,
        duration: row.get(8)?,
        age_rating: row.get(9)?,
        content_warnings: row.get(10)?,
        cast,
        genres,
        moods,
        local_file_path: row.get(14)?,
        audio_languages,
        subtitle_languages,
        rating,
    })
}

pub fn get_media(conn: &Connection, type_filter: Option<&str>) -> Result<Vec<MediaItem>> {
    let query = match type_filter {
        Some(t) => format!("SELECT id, title, description, year, genre, posterUrl, backdropUrl, type, duration, ageRating, contentWarnings, cast_json, genres_json, moods_json, local_file_path, audio_languages_json, subtitle_languages_json, rating FROM media WHERE type = '{}'", t),
        None => "SELECT id, title, description, year, genre, posterUrl, backdropUrl, type, duration, ageRating, contentWarnings, cast_json, genres_json, moods_json, local_file_path, audio_languages_json, subtitle_languages_json, rating FROM media".to_string()
    };
    
    let mut stmt = conn.prepare(&query)?;
    let media_iter = stmt.query_map([], map_row_to_media)?;
    
    let mut items = Vec::new();
    for item in media_iter {
        items.push(item?);
    }
    
    Ok(items)
}

pub fn get_collection(conn: &Connection, collection_id: &str) -> Result<Vec<MediaItem>> {
    let mut stmt = conn.prepare(
        "SELECT m.id, m.title, m.description, m.year, m.genre, m.posterUrl, m.backdropUrl, m.type, m.duration, m.ageRating, m.contentWarnings, m.cast_json, m.genres_json, m.moods_json, m.local_file_path, m.audio_languages_json, m.subtitle_languages_json, m.rating 
         FROM media m
         JOIN media_collections mc ON m.id = mc.media_id
         WHERE mc.collection_id = ?1"
    )?;
    
    let media_iter = stmt.query_map(params![collection_id], map_row_to_media)?;
    
    let mut items = Vec::new();
    for item in media_iter {
        items.push(item?);
    }
    
    Ok(items)
}

pub fn get_wishlist(conn: &Connection) -> Result<Vec<MediaItem>> {
    let mut stmt = conn.prepare(
        "SELECT m.id, m.title, m.description, m.year, m.genre, m.posterUrl, m.backdropUrl, m.type, m.duration, m.ageRating, m.contentWarnings, m.cast_json, m.genres_json, m.moods_json, m.local_file_path, m.audio_languages_json, m.subtitle_languages_json, m.rating 
         FROM media m
         JOIN wishlist w ON m.id = w.media_id
         ORDER BY w.added_at DESC"
    )?;
    
    let media_iter = stmt.query_map([], map_row_to_media)?;
    
    let mut items = Vec::new();
    for item in media_iter {
        items.push(item?);
    }
    
    Ok(items)
}

pub fn toggle_wishlist(conn: &Connection, media_id: &str) -> Result<bool> {
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM wishlist WHERE media_id = ?1")?;
    let count: i64 = stmt.query_row(params![media_id], |row| row.get(0))?;
    
    if count > 0 {
        conn.execute("DELETE FROM wishlist WHERE media_id = ?1", params![media_id])?;
        Ok(false)
    } else {
        conn.execute("INSERT INTO wishlist (media_id) VALUES (?1)", params![media_id])?;
        Ok(true)
    }
}

pub fn is_in_wishlist(conn: &Connection, media_id: &str) -> Result<bool> {
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM wishlist WHERE media_id = ?1")?;
    let count: i64 = stmt.query_row(params![media_id], |row| row.get(0))?;
    Ok(count > 0)
}

// ==========================================
// SPRÁVA SLOŽEK A PLNOHODNOTNÝ SKENER SOUBORŮ
// ==========================================

pub fn get_scan_folders(conn: &Connection) -> Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT path FROM scan_folders ORDER BY added_at ASC")?;
    let folder_iter = stmt.query_map([], |row| row.get(0))?;
    let mut folders = Vec::new();
    for f in folder_iter {
        folders.push(f?);
    }
    Ok(folders)
}

pub fn add_scan_folder(conn: &Connection, path: &str) -> Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO scan_folders (path) VALUES (?1)",
        params![path],
    )?;
    Ok(())
}

pub fn remove_scan_folder(conn: &Connection, path: &str) -> Result<()> {
    conn.execute(
        "DELETE FROM scan_folders WHERE path = ?1",
        params![path],
    )?;
    Ok(())
}

fn scan_dir_recursive(dir: &Path, files: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                scan_dir_recursive(&path, files);
            } else if path.is_file() {
                if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                    let ext_lower = ext.to_lowercase();
                    if ["mp4", "mkv", "avi", "mov", "m4v", "webm"].contains(&ext_lower.as_str()) {
                        files.push(path);
                    }
                }
            }
        }
    }
}

fn parse_video_filename(file_path: &Path) -> (String, i32, Vec<String>, Vec<String>, String) {
    let stem = file_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Neznámý film");

    let lower = stem.to_lowercase();
    
    // Zjištění typu: seriál vs film
    let is_series = lower.contains("s0") || lower.contains("s1") || lower.contains("s2") || lower.contains("ep") || lower.contains("e0") || lower.contains("e1");
    let media_type = if is_series { "series".to_string() } else { "movie".to_string() };

    // Zjištění jazyků zvuku
    let mut audio = Vec::new();
    if lower.contains("cz") || lower.contains("cesky") || lower.contains("dabing") || lower.contains("cze") {
        audio.push("CZ".to_string());
    }
    if lower.contains("en") || lower.contains("eng") || lower.contains("english") {
        audio.push("EN".to_string());
    }
    if audio.is_empty() {
        audio.push("CZ".to_string());
        audio.push("EN".to_string());
    }

    // Zjištění titulků
    let subtitles = vec!["CZ".to_string(), "EN".to_string()];

    // Vyčištění názvu a roku
    let clean_chars = stem.replace('.', " ").replace('_', " ").replace('-', " ");
    let parts: Vec<&str> = clean_chars.split_whitespace().collect();

    let mut detected_year = 2024;
    let mut title_parts = Vec::new();
    let mut year_found = false;

    for part in parts {
        if !year_found && part.len() == 4 && (part.starts_with("19") || part.starts_with("20")) {
            if let Ok(y) = part.parse::<i32>() {
                if (1930..=2035).contains(&y) {
                    detected_year = y;
                    year_found = true;
                    continue;
                }
            }
        }

        let p_lower = part.to_lowercase();
        // Ignorujeme známé kvality a kodeky
        if ["1080p", "720p", "2160p", "4k", "uhd", "bluray", "webrip", "webdl", "dvdrip", "x264", "x265", "hevc", "h264", "hdr", "repack"].contains(&p_lower.as_str()) {
            break;
        }

        if !year_found {
            title_parts.push(part);
        }
    }

    let title = if title_parts.is_empty() {
        clean_chars
    } else {
        title_parts.join(" ")
    };

    (title, detected_year, audio, subtitles, media_type)
}

pub fn scan_all_folders(conn: &Connection) -> Result<ScanSummary> {
    let folders = get_scan_folders(conn)?;
    let mut files = Vec::new();

    for f in &folders {
        let p = PathBuf::from(f);
        if p.exists() {
            scan_dir_recursive(&p, &mut files);
        }
    }

    let files_found = files.len();
    let mut new_added = 0;
    let mut new_titles = Vec::new();

    let default_posters = [
        "https://images.unsplash.com/photo-1536440136628-849c177e76a1?w=800&auto=format&fit=crop&q=80",
        "https://images.unsplash.com/photo-1489599849927-2ee91cede3ba?w=800&auto=format&fit=crop&q=80",
        "https://images.unsplash.com/photo-1518709268805-4e9042af9f23?w=800&auto=format&fit=crop&q=80",
        "https://images.unsplash.com/photo-1509198397868-475647b2a1e5?w=800&auto=format&fit=crop&q=80",
    ];

    for (idx, file) in files.iter().enumerate() {
        let path_str = file.to_string_lossy().to_string();

        let mut check_stmt = conn.prepare("SELECT COUNT(*) FROM media WHERE local_file_path = ?1")?;
        let exists: i64 = check_stmt.query_row(params![path_str], |row| row.get(0))?;

        if exists == 0 {
            let (title, year, audio, subs, m_type) = parse_video_filename(file);
            let id = format!("loc_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_micros());
            let audio_json = serde_json::to_string(&audio).unwrap_or_default();
            let subs_json = serde_json::to_string(&subs).unwrap_or_default();
            let poster = default_posters[idx % default_posters.len()].to_string();
            let backdrop = poster.clone();
            let desc = format!("Lokální soubor nalezený v: {}", path_str);

            conn.execute(
                "INSERT INTO media (id, title, description, year, genre, posterUrl, backdropUrl, type, duration, ageRating, contentWarnings, cast_json, genres_json, moods_json, local_file_path, audio_languages_json, subtitle_languages_json, rating)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18)",
                params![
                    id,
                    title,
                    desc,
                    year,
                    "Lokální média",
                    poster,
                    backdrop,
                    m_type,
                    "Neznámá délka",
                    "12+",
                    "Lokální soubor",
                    "[]",
                    "[\"Lokální\"]",
                    "[\"Nové\"]",
                    path_str,
                    audio_json,
                    subs_json,
                    8.2
                ],
            )?;

            new_titles.push(title);
            new_added += 1;
        }
    }

    Ok(ScanSummary {
        scanned_folders: folders.len(),
        files_found,
        new_added,
        new_titles,
    })
}
