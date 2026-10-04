use rusqlite::{Connection, Result, params};
use crate::register;

pub fn init_db() {
    let conn = Connection::open("data.db").unwrap();

    conn.execute(
        "create table if not exists users (
            id integer primary key,
            username text not null,
            password text not null
        )",
        (),
    ).unwrap();
}

pub fn register_user(user: &register::RegisterUser) {
    let conn = Connection::open("data.db").unwrap();

    let mut new_id = get_current_max_id(&conn);
    new_id += 1;

    insert_user(&conn, user, new_id);
}

fn insert_user(conn: &Connection ,user: &register::RegisterUser, id: u16) {
    conn.execute(
        "INSERT INTO users (id, username, password) VALUES (?1, ?2, ?3)",
        params![id, user.get_username(), user.get_password()],
    ).unwrap();

    println!("Added user to db...");
}

fn get_current_max_id(conn: &Connection) -> u16 {

    let max_id: Option<u16> = conn.query_row(
        "SELECT MAX(id) from users",
        params![],
        |row| row.get(0),
    ).unwrap_or(None);

    max_id.unwrap_or(0)
}