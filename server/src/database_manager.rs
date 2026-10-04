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

// pub fn register_user(user: &Register) {
//     let conn = Connection::open("data.db").unwrap();

//     conn.execute(
//         "SELECT MAX(id) FROM users"
//     ).unwrap();
// }

// fn get_current_max_id(conn: &Connection) {
//     conn.execute(
//         "SELECT MAX(id) FROM users"
//     ).unwrap();
// }