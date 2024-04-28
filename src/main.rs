
mod header;
mod response;
mod request;
mod constants;

use std::fmt::Error;
use response::Response as resp;
use request::Request as reqs;
// Uncomment this block to pass the first stage
use std::net::{TcpListener, TcpStream};
use crate::constants::{HTTP_STATUS_NOT_FOUND, HTTP_STATUS_OK};
use crate::response::Response;

const SERVER_ADDRESS: &str = "127.0.0.1:4221";
fn main() {
    let listener = TcpListener::bind(SERVER_ADDRESS).unwrap();

    println!("Started listening to server {}", SERVER_ADDRESS);

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

fn handle_conn(mut conn: TcpStream) -> Result<(), Error> {
    println!("Handling connection");
    let mut req =  reqs::new();
    req.parse_data(&mut conn)?;
    let res = process_req(req);
    res.write(&mut conn)
}

fn process_req(req: reqs) -> Response {
    match req.get_path(){
        "/" => resp::create_response(HTTP_STATUS_OK, vec![]),
        _ => resp::create_response(HTTP_STATUS_NOT_FOUND, vec![])
    }
}