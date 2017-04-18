extern crate tinyIO;
use tinyIO::server::{Server, Message};
use std::io::{self, Read, Write, BufReader, BufRead};
use std::net::{TcpListener, TcpStream};
use tinyIO::service::Service;
use tinyIO::http::Response;

struct Hello;
impl Service for Hello {
	
	fn ready(&mut self, mut message: Message) -> Message {
		let mut response = Response::ok();
		response.body("<html><body><h>Hello World!</h><p>Using the rust server</p></body></html>".to_string());
		println!("{}", response.to_string());
		response.to_message()
	}	
}
fn main() {
  let listener = TcpListener::bind("127.0.0.1:1300").unwrap();
  
  let service = Hello;

  let mut server = Server::new(listener, service);

  //server.initialize();
  server.run();
	
}


