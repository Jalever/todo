use console::style;
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

    pub fn add(conn: &Connection, name: &str) -> Result<()> {
        conn.execute("INSERT INTO todolist (name) VALUES (?)", &[name])?;
        Ok(())
    }

    pub fn list(conn: &Connection, sort_by_id: bool) -> Result<Vec<TodoList>> {
        let sql = if sort_by_id {
            "SELECT * FROM todolist BY id"
        } else {
            "SELECT * FROM todolist BY is_done, id"
        };
        let mut stmt = conn.prepare(sql)?;
        let todo_iter = stmt.query_map((), |row| {
            Ok(TodoList::new(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
            ))
        })?;
        let mut todo_item = Vec::new();
        for item in todo_iter {
            todo_item.push(item?);
        }
        Ok(todo_item)
    }

    pub fn print_list(todo_list: Vec<TodoList>) -> Result<()> {
        for todo_item in todo_list {
            let status = if todo_item.is_done == 1 {
                style("Done").green()
            } else {
                style("Pending").red()
            };
            print!(
                "{:>4} | {:<44} {:<8} {}",
                style(todo_item.id).cyan().bright(),
                style(truncate_at(&todo_item.name, 44)).bright(),
                status,
                style(todo_item.date_added).dim(),
            )
        }
        Ok(())
    }
}

pub fn truncate_at(input: &str, max: i32) -> String {
    let max_len: usize = max as usize;
    if input.len() > max_len {
        let truncated = &input[..(max_len - 3)];
        return format!("{}...", truncated);
    };

    input.to_string()
}

pub fn help() -> Result<()> {
    let help_title = "\nAvailable commands";
    let help_text = r#"    
        - add [TASK]
            Ads new task/s
            Example: tbr add "take a shower"

        - list
            Lists all tasks
            Example: tbr list
    "#;
    println!("{}", style(help_title).cyan().bright());
    println!("{}", style(help_text).green());
    Ok(())
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
