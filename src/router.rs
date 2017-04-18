use http::{Request, Response, Method};
use std::io::{self};
use std::option;
use regex::Regex;
use std::collections::HashMap;

pub trait Action {
    fn render(&self) -> Response;
}

pub struct Not_Found;
impl Action for Not_Found {
	fn render(&self) -> Response {
		let mut response = Response::ok();
		response.body("<html><body> PAGE NOT FOUND </body></html>".to_string());
		response
	}
}

pub struct Route<A: Action> {
	method: Method,
	reg: Regex,
	action: A,
}
impl<A: Action> Route<A> {
	// pub fn new(action: A) -> Route<A> {
	// 	Route {
	// 		method: Method::NONE,
	// 	}
	// }
}

impl<A: Action> Route<A> {

}
pub struct Router<A: Action> {
	base: String,
	routes: HashMap<String, Route<A>>,
	defaults: HashMap<String, A>,
}

impl<A: Action> Router<A> {
	pub fn new(base: String) -> io::Result<Router<A>> {
		let mut defaults = HashMap::new();
		defaults.insert("404", Not_Found);
		Ok(Router{
			base: base,
			routes: HashMap::new(),
			defaults: HashMap::new(),
		})
	}
	pub fn serve(&mut self, mut req: Request) -> Response {
		match self.route(req.method(), &req.uri()) {
			Some(r) => r.action.render(),
			None => {
				self.defaults.get("404").unwrap().render()
			}
		}
	}
	fn route(&self, method: Method, path: &str) -> Option<Route<A>> {
		for route in self.routes.values() {
			if route.method == method && route.reg.is_match(path) {
				Some(route);
			}
		}
		None
	}
}


#[test]
fn it_works() {
	println!("Yes it works");
}

