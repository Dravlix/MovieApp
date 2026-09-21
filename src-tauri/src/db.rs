use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
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
            local_file_path TEXT
        )",
        [],
    )?;
    
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
    
    seed_db_if_empty(&conn)?;
    
    Ok(conn)
}

fn seed_db_if_empty(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM media")?;
    let count: i64 = stmt.query_row([], |row| row.get(0))?;
    
    if count == 0 {
        let mock_items = vec![
            MediaItem {
                id: "m1".to_string(),
                title: "THE SHIMMER".to_string(),
                description: "Skupina vědců vstupuje do anomální zóny zvané Lesk, kde neplatí zákony přírody, biologie ani fyziky. Co najdou uvnitř, navždy změní lidstvo.".to_string(),
                year: 2026,
                genre: "Sci-Fi / Thriller".to_string(),
                poster_url: "https://images.unsplash.com/photo-1618331835717-801e976710b2?q=80&w=800&auto=format&fit=crop".to_string(),
                backdrop_url: "https://images.unsplash.com/photo-1618331835717-801e976710b2?q=80&w=1920&auto=format&fit=crop".to_string(),
                r#type: Some("movie".to_string()),
                duration: Some("1h 55m".to_string()),
                age_rating: Some("18+".to_string()),
                content_warnings: Some("násilí, hrubý jazyk".to_string()),
                cast: Some(vec!["Natalie Portman".to_string(), "Oscar Isaac".to_string(), "Jennifer Jason Leigh".to_string()]),
                genres: Some(vec!["Sci-Fi".to_string(), "Thriller".to_string(), "Mysteriózní".to_string()]),
                moods: Some(vec!["Temné".to_string(), "Napínavé".to_string(), "Mind-bending".to_string()]),
                local_file_path: None,
            },
            MediaItem {
                id: "s1".to_string(),
                title: "CYBER CITY".to_string(),
                description: "V neónem ozářeném městě budoucnosti se detektiv snaží rozplést síť korporátních konspirací.".to_string(),
                year: 2025,
                genre: "Sci-Fi / Akční".to_string(),
                poster_url: "https://images.unsplash.com/photo-1518773553398-650c184e0bb3?q=80&w=800&auto=format&fit=crop".to_string(),
                backdrop_url: "https://images.unsplash.com/photo-1518773553398-650c184e0bb3?q=80&w=1920&auto=format&fit=crop".to_string(),
                r#type: Some("series".to_string()),
                duration: Some("3 Série".to_string()),
                age_rating: Some("15+".to_string()),
                content_warnings: Some("násilí, drogy".to_string()),
                cast: Some(vec!["Keanu Reeves".to_string(), "Carrie-Anne Moss".to_string()]),
                genres: Some(vec!["Sci-Fi".to_string(), "Akční".to_string(), "Kyberpunk".to_string()]),
                moods: Some(vec!["Akční".to_string(), "Drsné".to_string()]),
                local_file_path: None,
            },
            MediaItem { id: "m2".to_string(), title: "Blade Runner 2049".to_string(), description: "".to_string(), year: 2017, genre: "Sci-Fi".to_string(), poster_url: "https://images.unsplash.com/photo-1534447677768-be436bb09401?q=80&w=500&auto=format&fit=crop".to_string(), backdrop_url: "".to_string(), r#type: Some("movie".to_string()), duration: None, age_rating: None, content_warnings: None, cast: None, genres: None, moods: None, local_file_path: None, },
            MediaItem { id: "m3".to_string(), title: "Interstellar".to_string(), description: "".to_string(), year: 2014, genre: "Sci-Fi".to_string(), poster_url: "https://images.unsplash.com/photo-1444703686981-a3abbc4d4fe3?q=80&w=500&auto=format&fit=crop".to_string(), backdrop_url: "".to_string(), r#type: Some("movie".to_string()), duration: None, age_rating: None, content_warnings: None, cast: None, genres: None, moods: None, local_file_path: None, },
            MediaItem { id: "m4".to_string(), title: "Drive".to_string(), description: "".to_string(), year: 2011, genre: "Action".to_string(), poster_url: "https://images.unsplash.com/photo-1492144534655-ae79c964c9d7?q=80&w=500&auto=format&fit=crop".to_string(), backdrop_url: "".to_string(), r#type: Some("movie".to_string()), duration: None, age_rating: None, content_warnings: None, cast: None, genres: None, moods: None, local_file_path: None, },
            MediaItem { id: "m5".to_string(), title: "Joker".to_string(), description: "".to_string(), year: 2019, genre: "Drama".to_string(), poster_url: "https://images.unsplash.com/photo-1620336655055-088d06e36bf0?q=80&w=500&auto=format&fit=crop".to_string(), backdrop_url: "".to_string(), r#type: Some("movie".to_string()), duration: None, age_rating: None, content_warnings: None, cast: None, genres: None, moods: None, local_file_path: None, },
            MediaItem { id: "m6".to_string(), title: "Mad Max".to_string(), description: "".to_string(), year: 2015, genre: "Action".to_string(), poster_url: "https://images.unsplash.com/photo-1542362567-b07e54358753?q=80&w=500&auto=format&fit=crop".to_string(), backdrop_url: "".to_string(), r#type: Some("movie".to_string()), duration: None, age_rating: None, content_warnings: None, cast: None, genres: None, moods: None, local_file_path: None, },
            MediaItem { id: "m7".to_string(), title: "Dune".to_string(), description: "".to_string(), year: 2021, genre: "Sci-Fi".to_string(), poster_url: "https://images.unsplash.com/photo-1547333590-478e7279b940?q=80&w=500&auto=format&fit=crop".to_string(), backdrop_url: "".to_string(), r#type: Some("movie".to_string()), duration: None, age_rating: None, content_warnings: None, cast: None, genres: None, moods: None, local_file_path: None, },
            MediaItem { id: "m8".to_string(), title: "John Wick".to_string(), description: "".to_string(), year: 2014, genre: "Action".to_string(), poster_url: "https://images.unsplash.com/photo-1566373767864-16a3a7891825?q=80&w=500&auto=format&fit=crop".to_string(), backdrop_url: "".to_string(), r#type: Some("movie".to_string()), duration: None, age_rating: None, content_warnings: None, cast: None, genres: None, moods: None, local_file_path: None, },
            MediaItem { id: "m9".to_string(), title: "The Matrix".to_string(), description: "Co je Matrix?".to_string(), year: 1999, genre: "Sci-Fi".to_string(), poster_url: "https://images.unsplash.com/photo-1526304640581-d334cdbbf45e?q=80&w=500&auto=format&fit=crop".to_string(), backdrop_url: "".to_string(), r#type: Some("movie".to_string()), duration: None, age_rating: None, content_warnings: None, cast: None, genres: None, moods: None, local_file_path: None, },
            MediaItem { id: "m10".to_string(), title: "Inception".to_string(), description: "".to_string(), year: 2010, genre: "Sci-Fi".to_string(), poster_url: "https://images.unsplash.com/photo-1618042164219-62c820f10723?q=80&w=500&auto=format&fit=crop".to_string(), backdrop_url: "".to_string(), r#type: Some("movie".to_string()), duration: None, age_rating: None, content_warnings: None, cast: None, genres: None, moods: None, local_file_path: None, },
            MediaItem { id: "m11".to_string(), title: "Terminator 2".to_string(), description: "".to_string(), year: 1991, genre: "Action".to_string(), poster_url: "https://images.unsplash.com/photo-1535295972055-1c762f4483e5?q=80&w=500&auto=format&fit=crop".to_string(), backdrop_url: "".to_string(), r#type: Some("movie".to_string()), duration: None, age_rating: None, content_warnings: None, cast: None, genres: None, moods: None, local_file_path: None, },
            
            MediaItem { id: "sp1".to_string(), title: "Spider-Man".to_string(), description: "Peter Parker získá pavoučí schopnosti.".to_string(), year: 2002, genre: "Action".to_string(), poster_url: "https://images.unsplash.com/photo-1635805737707-575885ab0820?q=80&w=500&auto=format&fit=crop".to_string(), backdrop_url: "".to_string(), r#type: Some("movie".to_string()), duration: None, age_rating: None, content_warnings: None, cast: None, genres: None, moods: None, local_file_path: None, },
            MediaItem { id: "sp2".to_string(), title: "Spider-Man 2".to_string(), description: "Peter Parker čelí Dr. Octopusovi.".to_string(), year: 2004, genre: "Action".to_string(), poster_url: "https://images.unsplash.com/photo-1635805737707-575885ab0820?q=80&w=500&auto=format&fit=crop".to_string(), backdrop_url: "".to_string(), r#type: Some("movie".to_string()), duration: None, age_rating: None, content_warnings: None, cast: None, genres: None, moods: None, local_file_path: None, },
            MediaItem { id: "sp3".to_string(), title: "Spider-Man 3".to_string(), description: "Peter bojuje s temnotou v sobě a s Venomem.".to_string(), year: 2007, genre: "Action".to_string(), poster_url: "https://images.unsplash.com/photo-1635805737707-575885ab0820?q=80&w=500&auto=format&fit=crop".to_string(), backdrop_url: "".to_string(), r#type: Some("movie".to_string()), duration: None, age_rating: None, content_warnings: None, cast: None, genres: None, moods: None, local_file_path: None, },
            MediaItem { id: "sp4".to_string(), title: "The Amazing Spider-Man".to_string(), description: "Reboot s Andrewem Garfieldem.".to_string(), year: 2012, genre: "Action".to_string(), poster_url: "https://images.unsplash.com/photo-1635805737707-575885ab0820?q=80&w=500&auto=format&fit=crop".to_string(), backdrop_url: "".to_string(), r#type: Some("movie".to_string()), duration: None, age_rating: None, content_warnings: None, cast: None, genres: None, moods: None, local_file_path: None, },
            MediaItem { id: "sp5".to_string(), title: "Spider-Man: Homecoming".to_string(), description: "Peter Parker v Marvel Cinematic Universe.".to_string(), year: 2017, genre: "Action".to_string(), poster_url: "https://images.unsplash.com/photo-1635805737707-575885ab0820?q=80&w=500&auto=format&fit=crop".to_string(), backdrop_url: "".to_string(), r#type: Some("movie".to_string()), duration: None, age_rating: None, content_warnings: None, cast: None, genres: None, moods: None, local_file_path: None, },
            MediaItem { id: "sp6".to_string(), title: "Spider-Man: Far From Home".to_string(), description: "Peter na školním výletě v Evropě.".to_string(), year: 2019, genre: "Action".to_string(), poster_url: "https://images.unsplash.com/photo-1635805737707-575885ab0820?q=80&w=500&auto=format&fit=crop".to_string(), backdrop_url: "".to_string(), r#type: Some("movie".to_string()), duration: None, age_rating: None, content_warnings: None, cast: None, genres: None, moods: None, local_file_path: None, },
            MediaItem { id: "sp7".to_string(), title: "Spider-Man: No Way Home".to_string(), description: "Multivesmír se otevírá.".to_string(), year: 2021, genre: "Action".to_string(), poster_url: "https://images.unsplash.com/photo-1635805737707-575885ab0820?q=80&w=500&auto=format&fit=crop".to_string(), backdrop_url: "".to_string(), r#type: Some("movie".to_string()), duration: None, age_rating: None, content_warnings: None, cast: None, genres: None, moods: None, local_file_path: None, },
            
            MediaItem { id: "mx2".to_string(), title: "The Matrix Reloaded".to_string(), description: "Neo se učí ovládat své schopnosti.".to_string(), year: 2003, genre: "Sci-Fi".to_string(), poster_url: "https://images.unsplash.com/photo-1526304640581-d334cdbbf45e?q=80&w=500&auto=format&fit=crop".to_string(), backdrop_url: "".to_string(), r#type: Some("movie".to_string()), duration: None, age_rating: None, content_warnings: None, cast: None, genres: None, moods: None, local_file_path: None, },
            MediaItem { id: "mx3".to_string(), title: "The Matrix Revolutions".to_string(), description: "Závěrečná bitva o Zion.".to_string(), year: 2003, genre: "Sci-Fi".to_string(), poster_url: "https://images.unsplash.com/photo-1526304640581-d334cdbbf45e?q=80&w=500&auto=format&fit=crop".to_string(), backdrop_url: "".to_string(), r#type: Some("movie".to_string()), duration: None, age_rating: None, content_warnings: None, cast: None, genres: None, moods: None, local_file_path: None, },
            MediaItem { id: "mx4".to_string(), title: "The Matrix Resurrections".to_string(), description: "Návrat ke kořenům v nové formě.".to_string(), year: 2021, genre: "Sci-Fi".to_string(), poster_url: "https://images.unsplash.com/photo-1526304640581-d334cdbbf45e?q=80&w=500&auto=format&fit=crop".to_string(), backdrop_url: "".to_string(), r#type: Some("movie".to_string()), duration: None, age_rating: None, content_warnings: None, cast: None, genres: None, moods: None, local_file_path: None, },
            
            MediaItem { id: "bm1".to_string(), title: "Batman Begins".to_string(), description: "Zrození temného rytíře.".to_string(), year: 2005, genre: "Action".to_string(), poster_url: "https://images.unsplash.com/photo-1509347528160-9a9e33742cdb?q=80&w=500&auto=format&fit=crop".to_string(), backdrop_url: "".to_string(), r#type: Some("movie".to_string()), duration: None, age_rating: None, content_warnings: None, cast: None, genres: None, moods: None, local_file_path: None, },
            MediaItem { id: "bm2".to_string(), title: "The Dark Knight".to_string(), description: "Joker rozpoutává chaos.".to_string(), year: 2008, genre: "Action".to_string(), poster_url: "https://images.unsplash.com/photo-1509347528160-9a9e33742cdb?q=80&w=500&auto=format&fit=crop".to_string(), backdrop_url: "".to_string(), r#type: Some("movie".to_string()), duration: None, age_rating: None, content_warnings: None, cast: None, genres: None, moods: None, local_file_path: None, },
            MediaItem { id: "bm3".to_string(), title: "The Dark Knight Rises".to_string(), description: "Poslední vzdor.".to_string(), year: 2012, genre: "Action".to_string(), poster_url: "https://images.unsplash.com/photo-1509347528160-9a9e33742cdb?q=80&w=500&auto=format&fit=crop".to_string(), backdrop_url: "".to_string(), r#type: Some("movie".to_string()), duration: None, age_rating: None, content_warnings: None, cast: None, genres: None, moods: None, local_file_path: None, },
            
            MediaItem { id: "s2".to_string(), title: "Stranger Things".to_string(), description: "".to_string(), year: 2016, genre: "Sci-Fi".to_string(), poster_url: "https://images.unsplash.com/photo-1614145121029-83a9f7b68bf4?q=80&w=500&auto=format&fit=crop".to_string(), backdrop_url: "".to_string(), r#type: Some("series".to_string()), duration: None, age_rating: None, content_warnings: None, cast: None, genres: None, moods: None, local_file_path: None, },
            MediaItem { id: "s3".to_string(), title: "The Last of Us".to_string(), description: "".to_string(), year: 2023, genre: "Drama".to_string(), poster_url: "https://images.unsplash.com/photo-1605806616949-1e87b487cb2a?q=80&w=500&auto=format&fit=crop".to_string(), backdrop_url: "".to_string(), r#type: Some("series".to_string()), duration: None, age_rating: None, content_warnings: None, cast: None, genres: None, moods: None, local_file_path: None, },
            MediaItem { id: "s4".to_string(), title: "Breaking Bad".to_string(), description: "".to_string(), year: 2008, genre: "Crime".to_string(), poster_url: "https://images.unsplash.com/photo-1587314645258-202d08a0d4c8?q=80&w=500&auto=format&fit=crop".to_string(), backdrop_url: "".to_string(), r#type: Some("series".to_string()), duration: None, age_rating: None, content_warnings: None, cast: None, genres: None, moods: None, local_file_path: None, },
            MediaItem { id: "s5".to_string(), title: "Dark".to_string(), description: "".to_string(), year: 2017, genre: "Mystery".to_string(), poster_url: "https://images.unsplash.com/photo-1509347528160-9a9e33742cdb?q=80&w=500&auto=format&fit=crop".to_string(), backdrop_url: "".to_string(), r#type: Some("series".to_string()), duration: None, age_rating: None, content_warnings: None, cast: None, genres: None, moods: None, local_file_path: None, },
            MediaItem { id: "s6".to_string(), title: "The Mandalorian".to_string(), description: "Bounty hunter v předaleké galaxii.".to_string(), year: 2019, genre: "Sci-Fi".to_string(), poster_url: "https://images.unsplash.com/photo-1534447677768-be436bb09401?q=80&w=500&auto=format&fit=crop".to_string(), backdrop_url: "".to_string(), r#type: Some("series".to_string()), duration: None, age_rating: None, content_warnings: None, cast: None, genres: None, moods: None, local_file_path: None, },
            MediaItem { id: "s7".to_string(), title: "Arcane".to_string(), description: "Dvě sestry, dvě města, jedna válka.".to_string(), year: 2021, genre: "Animation".to_string(), poster_url: "https://images.unsplash.com/photo-1618042164219-62c820f10723?q=80&w=500&auto=format&fit=crop".to_string(), backdrop_url: "".to_string(), r#type: Some("series".to_string()), duration: None, age_rating: None, content_warnings: None, cast: None, genres: None, moods: None, local_file_path: None, }
        ];
        
        for item in mock_items {
            let cast_json = item.cast.map(|v| serde_json::to_string(&v).unwrap_or_default());
            let genres_json = item.genres.map(|v| serde_json::to_string(&v).unwrap_or_default());
            let moods_json = item.moods.map(|v| serde_json::to_string(&v).unwrap_or_default());
            
            conn.execute(
                "INSERT INTO media (id, title, description, year, genre, posterUrl, backdropUrl, type, duration, ageRating, contentWarnings, cast_json, genres_json, moods_json, local_file_path)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
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
    
    let cast = cast_str.and_then(|s| serde_json::from_str(&s).ok());
    let genres = genres_str.and_then(|s| serde_json::from_str(&s).ok());
    let moods = moods_str.and_then(|s| serde_json::from_str(&s).ok());
    
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
    })
}

pub fn get_media(conn: &Connection, type_filter: Option<&str>) -> Result<Vec<MediaItem>> {
    let query = match type_filter {
        Some(t) => format!("SELECT id, title, description, year, genre, posterUrl, backdropUrl, type, duration, ageRating, contentWarnings, cast_json, genres_json, moods_json, local_file_path FROM media WHERE type = '{}'", t),
        None => "SELECT id, title, description, year, genre, posterUrl, backdropUrl, type, duration, ageRating, contentWarnings, cast_json, genres_json, moods_json, local_file_path FROM media".to_string()
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
        "SELECT m.id, m.title, m.description, m.year, m.genre, m.posterUrl, m.backdropUrl, m.type, m.duration, m.ageRating, m.contentWarnings, m.cast_json, m.genres_json, m.moods_json, m.local_file_path 
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
        "SELECT m.id, m.title, m.description, m.year, m.genre, m.posterUrl, m.backdropUrl, m.type, m.duration, m.ageRating, m.contentWarnings, m.cast_json, m.genres_json, m.moods_json, m.local_file_path 
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
