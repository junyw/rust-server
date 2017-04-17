extern crate tinyIO;
use tinyIO::server::{Server, Message};
use std::io::{self, Read, Write, BufReader, BufRead};
use std::net::{TcpListener, TcpStream};
use tinyIO::service::Service;

struct Echo;
impl Service for Echo {
	
	fn ready(&mut self, mut message: Message) -> Message {
		println!("service working...");
		message.print();
		message.write(b"result");
		message
	}	
}
fn main() {
  let listener = TcpListener::bind("127.0.0.1:1300").unwrap();
  
  let service = Echo;

  let mut server = Server::new(listener, service);

  server.initialize();
  server.run();
	
}


