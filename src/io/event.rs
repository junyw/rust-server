use nix::sys::event::{KEvent, kqueue, kevent, EventFilter, FilterFlag};
use nix::sys::event::{EV_ADD, EV_ENABLE, EV_DELETE, EV_CLEAR, EV_ONESHOT, EV_ERROR};
use std::os::unix::io::{AsRawFd, RawFd};

use io::notification::Identifier;
pub struct Event {
	pub kevent: KEvent,
}
impl Event {
	pub fn new(id: &Identifier) -> Event {
		let mut kevent = Event::new_kevent(&id.get_fd());
		if id.readable() {
			kevent.filter = EventFilter::EVFILT_READ;
			Event {
	 	      kevent: kevent,
		    }
		} else if id.writable() {
			kevent.filter = EventFilter::EVFILT_WRITE;
			Event {
	 	      kevent: kevent,
		    }
		} else {
			panic!("");
		}
   		
	} 
	pub fn new_from_kevent(kevent: KEvent) -> Event {
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
	pub fn ev_set_add(&mut self) {
		self.kevent.flags = EV_ADD | EV_ENABLE;
	}
	pub fn ev_set_write(&mut self) {
		self.kevent.filter = EventFilter::EVFILT_WRITE;
		self.kevent.flags = EV_ADD | EV_ENABLE;
	}
	pub fn ev_set_delete(&mut self) {
		self.kevent.flags = EV_DELETE;
	}
	fn new_kevent(id: & RawFd) -> KEvent {
		KEvent {
	        ident: *id as usize, 
	        filter: EventFilter::EVFILT_READ,
	        //flags: EV_ADD | EV_ENABLE,
	        // EV_CLEAR for edge, EV_ONESHOT for oneshot
	        flags: EV_ADD | EV_ENABLE ,
	        fflags: FilterFlag::empty(),
	        data: 0,
	        udata: 0,
	    }
	}
	pub fn new_timer_event(id: usize, timer: isize) -> Event {
		// helper function to create a new Event
		// this is a timer event
		// id is a value used to identify the event.
		// timer is a timer in milliseconds.
		// EV_ADD | EV_ENABLE indicates we want to add and enable the timer at the same time.
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
