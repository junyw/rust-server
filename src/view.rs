use std::path::{Path,PathBuf};
use std::fs::File;
use http::Response;
//use http::{Request, Response, Method};
use std::io::prelude::*;
use std::error::Error;
use std::env;

pub trait View {
    fn render(&self) -> Response;
}

pub struct NotFound;
impl View for NotFound {
	fn render(&self) -> Response {
		let response = Response::ok()
					   .body("<html><body> PAGE NOT FOUND </body></html>".to_string());
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
		let response = Response::ok();
		
		let mut root: PathBuf = env::current_dir().expect("current directory error");
		root.push(Path::new(self.url));
		let path = root.as_path();
		let display = path.display();
		let mut s = String::new();
		{
			let mut file = match File::open(&path) {
	        	Err(why) => {
	        					println!("couldn't open {}: {}", display,
	                                                   why.description());
	        					return Response::not_found(); //not found page
	        				}
	        	Ok(file) => file,
	    	};
	    	match file.read_to_string(&mut s) {
	        	Err(why) => println!("couldn't read {}: {}", display,
	                                                   why.description()),
	        	Ok(_) => (),//print!("{} contains:\n{}", display, s),
	    	} // file has been closed at this point
   		}
		response.body(s)
	}
}
