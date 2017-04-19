use std::path::Path;
use std::fs::File;
use http::{Request, Response, Method};
use std::io::prelude::*;
use std::error::Error;

pub trait View {
    fn render(&self) -> Response;
}

pub struct NotFound;
impl View for NotFound {
	fn render(&self) -> Response {
		let mut response = Response::ok();
		response.body("<html><body> PAGE NOT FOUND </body></html>".to_string());
		response
	}
}
pub struct StaticPage {
	url: &'static str,
}
impl StaticPage {
	pub fn new(path: &'static str) -> StaticPage {
		StaticPage {
			url: path,
		}
	}
}

impl View for StaticPage {
	fn render(&self) -> Response {
		let mut response = Response::ok();
		let mut path = Path::new(self.url);
		let display = path.display();

		let mut file = match File::open(&path) {
        	Err(why) => {
        					println!("couldn't open {}: {}", display,
                                                   why.description());
        					return Response::ok(); //not found page
        				}
        	Ok(file) => file,
    	};
		let mut s = String::new();
    	match file.read_to_string(&mut s) {
        	Err(why) => println!("couldn't read {}: {}", display,
                                                   why.description()),
        	Ok(_) => print!("{} contains:\n{}", display, s),
    	}
		response.body(s);
		response
	}
}
