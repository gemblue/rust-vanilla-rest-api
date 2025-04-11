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
   
}