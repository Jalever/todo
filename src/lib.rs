use rusqlite::{Connection, Result};
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct TodoList {
    pub id: i32,
    pub name: String,
    pub date_added: String,
    pub is_done: u8,
}

impl TodoList {
    pub fn new(id: i32, name: String, date_added: String, is_done: u8) -> Self {
        TodoList {
            id,
            name,
            date_added,
            is_done,
        }
    }
}

pub fn get_connection() -> Result<Connection> {
    let db_folder = "./todolist_db/".to_string();
    let db_file_path = db_folder.clone() + "todolist.sqlite";
    verify_db_path(&db_folder)?;

    let conn = Connection::open(db_file_path)?;
    verify_db(&conn)?;
    Ok(conn)
}

pub fn verify_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo (
    	id	        INTEGER NOT NULL,
    	name	    TEXT NOT NULL,
    	date_added	REAL NOT NULL DEFAULT current_timestamp,
    	is_done	    NUMERIC NOT NULL DEFAULT 0,
    	    PRIMARY KEY(id AUTOINCREMENT)
    )",
        [],
    )?;
    Ok(())
}

pub fn verify_db_path(db_folder: &str) -> Result<()> {
    if !Path::new(db_folder).exists() {
        match fs::create_dir(db_folder) {
            Ok(_) => println!("folder '{}' created.", db_folder),
            Err(e) => eprintln!("Error creating folder: {}", e),
        }
    }
    Ok(())
}
