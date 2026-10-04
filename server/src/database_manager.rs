use rusqlite::{Connection, Result};

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

// TO-DO change to RUSQLITE??