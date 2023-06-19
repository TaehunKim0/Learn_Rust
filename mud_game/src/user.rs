use rusqlite::{Connection, Result};
use std::io;

pub struct UserInfo {
    id: String,
    password: String,
}

pub type Id = u64;

fn get_user(username: &str, password: &str, conn: &Connection) -> Result<Id>
{
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM users WHERE username = ?1 AND password = ?2")?;
    let id: i64 = stmt.query_row(&[username, password], |row| row.get(0))?;
    
    if id == 0 {
        return Err(rusqlite::Error::InvalidQuery);
    }

    Ok(id as Id)
}

fn create_user(username: &str, password: &str, connection: &Connection) ->Result<Id>
{
    if let Ok(_) = get_user(&username, &password, &connection)
    {
        println!("Already Created User");
        return Err(rusqlite::Error::InvalidQuery);
    }
    
    connection.execute(
        "INSERT INTO Users (username, password) VALUES (?1 , ?2)",
        &[username, password])?;

    let user_id: Option<u64> = connection.query_row(
        "SELECT MAX(ID) FROM Users",
        [],
        |row| row.get(0)
    )?;

    return Ok(user_id.unwrap() as Id);
}

fn login_user(username: &str, password: &str, conn: &Connection) -> Result<Id>
{
    let mut stmt = conn.prepare("SELECT id FROM users WHERE username = ?1 AND password = ?2")?;
    let id: i64 = stmt.query_row(&[username, password], |row| row.get(0))?;

    if id == 0 {
        return Err(rusqlite::Error::QueryReturnedNoRows);
    }

    Ok(id as Id)
}

pub fn create_user_flow(conn: &Connection) -> Result<Id> {
    let mut user_info = UserInfo {
        id: String::new(),
        password: String::new(),
    };

    println!("Enter username:");
    io::stdin()
        .read_line(&mut user_info.id)
        .expect("Failed to read input");

    println!("Enter password:");
    io::stdin()
        .read_line(&mut user_info.password)
        .expect("Failed to read input");

    match create_user(&user_info.id.trim(), &user_info.password.trim(), conn) {
        Ok(id) => {
            println!("User created successfully");
            return Ok(id as Id);
        }, 
        Err(err) => {
            println!("Failed to create user : {} ", err);
            return Err(rusqlite::Error::InvalidQuery);
        }
    }
}

pub fn login_flow(conn: &Connection) -> Result<Id> {
    let mut login_info = UserInfo {
        id: String::new(),
        password: String::new(),
    };

    println!("User Login");
    println!("Enter username:");
    io::stdin()
        .read_line(&mut login_info.id)
        .expect("Failed to read input");

    println!("Enter password:");
    io::stdin()
        .read_line(&mut login_info.password)
        .expect("Failed to read input");

    if let Ok(id) = login_user(&login_info.id.trim(), &login_info.password.trim(), conn) {
        println!("User logged in successfully");
        return Ok(id);

    } else {
        println!("Failed to log in");
        return Err(rusqlite::Error::InvalidQuery);
    }
}
