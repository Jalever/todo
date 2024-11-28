use rusqlite::Result;
use todolist_by_rust::get_connection;

fn main() -> Result<()> {
    let conn = get_connection()?;
    Ok(())
}
