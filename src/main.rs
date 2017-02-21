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

  let mut tcp_event = Event::new_tcp_event(listener.as_raw_fd() as usize);
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


// fn main() {
//   // Initialize the Kqueue
//   let kq = kqueue().expect("Could not get kqueue");

//   // Create a Vec<KEvent> with both events
//   //let mut changes = vec![event(1, 1000), event(2, 1500)];

//   // Register the events in the `changelist`.
//   //kevent(kq, changes.as_slice(), &mut [], 0).unwrap();
  
//   // handle tcp events
//   //
//   let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
//   let mut changes = vec![tcpEvent(listener.as_raw_fd() as usize)];
//   kevent(kq, changes.as_slice(), &mut [], 0).unwrap();

//   loop {
//     match kevent(kq, &[], changes.as_mut_slice(), 0) {
//       Ok(v) if v > 0 => {
//         println!("---");
//         for i in 0..v {
//           println!("Event with ID {:?} triggered", changes.get(i).unwrap().ident);
          
//           // since we have a connection, accept it and start a stream
//           // the problem is we don't know which event it corresponds to 
          
//           // we start to handle the event;
//           let stream = listener.accept().unwrap().0;
//           handle_request(stream);
//           //match listener.accept() {
//           //  Ok((stream, addr)) => println!("new client: {:?}", addr),
//           //  Err(e) => println!("couldn't get client: {:?}", e),
//           //} 
//         }
//       }
//       Err(e) => panic!("{:?}", e), // Panic on Errors
//       _ => () // Ignore Ok(0),
//     }
//   }
// }
// 
