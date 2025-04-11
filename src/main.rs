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
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);

    if request.starts_with("GET /get") {
        
        let mut stmt = db.prepare("SELECT id, title FROM todos")?;

        let todos_iter = stmt.query_map([], |row| {
            Ok(Todo {
                id: row.get(0)?,
                title: row.get(1)?,
            })
        })?;

        let mut todos = Vec::new();

        for todo in todos_iter {
            todos.push(todo?);
        }

        let body = serde_json::to_string(&todos).unwrap();
        
        respond_json(&mut stream, &body);
    
    }
}

fn respond_json(stream: &mut TcpStream, body: &str) {
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        body.len(),
        body
    );
    stream.write_all(response.as_bytes()).unwrap();
}