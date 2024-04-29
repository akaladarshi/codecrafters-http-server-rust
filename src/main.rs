use std::{env, fs, io, thread};
use std::fmt::Error;
// Uncomment this block to pass the first stage
use std::net::{TcpListener, TcpStream};
use std::path::Path;

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
        thread::spawn(|| {
            println!("Received connection");
            if let Ok(s) = stream {
                match handle_conn(s) {
                    Ok(_) => {},
                    Err(e) => eprintln!("failed to handle the conn: {}", e)
                }
            }
        });
    }
}

fn handle_conn(mut conn: TcpStream) -> Result<(), Error> {
    println!("Handling connection");
    let mut req =  reqs::new();
    req.parse_data(&mut conn)?;
    let res = process_req(req).map_err(|_| Error)?;
    res.write(&mut conn)
}

fn process_req(req: reqs) -> Result<Response,io::Error> {
    let echo_reg = Regex::new(r"^/echo/([a-z]+)").unwrap();
    let files_regex = Regex::new(r"^/files/(.+)").unwrap();
    match req.get_path() {
        "/" => Ok(resp::create_response(HTTP_STATUS_OK, Body::empty())),
        "/user-agent" => {
            let body = Body::new(CONTENT_TYPE_TEXT,  Vec::from(req.get_data("user-agent")));
            Ok(resp::create_response(HTTP_STATUS_OK, body))
        },
        path if echo_reg.is_match(path) => {
            let captures = echo_reg.captures(path).unwrap().get(1).unwrap();
            let body = Body::new(CONTENT_TYPE_TEXT,  Vec::from(captures.as_str()));
            Ok(resp::create_response(HTTP_STATUS_OK, body))
        },
        path if files_regex.is_match(path) => {
            let captures = files_regex.captures(path).unwrap().get(1).unwrap();
            match handle_files(captures.as_str()) {
                Ok(res) => Ok(res),
                Err(e) => Err(io::Error::new(io::ErrorKind::Other, format!("failed to handle files: {}", e)))
            }
        }
        _ => Ok(resp::create_response(HTTP_STATUS_NOT_FOUND, Body::empty()))
    }
}

fn handle_files(file_name: &str) -> Result<Response, io::Error> {
    let full_path = Path::new(get_directory().as_str()).join(file_name);
    if !full_path.exists() {
        return Ok(resp::create_response(HTTP_STATUS_NOT_FOUND, Body::empty()))
    }

    let content = fs::read_to_string(full_path)?;
    Ok(resp::create_response(HTTP_STATUS_OK, Body::new(CONTENT_TYPE_OCTET, Vec::from(content))))
}

fn get_directory() -> String {
    // first value is always name of the binary
    let mut args = env::args().skip(1);
    let mut dir = String::new();
    while let Some(arg) = args.next() {
        if arg == "--directory" {
            dir = args.next().unwrap_or_default();
        }
    }

   dir
}