use console::style;
use rusqlite::{Connection, Result};
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct Todo {
    pub id: i32,
    pub name: String,
    pub date_added: String,
    pub is_done: u8,
}
impl Todo {
    pub fn new(id: i32, name: String, date_added: String, is_done: u8) -> Self {
        Todo {
            id,
            name,
            date_added,
            is_done,
        }
    }

    pub fn add(conn: &Connection, name: &str) -> Result<()> {
        conn.execute("INSERT INTO todo (name) VALUES (?)", &[name])?;
        Ok(())
    }

    pub fn remove(conn: &Connection, id: i32) -> Result<()> {
        conn.execute("DELETE FROM todo WHERE id = ?", &[&id])?;
        Ok(())
    }

    pub fn reset(conn: &Connection) -> Result<()> {
        conn.execute("DELETE FROM todo", ())?;
        Ok(())
    }

    pub fn list(conn: &Connection, sort_by_status: bool) -> Result<Vec<Todo>> {
        let sql = if sort_by_status {
            "SELECT * FROM todo ORDER BY is_done, id"
        } else {
            "SELECT * FROM todo ORDER BY id"
        };
        let mut stmt = conn.prepare(sql)?;
        let todo_iter = stmt.query_map((), |row| {
            Ok(Todo::new(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
            ))
        })?;

        let mut todos = Vec::new();
        for todo in todo_iter {
            todos.push(todo?);
        }
        Ok(todos)
    }

    pub fn print_list(todos: Vec<Todo>) -> Result<()> {
        for todo in todos {
            let status = if todo.is_done == 1 {
                style("Done").green()
            } else {
                style("Pending").red()
            };
            println!(
                "{:>4} | {:<44} {:<8} {}",
                style(todo.id).cyan().bright(),
                style(truncate_at(&todo.name, 44)).bright(),
                status,
                style(todo.date_added).dim(),
            );
        }
        Ok(())
    }

    pub fn toggle(conn: &Connection, id: i32) -> Result<()> {
        conn.execute("UPDATE todo SET is_done = 1 - is_done WHERE id = ?", &[&id])?;
        Ok(())
    }
}

pub fn get_connection() -> Result<Connection> {
    let db_folder = "./todo_db/".to_string();
    let db_file_path = format!("{}{}", db_folder, "todo.sqlite");
    verify_db_path(&db_folder)?;
    let conn = Connection::open(db_file_path)?;
    verify_db(&conn)?;
    Ok(conn)
}

pub fn verify_db_path(db_folder: &str) -> Result<()> {
    if !Path::new(db_folder).exists() {
        match fs::create_dir(db_folder) {
            Ok(_) => println!("Folder '{}' created.", db_folder),
            Err(err) => eprintln!("Error creating folder: {}", err),
        }
    }
    Ok(())
}

