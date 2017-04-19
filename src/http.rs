use std::collections::HashMap;
use std::io::{self, Read, Write, BufReader, BufRead};
use regex::Regex;
use chrono::prelude::*;
use server::Message;

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert(String::from($key), String::from($val)); )*
         map
    }}
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Method {
	GET,
	POST,
	PUT,
	DELETE,
	NONE,
}  
impl Method {
	pub fn to_str(self) -> &'static str {
		match self {
			Method::GET => "GET",
			Method::POST => "POST",
			Method::PUT => "PUT",
			Method::DELETE => "DELETE",
			Method::NONE => "NONE",
		}
	}
} 


#[derive(Debug)]
pub struct Request{
	method: Method,
	uri: String,
	version: String,
	fields: HashMap<String, String>,
}

impl Request {
	pub fn new() -> io::Result<Request> {
		Ok(Request {
			method: Method::NONE,
			uri: String::new(),
			version: String::new(),
			fields: HashMap::new(),
		})
	}
	pub fn method(&self) -> Method {
		self.method
	}
	pub fn uri(&self) -> String {
		self.uri.to_string()
	}
	pub fn parse(&mut self, input: &str) {
		let mut v: Vec<&str> = input.split('\n').collect();
		v.reverse();
		match v.pop() {
			Some(text) => {
				let re = Regex::new(r"(\D+)\s(.+)\s(HTTP/.+)\r").unwrap();
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
				self.fields.insert(cap[1].to_string(), cap[2].to_string());
		    	//println!("Name: {} Value: {} ", &cap[1], &cap[2]);
			}
		}
		println!("{:?}",self.method );
		println!("{:?}",self.uri );
		println!("{:?}",self.version );
		println!("{:?}",self.fields );
	}
}

#[derive(Debug)]
pub struct Response{
	version: String,
	status: String,
	fields: HashMap<String, String>,
	body: String,
}

impl Response {
	pub fn ok() -> Response {
		let dt = Local::now();

		Response {
			version: String::from("HTTP/1.1"),
			status:  String::from("200 OK"),
			fields: hashmap![ "Date"   => dt.format("%Y-%m-%d %H:%M:%S").to_string(), 
							  "Server" => "Rust-server/0.0.0", 
							  "Content-Length" => "0",
							  "Content-Type"   => "text/html",
							  "Connection"     => "Closed"],
			body: String::new(),
		}
	}
	pub fn body(&mut self, body: String) {
		self.fields.insert("Content-Length".to_string(), body.len().to_string());
		self.body = body;
	}
	pub fn to_string(&self) -> String {
		let mut s = String::new();
		s.push_str(&self.version);
		s.push_str(" ");
		s.push_str(&self.status);
		s.push_str("\n");
		for(name, value) in &self.fields {
			s.push_str(name);
			s.push_str(": ");
			s.push_str(value);
			s.push_str("\n");
		}
		s.push_str("\n");
		s.push_str(&self.body);
		s
	}
	pub fn to_message(&self) -> Message {
		let mut message = Message::new();
		message.write(self.to_string().as_bytes());
		message
	}
}




