use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use rusqlite::{params, Connection, Result};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    id: i64,
    title: String,
}

#[derive(Debug, Deserialize)]
struct NewTodo {
    title: String,
}

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Server berjalan di http://localhost:8080");

    let db = Connection::open("todos.db")?;
    db.execute("CREATE TABLE IF NOT EXISTS todos (id INTEGER PRIMARY KEY, title TEXT);", [])?;

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, &db)?;
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream, db: &Connection) -> Result<()> {

}