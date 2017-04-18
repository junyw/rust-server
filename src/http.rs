use std::collections::HashMap;
use std::io::{self, Read, Write, BufReader, BufRead};
use regex::Regex;

#[derive(Clone, Debug)]
enum Method {
	GET,
	POST,
	PUT,
	DELETE,
	NONE
}
pub struct Request{
	method: Method,
	uri: String,
	version: String,
	headers: HashMap<String, String>,
}

impl Request {
	pub fn new() -> io::Result<Request> {
		Ok(Request {
			method: Method::NONE,
			uri: String::new(),
			version: String::new(),
			headers: HashMap::new(),
		})
	}
	pub fn parse(&mut self, input: &str) {
		let mut v: Vec<&str> = input.split('\n').collect();
		v.reverse();
		match v.pop() {
			Some(text) => {
				let re = Regex::new(r"(\D+)\s(.+)\sHTTP/(.+)\r").unwrap();
				for cap in re.captures_iter(text) {
				    println!("Method: {} URI: {} Version: {}", &cap[1], &cap[2], &cap[3]);
				    match &cap[1] {
				    	"GET"  => self.method = Method::GET,
				    	"POST" => self.method = Method::POST,
				    	"PUT"  => self.method = Method::PUT,
				    	"DELETE" => self.method = Method::DELETE,
				    	_ => println!("unknown method {}", &cap[1]),
				    }
				    self.uri = String::from(&cap[2]);
				    self.version = String::from(&cap[3]);
				}
			}
			None => println!("invalid http header"),
		}
		let re = Regex::new(r"(.+):\s(.+)\r").unwrap();

		while let Some(text) = v.pop() {
			for cap in re.captures_iter(text) {
				self.headers.insert(cap[1].to_string(), cap[2].to_string());
		    	//println!("Name: {} Value: {} ", &cap[1], &cap[2]);
			}
		}
		println!("{:?}",self.method );
		println!("{:?}",self.uri );
		println!("{:?}",self.version );
		println!("{:?}",self.headers );


	}
}