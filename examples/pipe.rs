extern crate carbon;
use carbon::server::{Server, Message};
use std::io::{self, Read, Write};
use std::net::TcpListener;
use carbon::service::Service;
// use carbon::http::Request;

struct Test;
impl Service for Test {
  
  fn ready(&mut self, message: Message) -> Message {
    println!("recieved message from client{:?}", message.to_str());
    let mut response = Message::new();
    response.write(b"HTTP/1.1 200 OK\nContent-Length: 39\n\n<html><body>Hello, World!</body></html>").unwrap();
    response
  } 
}
fn main() {
  let listener = TcpListener::bind("127.0.0.1:1300").unwrap();
  
  let service = Test;

  let mut server = Server::new(listener, service);
  
  server.run();
  
}


