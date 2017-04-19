extern crate carbon;
use carbon::server::{Server, Message};
use std::io::{self, Read, Write, BufReader, BufRead};
use std::net::{TcpListener, TcpStream};
use carbon::service::Service;
use carbon::http::Request;
use carbon::router::*;
use carbon::view::*;
use std::path::{PathBuf, Path};
use std::env;

struct Echo;
impl Service for Echo {
  
  fn ready(&mut self, message: Message) -> Message {

    let mut request = Request::new().unwrap();
    request.parse(message.to_str());
    
    println!("Request Method and URI {:?} {:?}", request.method(), request.uri());
    

    let mut router = RouterBuilder::new().get(r"/$", Box::new(StaticPage::new(r"examples/index.html"))).build();
                                         //.get(r"/info", Box::new(StaticPage::new()));
    router.response(request.method(), &request.uri()).to_message() 
  } 
}
fn main() {
  let listener = TcpListener::bind("127.0.0.1:1300").unwrap();
  
  let service = Echo;

  let mut server = Server::new(listener, service);

  server.run();
  
}


