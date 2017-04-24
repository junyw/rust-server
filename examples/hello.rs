extern crate carbon;
extern crate ansi_term;
use carbon::server::{Server, Message};
use std::net::TcpListener;
use carbon::service::Service;
use carbon::http::Request;
use carbon::router::*;
use carbon::view::*;
use ansi_term::Colour::*;

// use std::path::{PathBuf, Path};
// use std::env;

struct Hello {
  router: Router,
}
impl Hello {
  pub fn new() -> Hello {
    Hello {
      router: RouterBuilder::new()
                .get(r"/$", Box::new(Page::new(r"assets/index.html")))
                .get(r"/info$", Box::new(Page::new(r"examples/info.html")))
                .get(r"/css/style.css$", Box::new(Page::new(r"assets/css/style.css")))
                .build(),
    }
  }
}
impl Service for Hello {
  
  fn ready(&mut self, message: Message) -> Message {
    let mut request = Request::parse(&message);
    
    // println!("Request Method and URI {:?} {:?}", request.method(), request.uri());
    self.router.response(request.method(), &request.uri()).to_message() 

  } 
}
fn main() {
  let listener = TcpListener::bind("192.168.0.31:1300").expect("tcp listener error");
  
  let service = Hello::new();

  let mut server = Server::new(listener, service);
  
  println!("{} http://192.168.0.31:1300", Blue.bold().paint("Open"));
  
  server.run();

}


