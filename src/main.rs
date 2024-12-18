use console::style;
use dialoguer::Confirm;
use rusqlite::Result;
use std::env;
use todo::*;

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
        "remove" => {
            if args.len() < 3 {
                help()?;
                std::process::exit(1);
            } else {
                let id = args[2].parse::<i32>().unwrap();
                Todo::remove(&conn, id)?;
                println!("Removed task with ID: {}", id);
            }
            Ok(())
        }
        "reset" => {
            let confirmation = Confirm::new()
                .with_prompt(
                    style("Do you really want to reset?")
                        .bright()
                        .red()
                        .to_string(),
                )
                .interact();
            match confirmation {
                Ok(c) => {
                    if c {
                        Todo::reset(&conn)?;
                        println!("Database reset, all tasks were removed.");
                    } else {
                        println!("Allright! well done.");
                    }
                }
                Err(err) => {
                    eprintln!("{}", err)
                }
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
