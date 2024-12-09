use rusqlite::Result;
use std::env;
use todolist_by_rust::*;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        std::process::exit(1);
    }

    let conn = get_connection()?;

    let command = &args[1];
    let remaining: &String = &args[2..].iter().cloned().collect::<Vec<_>>().join(" ");

    match command.as_str() {
        "add" => {
            if remaining.as_str().is_empty() {
                std::process::exit(1);
            } else {
                Todo::add(&conn, remaining.as_str())?;
            }
            Ok(())
        }
        "toggle" => {
            if args.len() < 3 {
                std::process::exit(1);
            } else {
                let id = args[2].parse::<i32>().unwrap();
                Todo::toggle(&conn, id)?;
                println!("Toggle task with Id: {}", id);
            }
            Ok(())
        }
        "list" => {
            println!("TODO List (sorted by id):");
            let todos = Todo::list(&conn, false)?;
            Todo::print_list(todos)?;
            Ok(())
        }
        "help" | "--help" | "-h" | _ => help(),
    }?;

    Ok(())
}
