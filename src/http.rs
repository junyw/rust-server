use std::collections::HashMap;
use std::io::{self, Write};
use regex::Regex;
use chrono::prelude::*;
use server::Message;
use httparse;
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
	version: u8,
	//fields: HashMap<String, String>,
}

impl Request {
	pub fn parse(message: & Message) -> Request {
		let mut headers = [httparse::EMPTY_HEADER; 16];
		let mut req = httparse::Request::new(&mut headers);
		let res = req.parse(&message.buf).unwrap();
		Request {
			method: match req.method.expect("error parsing request") {
				"GET"    => Method::GET,
				"PUT"    => Method::PUT,
				"POST"   => Method::POST,
				"DELETE" => Method::DELETE,
				_ => Method::NONE,
			},
			uri: String::from(req.path.expect("error parsing request")),
			version: req.version.expect("error parsing request"),
		}
	}
	pub fn method(&self) -> Method {
		self.method
	}
	pub fn uri(&self) -> String {
		self.uri.to_string()
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
	fn new() -> Response {
		let dt = Local::now();
		Response {
			version: String::from("HTTP/1.1"),
			status:  String::new(),
			fields: hashmap![ "Date"   => dt.format("%Y-%m-%d %H:%M:%S").to_string(), 
							  "Server" => "Rust-server/0.0.0", 
							  "Content-Length" => "0",
							  "Content-Type"   => "text/html"],
			body: String::new(),
		}
	}
	pub fn ok() -> Response {
		let mut r = Response::new();
		r.status = String::from("200 OK");
		r

	}
	pub fn not_found() -> Response {
		let mut r = Response::new();
		r.status = String::from("404 NOT FOUND");
		r
	}
	pub fn server_error() -> Response {
		let mut r = Response::new();
		r.status = String::from("500 Internal Server Error");
		r
	}
	pub fn body(self, body: String) -> Response {
		match self {
			Response {version: v, status: s, fields:mut f, ..} => {
				f.insert("Content-Length".to_string(), body.len().to_string());
				Response {
					version: v,
					status:  s,
					fields: f,
					body: body,
				}
			}
		}
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
		message.write(self.to_string().as_bytes()).expect("response to message error");
		message
	}
}




