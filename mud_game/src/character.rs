use rusqlite::{Connection, Result};
use std::io;

use crate::user::Id;

pub enum GenderType {Man, Woman}

fn create_character(userid: &Id, name: &String, gender: &GenderType, connection: &Connection) -> Result<()> {
    let str_gender: &str = match gender {
        GenderType::Man => "Man",
        GenderType::Woman => "Woman",
    };

    connection.execute(
        "INSERT INTO Characters (user_id, name, gender) VALUES (?1, ?2, ?3)",
        rusqlite::params![userid, name, str_gender],
    )?;

    Ok(())
}

fn delete_character(userid: &Id, id : u64, connection: &Connection) -> Result<bool>
{
    let changecount : usize = connection.execute(
        "DELETE FROM Characters WHERE user_id = ?1 AND id = ?2",
        rusqlite::params![userid, id],
    )?;

    if changecount > 0 {
        println!("Delete Character Success");
        return Ok(true);
    }
    else {
        println!("Character does not exist");
        return Err(rusqlite::Error::InvalidQuery);
    }
}

pub fn show_all_characters(userid: &Id, connection : &Connection)
{
    let mut stmt = connection.prepare("SELECT id, name, gender FROM Characters WHERE user_id == ?1")
        .expect("Query Prepare Failed");

    let mut rows = stmt.query([*userid as u64]).unwrap();
    let mut bis_empty = true;

    while let Ok(Some(row)) = rows.next()
    {
        bis_empty = false;
        println!("character id : {} name : {} gender : {}", row.get::<_,u64>(0).unwrap(), row.get::<_,String>(1).unwrap(), row.get::<_,String>(2).unwrap());
    }

    if bis_empty == true {
        println!("characters empty");
    }
}

pub fn create_character_flow(userid: &Id, connection: &Connection) -> Result<()>
{
    let mut name_input = String::new();
    println!("Enter character name:");
    io::stdin().read_line(&mut name_input).expect("Invalid Input");

    let mut gender_input = String::new();
    println!("Enter character gender (Man/Woman):");
    io::stdin().read_line(&mut gender_input).expect("Invalid Input");

    let name = name_input.trim();
    let gender = match gender_input.trim().to_lowercase().as_str() {
        "man" => GenderType::Man,
        "woman" => GenderType::Woman,
        _ => {
            println!("Invalid gender");
            return Err(rusqlite::Error::InvalidQuery);
        }
    };

    if let Err(err) = create_character(userid, &name.to_string(), &gender, connection) {
        println!("Failed to create character: {}", err);
        return Err(err);
    }

    println!("Character created successfully");
    Ok(())
}

pub fn delete_character_flow(userid: &Id, connection: &Connection)
{
    let mut id_input = String::new();
    println!("Enter character ID to delete:");
    io::stdin().read_line(&mut id_input)
        .expect("Failed to read input");

    let id: u64 = match id_input.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid character ID");
            return;
        }
    };

    if let Err(_) = delete_character(userid, id, connection) {
        return;
    }

    println!("Character deleted successfully");

}
