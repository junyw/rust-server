use http::{Request, Response, Method};
use std::io::{self};
use std::option;
use regex::{Regex, RegexSet};
use std::collections::HashMap;

pub trait Action {
    fn render(&self) -> Response;
}

pub struct NotFound;
impl Action for NotFound {
	fn render(&self) -> Response {
		let mut response = Response::ok();
		response.body("<html><body> PAGE NOT FOUND </body></html>".to_string());
		response
	}
}

#[test]
fn it_works() {
	// let mut router = Router::new();
	// let mut router2 = router.get();
	// println!("{:?}", router2.views[0].render());
	let mut routerBuilder = RouterBuilder::new();
	let mut router = routerBuilder.get().build();
	println!("{:?}", router.views[0].render());
}
pub struct RouterBuilder {
	regexs: Vec<&'static str>,
	methods: Vec<Method>,
	views: Vec<Box<Action>>,
}
impl RouterBuilder {
	pub fn new() -> RouterBuilder {
		RouterBuilder {
			regexs: Vec::new(),
			methods: Vec::new(),
			views: Vec::new(),
		}
	}
	pub fn get(self) -> RouterBuilder {
		match self {
			RouterBuilder {regexs: mut r, methods: mut m, views: mut v} => {
				r.push(r"a");
				m.push(Method::GET);
				v.push(Box::new(NotFound));

				RouterBuilder {
					regexs: r,
					methods: m,
					views: v,
				}
			}
		}
	}
	pub fn build(self) -> Router {
		match self {
			RouterBuilder {regexs: mut r, methods: mut m, views: mut v} => {
				Router {
					regexs: RegexSet::new(r).unwrap(),
					methods: m,
					views: v,
				}
			}
		}
	}

}

pub struct Router  {
	//base: String,
	// routes: HashMap<String, Route<Action>>,
	regexs: RegexSet,
	methods: Vec<Method>,
	views: Vec<Box<Action>>,
}
impl Router {
	pub fn new() -> Router {
		Router {
			regexs: RegexSet::new(&[r"a",r"b"]).unwrap(),
			methods: vec![Method::GET, Method::GET],
			views: vec![Box::new(NotFound), Box::new(NotFound)],
		}
	}
}

// impl<A: Action> Router<A> {

// 	pub fn new(base: String) -> io::Result<Router<A>> {
// 		let mut defaults = HashMap::<String, A>::new();
// 		let mut not_found = NotFound;
// 		defaults.insert(String::from("404"), not_found);
// 		Ok(Router{
// 			base: base,
// 			routes: HashMap::<String, Route<A>>::new(),
// 			defaults: defaults,
// 		})
// 	}
// }

// 	pub fn serve(&mut self, req: &Request) -> Response {
// 		match self.route(req.method(), &req.uri()) {
// 			Some(r) => r.action.render(),
// 			None => {
// 				self.defaults.get("404").unwrap().render()
// 			}
// 		}
// 	}
// 	fn route(&self, method: Method, path: &str) -> Option<Route<A>> {
// 		for route in self.routes.values() {
// 			if route.method == method && route.reg.is_match(path) {
// 				Some(route);
// 			}
// 		}
// 		None
// 	}
// }


