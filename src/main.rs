use rusqlite::Result;
use std::env;
use todolist_by_rust::{get_connection, help, TodoList};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        help()?;
        std::process::exit(1);
    }

    let conn = get_connection()?;
    let command = &args[1];
    let remaining = &args[2..].iter().cloned().collect::<Vec<_>>().join(" ");

    match command.as_str() {
        "add" => {
            if remaining.as_str().is_empty() {
                help()?;
                std::process::exit(1);
            } else {
                TodoList::add(&conn, remaining.as_str())?;
            }
            Ok(())
        }
        "list" => {
            println!("TODO List (sorted by id):");
            let todos = TodoList::list(&conn, false)?;
            TodoList::print_list(todos)?;
            Ok(())
        }
    }?;

    Ok(())
}