pub fn verify_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo (
                id INTEGER NOT NULL,
                name TEXT NOT NULL,
                date_added REAL NOT NULL DEFAULT current_timestamp,
                is_done NUMERIC NOT NULL DEFAULT 0,
                PRIMARY KEY(id AUTOINCREMENT)
            )",
        [],
    )?;
    Ok(())
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
    let help_title = "\nAvailable commands:";
    let help_text = r#"
        - add [TASK]
            add new task/s

        - toggle [TASK_ID]
            toggle the status of a task (Done/Pending)
            
        - remove [TASK_ID]
            remove a task
            
        - reset
            remove all tasks
        "#;
    println!("{}", style(help_title).cyan().bright());
    println!("{}", style(help_text).green());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;
    use std::sync::Mutex;

    lazy_static! {
        static ref DATABASE_CONNECTION: Mutex<Connection> = {
            let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
            verify_db(&conn).expect("Cannot create tables");
            Mutex::new(conn)
        };
    }

    fn reset_db(conn: &Connection) -> Result<()> {
        conn.execute("DELETE FROM todo", ())?;
        Ok(())
    }

    fn contains_task(todos: &Vec<Todo>, target_name: &str) -> bool {
        for todo in todos {
            if todo.name == target_name {
                return true;
            }
        }
        false
    }

    #[test]
    fn test_add_todo() {
        let conn = DATABASE_CONNECTION.lock().expect("Mutex lock failed");
        reset_db(&conn).expect("failed to resetting the db");

        let name = "Test todo";
        Todo::add(&conn, name).expect("Failed to add todo.");

        let mut stmt = conn
            .prepare("SELECT COUNT(*) FROM todo WHERE name = ?")
            .expect("Failed to prepare statement");
        let count: i32 = stmt
            .query_row(&[name], |row| row.get(0))
            .expect("Failed to prepare statement");

        assert_eq!(count, 1, "Todo was not added to the database.")
    }

    #[test]
    fn test_list_todo() {
        let conn = DATABASE_CONNECTION.lock().expect("Mutex lock failed.");
        reset_db(&conn).expect("Failed to reset the db.");

        Todo::add(&conn, "Task 1").expect("could not add todo");
        Todo::add(&conn, "Task 2").expect("could not add todo");
        Todo::add(&conn, "Task 3").expect("could not add todo");

        let todos = Todo::list(&conn, false).expect("Failed to list todo");
        assert_eq!(
            todos.len(),
            3,
            "Wrong number of todo items returned by list()"
        )
    }

    #[test]
    fn test_sort_todo() {
        let conn = DATABASE_CONNECTION.lock().expect("Mutext lock failed.");
        reset_db(&conn).expect("Failed to resetting the db.");
        Todo::add(&conn, "Task 1").expect("Could not add todo.");
        Todo::add(&conn, "Task 2").expect("Could not add Todo.");
        Todo::add(&conn, "Task 3").expect("Could not add Todo.");
        let todos = Todo::list(&conn, false).expect("Failed to list todo.");
        Todo::toggle(&conn, todos[0].id).expect("Could not to toggle first entry.");
        let todos = Todo::list(&conn, true).expect("Failed to sort todos");
        assert_eq!(
            todos[2].name, "Task 1",
            "The todo marked as done was not the LAST one returned"
        );
        assert_eq!(
            todos.len(),
            3,
            "Wrong number of todo items returned by sort()"
        );
    }

    #[test]
    fn test_rm_todo() {
        let conn = DATABASE_CONNECTION.lock().expect("Mutext lock failed");
        reset_db(&conn).expect("Failed to reset the db.");
        Todo::add(&conn, "Task 1").expect("Could not add Todo.");
        Todo::add(&conn, "Task 2").expect("Could not add Todo.");
        Todo::add(&conn, "Task 3").expect("Could not add Todo.");
        let todos = Todo::list(&conn, false).expect("Failed to list Todos");
        Todo::remove(&conn, todos[0].id).expect("Could not remove first todo.");
        let todos = Todo::list(&conn, false).expect("Failed to list Todos.");
        dbg!(&todos);
        assert_eq!(
            todos.len(),
            2,
            "Wrong number of todo items returned by sort()"
        );
        assert_eq!(
            contains_task(&todos, "Task 1"),
            false,
            "Task 1 was not deleted."
        )
    }

    #[test]
    fn test_toggle_todo() {
        let conn = DATABASE_CONNECTION.lock().expect("Mutex lock failed.");
        reset_db(&conn).expect("Failed to reset the db.");
        Todo::add(&conn, "Task 1").expect("Coudld not add todos.");
        Todo::add(&conn, "Task 2").expect("Could not add todos.");

        let todos = Todo::list(&conn, false).expect("Failed to list todos.");
        Todo::toggle(&conn, todos[0].id).expect("Could not toggle first todo.");
        let todos = Todo::list(&conn, false).expect("Failed to sort todos.");
        dbg!(&todos);
        assert_eq!(
            todos.len(),
            2,
            "Wrong number of todo items returned by toggle()"
        );
        assert_eq!(todos[0].is_done, 1, "Task 1 was not toggled.");
    }
}
