use std::path::{Path,PathBuf};
use std::fs::File;
use http::Response;
//use http::{Request, Response, Method};
use std::io::prelude::*;
use std::error::Error;
use std::env;
use std::collections::HashMap;

pub trait View {
    fn render(&self, cache: &mut HashMap<String, String>) -> Response;
}

pub struct NotFound;
impl View for NotFound {
	fn render(&self, cache: &mut HashMap<String, String>) -> Response {
		let response = Response::ok()
					   .body("<html><body> PAGE NOT FOUND </body></html>".to_string());
		response
	}
}
pub struct Page {
	url: PathBuf,
}
impl Page {
	pub fn new(path: &'static str) -> Page {
		let mut path_buf = env::current_dir().expect("current directory error");
		path_buf.push(Path::new(path));
		Page {
			url: path_buf,
		}
	}
}
impl View for Page {
	fn render(&self, cache: &mut HashMap<String, String>) -> Response {
		let response = Response::ok();
		let mut s = String::new();
		{
			let mut file = match File::open(self.url.as_path()) {
	        	Err(why) => return Response::not_found(), // can not find file
	        	Ok(file) => file,
		    };
	    	match file.read_to_string(&mut s) {
	        	Err(why) => println!("couldn't read file: {}", why.description()),
	        	Ok(_) => (),
	    	} 
   		} // file has been closed at this point
		response.body(s)
	}
}

