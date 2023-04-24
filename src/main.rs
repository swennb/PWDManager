use rusqlite::{Connection, Result};
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use std::collections::HashMap;
use std::fs;

fn create_database(databaseName: String) -> Result<()>{
    let static_DB_Name = databaseName.clone();
    let nullandvoid:&String = &String::from("0");
    let connection = Connection::open(databaseName)?;
    connection.execute("create table if not exists test (
        id integer primary key, 
        name text not null unique)", ())?;
    let mut me = HashMap::new();
    me.insert(String::from("ID"), String::from("01"));
    me.insert(String::from("Name"), String::from("Swen"));
    
    connection.execute("INSERT INTO test, (id, name) VALUES (?1, ?2)", (
        &me.get("ID").unwrap_or(nullandvoid), &me.get("Name").unwrap_or(nullandvoid))
    )?;
    encrypt(static_DB_Name);
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
   let input1 = std::io::stdin().read_line(&mut line).unwrap().to_string();
   create_database(input1).expect("Not working");

    Ok(())
}