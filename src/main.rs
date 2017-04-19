extern crate carbon;
use carbon::server::{Server, Message};
use std::net::{TcpListener, TcpStream};
use carbon::service::Service;
use carbon::http::{Response, Request};
//use carbon::router::{Router, Action};

struct Hello;
impl Service for Hello {
	
	fn ready(&mut self, mut message: Message) -> Message {
		let mut response = Response::ok();
		//let mut router = Router::new(String::from("/")).unwrap();
		// router.serve(&Request::new().unwrap()).to_message()
		response = response.body("<html><body><h>Hello World!</h><p>Using the rust server</p></body></html>".to_string());
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


