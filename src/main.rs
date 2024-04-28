use std::fmt::Error;
// Uncomment this block to pass the first stage
use std::net::{TcpListener, TcpStream};

use regex::Regex;

use request::Request as reqs;
use response::Response as resp;

use crate::body::Body;
use crate::constants::*;
use crate::response::Response;

mod header;
mod response;
mod request;
mod constants;
mod body;
mod content;

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
    let echo_reg = Regex::new(r"^/echo/([a-z]+)").unwrap();
    match req.get_path() {
        "/" => resp::create_response(HTTP_STATUS_OK, Body::empty()),
        "/user-agent" => {
            let body = Body::new(CONTENT_TYPE_TEXT,  Vec::from(req.get_data("user-agent")));
            resp::create_response(HTTP_STATUS_OK, body)
        },
        path if echo_reg.is_match(path) => {
            let captures = echo_reg.captures(path).unwrap().get(1).unwrap();
            let body = Body::new(CONTENT_TYPE_TEXT,  Vec::from(captures.as_str()));
            resp::create_response(HTTP_STATUS_OK, body)
        },
        _ => resp::create_response(HTTP_STATUS_NOT_FOUND, Body::empty())
    }
}
