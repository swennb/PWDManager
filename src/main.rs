use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use std::collections::HashMap;
use std::fs;
use std::io;

mod database;

fn encrypt(database_name: String) {
    let static_db_name = database_name.clone();
    let pwd = new_magic_crypt!("pwd", 256);
    let open_file = fs::read(static_db_name).unwrap();
}

fn main() {
   let mut line = String::new();
   println!("Enter database name:");
   let input1 = io::stdin().read_line(&mut line);
   let line = line.trim().to_string();
   // Dit moet beter kunnen maar voor de time being een letterlijke copy;
   let line_copy = line.clone();
   database::create_database(line).unwrap();
   database::read_database(line_copy).unwrap();
}