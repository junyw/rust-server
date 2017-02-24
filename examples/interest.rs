extern crate tinyIO;
use tinyIO::io::{EventLoop, Event, Handler, Identifier, Interest};

fn main() {
	let mut id = Identifier::new(1, Interest::Read);
	println!("{}", id.readable());
	println!("{}", id.writable());
	let mut id2 = Identifier::new(1, Interest::Write);
	println!("{}", id2.readable());
	println!("{}", id2.writable());
}