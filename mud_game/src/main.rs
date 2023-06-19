use rusqlite::{Connection, Result};
use std::io;

mod character;
mod user;

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

    let user_id: Result<user::Id> = match choice.trim().parse::<u64>() {
        Ok(1) => user::create_user_flow(&conn),
        Ok(2) => user::login_flow(&conn),
        _ => return,
    };

    if let Err(_) = user_id {
        return;
    }

    let user_id = user_id.unwrap();
    let mut exit = false;

    while exit == false {
        println!("1. Create Character");
        println!("2. Show all Characters");
        println!("3. Delete Character");
        println!("4. Exit");

        let mut characterchoice = String::new();

        io::stdin()
            .read_line(&mut characterchoice)
            .expect("Failed to read input");

        match characterchoice.trim().parse::<u64>() {
            Ok(1) => match character::create_character_flow(&user_id, &conn) {
                Ok(()) => println!("Create Character Success"),
                _ => println!("Failed to create Character"),
            },
            Ok(2) => character::show_all_characters(&user_id, &conn),
            Ok(3) => character::delete_character_flow(&user_id, &conn),
            Ok(4) => { exit = true },
            _ => println!("Invalid input"),
        }
    }
}
