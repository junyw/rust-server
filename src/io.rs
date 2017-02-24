
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
use nix::sys::event::{KEvent, kqueue, kevent, EventFilter, FilterFlag};
use nix::sys::event::{EV_ADD, EV_ENABLE, EV_DELETE, EV_CLEAR, EV_ONESHOT, EV_ERROR};
use std::os::unix::io::{AsRawFd, RawFd};
use std::ops::BitAnd;
// Read
// IO of the stream
use std::io::{Read, Write, BufReader, BufRead};
use std::io;
// sockets
use std::net::{TcpListener, TcpStream};

pub trait Handler {
    fn ready(&mut self, id:RawFd, event : Event, event_loop : &mut EventLoop);
}

const MAX_EVENT_COUNT : usize = 1024;
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
			evList: vec![Event::new_timer_event(0,0).kevent; MAX_EVENT_COUNT],
		})
	}

	pub fn register(&self, event: &mut Event) {
		//println!("register: {}", event.kevent.ident);
	    //let changes = vec![event.kevent];
	    event.ev_set_add();
	    let changes = vec![event.kevent];
	    println!("self.kqueue is {}", self.kqueue);
		match kevent(self.kqueue, &changes, &mut [], 0) {
			Ok(v) => println!("kevent returns: {}", v),
			_ => ()
		}
	}
	pub fn register_socket<T: AsRawFd>(&self, socket: &T) {
		let mut event: Event = Event::new_socket_event(socket);
		self.register(&mut event);
	}

	pub fn reregister() {
		//TO DO: implement me.
	}
	pub fn deregister(&self, event: &mut Event) {
		//TO DO: implement me.
		event.ev_set_delete();
		let changes = vec![event.kevent];
		match kevent(self.kqueue, &changes, &mut [], 0) {
			Ok(v) => (),
			_ => ()
		}
	}
	pub fn deregister_socket<T:AsRawFd>(&self, socket: &T) {
		let mut event = Event::new_socket_event(socket);
		self.deregister(&mut event);
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
	          handler.ready(self.evList.get(i).unwrap().ident as i32,
	          	Event::new(self.evList[i]), 
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
pub struct Event {
	kevent: KEvent,
}
impl Event {
	fn new(kevent: KEvent) -> Event {
		Event {
			kevent: kevent,
		}
	} 

	pub fn get_data(&self) -> u32 {
		self.kevent.data as u32
	}
	pub fn is_readable(&self) -> bool {
		self.kevent.filter == EventFilter::EVFILT_READ
	}		

	pub fn is_writable(&self) -> bool {
		self.kevent.filter == EventFilter::EVFILT_WRITE
	}
	pub fn is_error(&self) -> bool {
		(self.kevent.flags.bits() & EV_ERROR.bits()) == EV_ERROR.bits()
	}
	pub fn is_hup(&self)  {
		//TODO: implement me
	}
	fn ev_set_add(&mut self) {
		self.kevent.flags = EV_ADD | EV_ENABLE;
	}
	fn ev_set_delete(&mut self) {
		self.kevent.flags = EV_DELETE;
	}
	pub fn new_socket_event<T: AsRawFd>(listener: & T) -> Event {
		println!("new_tcp_event: {}", listener.as_raw_fd());
		let new_event = KEvent {
	        ident: listener.as_raw_fd() as usize, 
	        filter: EventFilter::EVFILT_READ,
	        //flags: EV_ADD | EV_ENABLE,
	        // EV_CLEAR for edge, EV_ONESHOT for oneshot
	        flags: EV_ADD | EV_ENABLE ,
	        fflags: FilterFlag::empty(),
	        data: 0,
	        udata: 0,
	    };
		Event {
			kevent: new_event,
		}
	}
	pub fn new_timer_event(id: usize, timer: isize) -> Event {
		// helper function to create a new Event
		// this is a timer event
		// id is a value used to identify the event.
		// timer is a timer in milliseconds.
		// EV_ADD | EV_ENABLE indicates we want to add and enable the timer at the same time.
		println!("new_timer_event: {}", id);
		let new_event = KEvent {
	        ident: id, 
	        filter: EventFilter::EVFILT_TIMER,
	        flags: EV_ADD | EV_ENABLE,
	        fflags: FilterFlag::empty(),
	        data: timer,
	        udata: 0,
	    };
		Event {
			kevent: new_event,
		}
	}
}