extern crate tinyIO;
use tinyIO::server::{Server};
use std::io::{self, Read, Write, BufReader, BufRead};
use std::net::{TcpListener, TcpStream};

fn main() {
  let listener = TcpListener::bind("127.0.0.1:1300").unwrap();
  let mut echo_server = Server::new(listener);
  echo_server.initialize();
  echo_server.run();
}


