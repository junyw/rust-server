
/* 
there are two major functions for the kqueue system in OS X
 int kqueue(void);
 int kevent(int kq, const struct kevent *changelist, int nchanges,
      struct kevent *eventlist, int nevents, const struct timespec *timeout);
nix provides these bindings:
  fn kqueue() -> Result<RawFd>
  fn kevent(kq: RawFd, changelist: &[KEvent], eventlist: &mut [KEvent], timeout_ms: usize) -> Result<usize>
  The changelist is used to register events with kqueue.
  The eventlist contains all the events which are currently active at the time of polling.

nix defined a KEvent structure:
   pub struct KEvent {
      pub ident: uintptr_t,
      pub filter: EventFilter,
      pub flags: EventFlag,
      pub fflags: FilterFlag,
      pub data: intptr_t,
      pub udata: usize,
   }
*/
use std::ops::BitAnd;
// Read
// IO of the stream
use std::io::{self, Read, Write, BufReader, BufRead};
use nix::sys::event::{KEvent, kqueue, kevent, EventFilter, FilterFlag};
use nix::sys::event::{EV_ADD, EV_ENABLE, EV_DELETE, EV_CLEAR, EV_ONESHOT, EV_ERROR};
use std::os::unix::io::{AsRawFd, RawFd};
use std::net::{TcpListener, TcpStream};

use io::event::Event;

pub trait Handler {
    fn ready(&mut self, id:RawFd, ev_set : EventSet, event_loop : &mut EventLoop);
}

pub struct EventLoop {
	kqueue: RawFd,
	// evList is used for retrival
	evList: Vec<KEvent>,
}
impl EventLoop {
	pub fn new() -> io::Result<EventLoop> {
		let kq = kqueue().expect("could not get kq");
		println!("created kq = {}", kq);
		Ok(EventLoop {
			kqueue: kq,
			evList: vec![Event::new_timer_event(0,0).kevent],
		})
	}
	fn ev_register(&self, event: Event) {
		let changes = vec![event.kevent];
		match kevent(self.kqueue, &changes, &mut [], 0) {
			Ok(v) => (),
			_ => ()
		}
	}
	pub fn register(&self, id: &Identifier) {
		let mut event = Event::new(&id);
		self.ev_register(event);

	}
	pub fn reregister() {
		//TO DO: implement me.
	}
	pub fn deregister(&self, id: &Identifier) {
		let mut event = Event::new(&id);
		event.ev_set_delete();
		self.ev_register(event);

	}

	pub fn run<H: Handler>(&mut self, handler: &mut H) {
		self.poll(handler);
	}
	fn poll<H: Handler>(&mut self, handler: &mut H) {
	  //println!("polling...");
	 //let mut changes : Vec<KEvent> = vec![];
	  //let mut changes = Vec::with_capacity(1024);
	  //changes.push(event(0,0));
	  //changes.push(event(0,0));
      match kevent(self.kqueue, &[], self.evList.as_mut_slice(), 0) {
	      Ok(n) if n > 0 => {
	        println!("poll triggered......");
	        for i in 0..n {
				// if (evi.flags & EV_ERROR)
				//     /* error */
				// if (evi.filter == EVFILT_READ)
				//     readable_fd(evi.ident);
				// if (evi.filter == EVFILT_WRITE)
				//     writeable_fd(evi.ident);
	          println!("Event with ID {:?} triggered", self.evList.get(i).unwrap().ident);
	          let mut ev_set : EventSet;
	          if self.evList[i].filter == EventFilter::EVFILT_READ  {
	          	ev_set = EventSet::readable();
	          	ev_set.set_data(self.evList[i].data as usize);
	          } else if self.evList[i].filter == EventFilter::EVFILT_WRITE {
	          	ev_set = EventSet::writable();
	          } else {
	          	ev_set = EventSet::new();
	          }
	          handler.ready(self.evList.get(i).unwrap().ident as i32,
	          	ev_set, 
	          	self);
	          // since we have a connection, accept it and start a stream
	          // the problem is we don't know which event it corresponds to 
	          
	          // we start to handle the event;
	          //let stream = listener.accept().unwrap().0;
	          //handle_request(stream);
	          //match listener.accept() {
	          //  Ok((stream, addr)) => println!("new client: {:?}", addr),
	          //  Err(e) => println!("couldn't get client: {:?}", e),
	          //} 
	        }
	      }
	      Ok(n) if n <= 0 => {
	      	//error or time out
	      }
	      Err(e) => panic!("{:?}", e), // Panic on Errors
	      _ => () // Ignore Ok(0),
	    }
	}
}


pub struct Identifier {
	fd: RawFd,
	filter: Interest,
}
impl Identifier {
	pub fn new(fd: RawFd, interest: Interest) -> Identifier {
		Identifier {
			fd: fd,
			filter: interest,
		}
	}
	pub fn get_fd(&self) -> RawFd {
		self.fd
	}
	pub fn readable(&self) -> bool {
		match self.filter {
			Interest::Read => true,
			_ => false,
		}
	}
	pub fn writable(&self) -> bool {
		match self.filter {
			Interest::Write => true,
			_ => false,
		}	}
}

pub enum Interest {
    Read,
    Write,
}
pub struct EventSet(usize, usize);
impl EventSet {
	// TODO: implement me
	pub fn new() -> EventSet {
		EventSet(0, 0)
	}
	pub fn readable() -> EventSet {
		EventSet(0x001, 0)
	}
	pub fn writable() -> EventSet {
		EventSet(0x002, 0)
	}
	pub fn set_data(&mut self, data :usize) {
		self.1 = data;
	}
	pub fn get_data(&self) -> usize {
		self.1
	}
	pub fn is_readable(&self) -> bool {
		self.0 & 0x001 == 0x001
	}
	pub fn is_writable(&self) -> bool {
		self.0 & 0x002 == 0x002
	}
	pub fn is_error(&self) {
		//TODO
	}
}
