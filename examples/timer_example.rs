//extern crate nix;

//use nix::sys::event::{KEvent, kqueue, kevent, EventFilter, FilterFlag};
//use nix::sys::event::{EV_ADD, EV_ENABLE};

use std::os::unix::io::AsRawFd;
use std::os::unix::io::RawFd;

// Read
// IO of the stream
//use std::io::{Read, Write, BufReader, BufRead};

// sockets
use std::net::{TcpListener, TcpStream};
extern crate tinyIO;
//use tinyIO::tcpEvent;
use tinyIO::io::{Event, EventLoop, Handler};
fn main() {
  println!("this is main");
  let mut event_loop : EventLoop= EventLoop::new().unwrap();
  let listener = TcpListener::bind("127.0.0.1:1314").unwrap();
  //let mut changes = vec![tcpEvent(listener.as_raw_fd() as usize)];
  //let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
  //let t_event = tcpEvent(listener.as_raw_fd() as usize);
  let mut tcp_event = Event::new_tcp_event(listener.as_raw_fd() as usize);
  let mut timer_event1 = Event::new_timer_event(1, 1000);
  let mut timer_event2 = Event::new_timer_event(2, 1000);
  let mut timer_event3 = Event::new_timer_event(3, 1000);
  //let mut changes = vec![event(1, 1000), event(2, 1500)];
  event_loop.register(&mut tcp_event);
  event_loop.register(&mut timer_event1);
  event_loop.register(&mut timer_event2);
  event_loop.register(&mut timer_event3);
  //event_loop.register_changes(changes);
  let mut server = Server::new();
  loop {
    //println!("polling..");
    event_loop.run(&mut server);
  }
}
pub struct Server {

}
impl Handler for Server {
    fn ready(&self, id:RawFd) {
      println!("handler ready: id is {}", id);
    } 
}
impl Server {
  fn new() -> Server {
    Server {

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
// fn handle_request(stream: TcpStream) {

//     let mut reader = BufReader::new(stream);

//     for line in reader.by_ref().lines() {
//       let one_line = line.unwrap();
//       println!("{}", one_line);
//       if one_line == "" {
//             break;
//       }
//     }

//     send_response(reader.into_inner());
// }

// fn send_response(mut stream: TcpStream) {
//     // Write the header and the html body
//     let response = "HTTP/1.1 200 OK\n\n<html><body>Hello, World!</body></html>";
//     stream.write_all(response.as_bytes()).unwrap();
// }


