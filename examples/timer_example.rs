//extern crate nix;

//use nix::sys::event::{KEvent, kqueue, kevent, EventFilter, FilterFlag};
//use nix::sys::event::{EV_ADD, EV_ENABLE};

use std::os::unix::io::AsRawFd;
use std::os::unix::io::RawFd;

// Read
// IO of the stream
//use std::io::{Read, Write, BufReader, BufRead};

extern crate tinyIO;
//use tinyIO::tcpEvent;
use tinyIO::io::{Event, EventLoop, Handler};
fn main() {
  println!("this is main");
  let mut event_loop : EventLoop= EventLoop::new().unwrap();

  let mut timer_event1 = Event::new_timer_event(1, 1000);
  let mut timer_event2 = Event::new_timer_event(2, 1000);
  let mut timer_event3 = Event::new_timer_event(3, 500);

  event_loop.register(&mut timer_event1);
  event_loop.register(&mut timer_event2);
  event_loop.register(&mut timer_event3);

  let mut server = Server::new();
  loop {
    //println!("polling..");
    event_loop.run(&mut server);
  }
}
pub struct Server {

}
impl Handler for Server {
    fn ready(&mut self, id: RawFd, event_loop: &mut EventLoop) {
      println!("handler ready: id is {}", id);
    } 
}
impl Server {
  fn new() -> Server {
    Server {

    }
  }
}

