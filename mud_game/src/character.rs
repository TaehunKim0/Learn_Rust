use rusqlite::{Connection, Result};
use std::io;

use crate::user::Id;

pub enum GenderType {Man, Woman}
pub type CharacterId = u64;

pub struct CharacterInfo
{
    user_id : Id,
    name : String,
    gender : GenderType,
}

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

pub fn show_all_characters(userid: &Id, connection : &Connection)
{
    let mut stmt = connection.prepare("SELECT name, gender FROM Characters")
        .expect("Query Prepare Failed");

    let mut rows = stmt.query([]).unwrap();

    while let Ok(Some(row)) = rows.next()
    {
        println!("{} {}", row.get::<_,String>(0).unwrap(), row.get::<_,String>(1).unwrap());
    }

}

pub fn create_character_flow(userid: &Id, connection: &Connection) -> Result<()>
{
    let mut name_input = String::new();
    println!("Enter character name:");
    io::stdin().read_line(&mut name_input);

    let mut gender_input = String::new();
    println!("Enter character gender (Man/Woman):");
    io::stdin().read_line(&mut gender_input);

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
