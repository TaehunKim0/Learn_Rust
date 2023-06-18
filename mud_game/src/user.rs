use rusqlite::{Connection, Result};

pub fn create_user(username: &str, password: &str, connection: &Connection) -> Result<()> 
{
    connection.execute(
        "INSERT INTO Users (username, password) VALUES (?1 , ?2)",
        &[username, password])?;

    Ok(())
}
