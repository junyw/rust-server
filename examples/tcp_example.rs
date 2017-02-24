//extern crate nix;

//use nix::sys::event::{KEvent, kqueue, kevent, EventFilter, FilterFlag};
//use nix::sys::event::{EV_ADD, EV_ENABLE};

use std::os::unix::io::AsRawFd;
use std::os::unix::io::RawFd;

//IO of the stream
use std::io::{Read, Write, BufReader, BufRead};

// sockets
use std::net::{TcpListener, TcpStream};
extern crate tinyIO;
//use tinyIO::tcpEvent;
use tinyIO::io::{Event, EventLoop, Handler};
fn main() {
  let mut event_loop : EventLoop= EventLoop::new().unwrap();
  let listener = TcpListener::bind("127.0.0.1:1300").unwrap();

  let mut tcp_event = Event::new_tcp_event(&listener);
  event_loop.register(&mut tcp_event);

  let mut server = Server::new(listener);
  loop {
    //println!("polling..");
    event_loop.run(&mut server);
  }
}
pub struct Server {
  id: RawFd,
  listener: TcpListener,
}
impl Server {
  fn new(tcp: TcpListener) -> Server {
    Server {
      id: tcp.as_raw_fd(),
      listener: tcp,
    }
  }

  // Accept a new client connection.
  fn accept(&mut self, event_loop: &mut EventLoop) {
    println!("accept new connection.");
    // Accept a new incoming connection from this listener with function accept(). 
    // fn accept(&self) -> Result<(TcpStream, SocketAddr)>
    let stream = self.listener.accept().unwrap().0;
    self.handle_request(stream);

    // let sock = match self.listener.accept() {
    //   Ok(s) => {
    //     match s {
    //       Some(sock) => sock,
    //       None => {
    //         println!("failed to accept new socket");
    //         return;
    //       }
    //     }
    //   },
    //   Err(e) +> {
    //     println!("failed to accept new socket, {}", e);
    //     return;
    //   }
    // }

  //   let sock = match self.sock.accept() {
  //       Ok(s) => {
  //           match s {
  //               Some(sock) => sock,
  //               None => {
  //                   error!("Failed to accept new socket");
  //                   self.reregister(event_loop);
  //                   return;
  //               }
  //           }
  //       },
  //       Err(e) => {
  //           error!("Failed to accept new socket, {:?}", e);
  //           self.reregister(event_loop);
  //           return;
  //       }
  //     }
  // } 
  }
  fn handle_request(&self, stream: TcpStream) {
    let mut reader = BufReader::new(stream);

    for line in reader.by_ref().lines() {
      let one_line = line.unwrap();
      println!("{}", one_line);
      if one_line == "" {
            break;
      }
    }
    self.send_response(reader.into_inner());
  }

  fn send_response(&self, mut stream: TcpStream) {
      // Write the header and the html body
      let response = "HTTP/1.1 200 OK\n\n<html><body>Hello, World!</body></html>";
      stream.write_all(response.as_bytes()).unwrap();
  }
}
impl Handler for Server {
    fn ready(&mut self, id: RawFd, event_loop: &mut EventLoop) {
      println!("Event is {}", id);
      // if events.is_error() {
      //       return;
      // }
      // if events.is_hup() {
      //       return;
      // }
      // if events.is_writable() {
      //     //Do something.
      // }
      if self.id == id {
        self.accept(event_loop);
      }
    }
}

