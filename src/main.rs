
mod header;
mod response;

use std::io;
use response::Response as resp;
use std::io::Read;
// Uncomment this block to pass the first stage
use std::net::{TcpListener, TcpStream};

const SERVER: &str = "127.0.0.1:4221";
fn main() {
    let listener = TcpListener::bind(SERVER).unwrap();

    println!("Started listening to server {}", SERVER);

    for stream in listener.incoming() {
        println!("Received connection");

        match stream {
            Ok(stream) => {
                match handle_conn(stream) {
                    Ok(_) => {},
                    Err(e) => println!("failed to handle the conn: {}", e)
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_conn(mut conn: TcpStream) -> io::Result<()> {
    println!("Handling connection");

    let mut buf = [0; 1024];
    let _ =  conn.read(&mut buf)?;

    resp::create_ok_response_with_data(Vec::new()).write(&mut conn)
}