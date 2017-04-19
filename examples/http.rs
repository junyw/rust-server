
extern crate carbon;
use carbon::server::{Server, Message};
use std::io::{self, Read, Write, BufReader, BufRead};
use std::net::{TcpListener, TcpStream};
use carbon::service::Service;
use carbon::http::Request;
use carbon::router::*;
use carbon::view::*;
use std::path::Path;

struct Echo;
impl Service for Echo {
  
  fn ready(&mut self, message: Message) -> Message {
    //println!("{}",message.to_str());
    let mut index = StaticPage::new("/Users/junyi/workspace/rust-server/examples/index.html");
    index.render().to_message()
    // let mut response = Message::new();
    // let mut request = Request::new().unwrap();
    // request.parse(message.to_str());
    // response.write(b"HTTP/1.1 200 OK\nContent-Length: 39\n\n<html><body>Hello, World!</body></html>").unwrap();
    // response
  } 
}
fn main() {
  let listener = TcpListener::bind("127.0.0.1:1300").unwrap();
  
  let service = Echo;

  let mut server = Server::new(listener, service);

  server.run();
  
}


