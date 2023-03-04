use crate::{types::{open_connection, prepare_statement}, readwrite::{exec, query}};

wit_bindgen::generate!({
    path: "../wit",
});

struct MyGuest;

impl fake_handler::FakeHandler for MyGuest {
    fn handle() {
        println!(">>> handle got called in guest");
        let sql = open_connection("my-db").unwrap();
        
        let create_statement = prepare_statement("CREATE TABLE IF NOT EXISTS users (id SERIAL PRIMARY KEY, name TEXT NOT NULL)", &[]).unwrap();
        exec(sql, create_statement).unwrap();

        let insert_statement = prepare_statement("INSERT INTO users (name) VALUES (?)", &["Alice"]).unwrap();
        exec(sql, insert_statement).unwrap();

        let select_statement = prepare_statement("SELECT name FROM users", &[]).unwrap();
        query(sql, select_statement).unwrap();
    }
}

export_sql!(MyGuest);