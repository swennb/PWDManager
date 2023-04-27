
use rusqlite::{Connection, Result, params};
#[derive(Debug)]
struct PasswordDummyData {
    id: String,
    name: String,

}
pub fn create_database(database_name: String) -> Result<()>{

    let passw = PasswordDummyData{
        id: String::from("1"),
        name: String::from("Test")
    };

    let static_db_name = database_name.clone()+ ".sqlite";
    let connection = Connection::open(static_db_name)?;
    connection.execute("create table if not exists passwords (
        id integer primary key, 
        name text not null unique)", ())?;
    connection.execute("INSERT INTO passwords (id, name) values (?1, ?2)", (&passw.id, &passw.name)).unwrap();
    //encrypt(static_db_name);
    connection.close();
    Ok(())

}

pub fn read_database(database_name: String) -> Result<()>{
    let static_db_name = database_name.clone()+".sqlite";
    let conn = Connection::open(static_db_name)?;
    let mut rec = conn.prepare("SELECT* FROM passwords")?;
    let data_iter = rec.query_map([], |row| {
        Ok(PasswordDummyData{
            id: row.get(0)?,
            name: row.get(1)?
        })
    })?;

    for x in data_iter{
        print!("Found {:?}", x);
    }

    Ok(())
}