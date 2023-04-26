use rusqlite::{Connection, Result};
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use std::collections::HashMap;
use std::fs;
use std::io;

struct PasswordDummyData {
    id: String,
    name: String,

}

fn create_database(database_name: String) -> Result<()>{

    let passw = PasswordDummyData{
        id: String::from("1"),
        name: String::from("Test")
    };

    let static_db_name = database_name.clone();
    let full_db_name = static_db_name.clone() + ".sqlite";
    let connection = Connection::open(full_db_name)?;
    connection.execute("create table if not exists passwords (
        id integer primary key, 
        name text not null unique)", ())?;
    let conn_insert = connection.execute("INSERT INTO passwords (id, name) values (?1, ?2)", (&passw.id, &passw.name));
    match conn_insert {
        Ok(conn_insert) => {
            println!("Yeayea {:?}", conn_insert);
        }
        Err(e) =>{
            println!("Sql not working \n {:?}", e);
        }
    }
    
    //encrypt(static_db_name);
    Ok(())

}

fn encrypt(database_name: String) {
    let static_db_name = database_name.clone();
    let pwd = new_magic_crypt!("pwd", 256);

    let open_file = fs::read(static_db_name);
    match open_file {
        Ok(open_file) => {
            println!("File found {:?}", open_file);
        }
        Err(e) =>{
            println!("Error file not found \n {:?}", e);
        }
    }
    

}

fn main() -> Result<()> {
   let mut line = String::new();
   println!("Enter database name:");
   let input1 = io::stdin().read_line(&mut line);
   let line = line.trim().to_string();
   create_database(line).expect("Not working");
    Ok(())
}