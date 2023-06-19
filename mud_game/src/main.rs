use rusqlite::{Connection, Result};
use std::io;

mod user;
mod character;

fn connect_to_database() -> Result<Connection> {
    Connection::open("data/game.db")
}

fn main() {
    let conn = connect_to_database();
    if let Err(_) = conn {
        println!("Failed to connect to the database");
        return;
    }

    let conn = conn.unwrap();

    let mut choice = String::new();

    println!("1. Create User");
    println!("2. Login");

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");

    let user_id : Result<user::Id> = match choice.trim().parse::<u64>() {
        Ok(1) => user::create_user_flow(&conn),
        Ok(2) => user::login_flow(&conn),
        _ => Err(rusqlite::Error::InvalidQuery),
    };

    println!("1. Create Character");
    println!("2. Show all Characters");
    
    let mut characterchoice = String::new();

    io::stdin()
        .read_line(&mut characterchoice)
        .expect("Failed to read input");

    match characterchoice.trim().parse::<u64>() {
        Ok(1) => match character::create_character_flow(&user_id.unwrap(), &conn){
            Ok(()) => println!("Create Character Success"),
            _ => println!("Failed to create Character"),
        }
            ,
        Ok(2) => character::show_all_characters(&user_id.unwrap(), &conn),
        _ => println!("Invalid input"),
    }
}
