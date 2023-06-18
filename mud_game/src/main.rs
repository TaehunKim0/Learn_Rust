use rusqlite::{Connection, Result};

mod user;

fn connect_to_database() -> Result<Connection> 
{
    if let Ok(_conn) = Connection::open("data/game.db") 
    {
        Ok(_conn)
    }
    else 
    {
        println!("Failed to open database");
        Err(rusqlite::Error::InvalidQuery)
    }
}

fn main() 
{
   if let Ok(_conn) = connect_to_database() 
   {
        if let Ok(_res) = user::create_user("김태훈","12345", &_conn)
        {
            println!("Create User");
        }
        else
        {
            println!("Failed Create User");
        }
   } 
   else 
   {

        println!("Failed to open the database");
   }
}
