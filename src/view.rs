use std::path::{Path,PathBuf};
use std::fs::File;
use http::Response;
//use http::{Request, Response, Method};
use std::io::prelude::*;
use std::error::Error;
use std::env;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use fnv::FnvHashMap;


pub trait View {
    fn render(&self, cache: &mut FnvHashMap<String, String>) -> Response;
}

pub struct NotFound;
impl View for NotFound {
	fn render(&self, cache: &mut FnvHashMap<String, String>) -> Response {
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
	fn render(&self, cache: &mut FnvHashMap<String, String>) -> Response {
		let path = self.url.as_path().to_str().expect("failure to get file path");
		let s = match cache.entry(String::from(path)) {
			Entry::Occupied(entry) => {
				entry.get().clone()
			}
			Entry::Vacant(entry) => {

				let mut s = String::new();
				{
					let mut file = match File::open(self.url.as_path()) {
			        	Err(why) => return Response::not_found(), // can not find file
			        	Ok(file) => file,
				    };
			    	match file.read_to_string(&mut s) {
			        	Err(why) => return Response::not_found(),
			        	Ok(_) => (),
			    	} 
		   		} // file has been closed at this point
				entry.insert(s.clone());
				s
			}
		};


		let response = Response::ok();
		// let mut s = String::new();
		// {
		// 	let mut file = match File::open(self.url.as_path()) {
	 //        	Err(why) => return Response::not_found(), // can not find file
	 //        	Ok(file) => file,
		//     };
	 //    	match file.read_to_string(&mut s) {
	 //        	Err(why) => println!("couldn't read file: {}", why.description()),
	 //        	Ok(_) => (),
	 //    	} 
  //  		} // file has been closed at this point
		response.body(s)
	}
}

