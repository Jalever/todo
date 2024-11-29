use rusqlite::Result;
use std::env;
use todolist_by_rust::{get_connection, help};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        help()?;
        std::process::exit(1);
    }

    // let conn = get_connection()?;
    let command = &args[1];
    let remaining: &String = &args[2..].iter().cloned().collect::<Vec<_>>().join(" ");

    println!("command: {}", command);
    println!("remaining: {}", remaining);

    Ok(())
}
