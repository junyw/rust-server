use http::{Request, Response, Method};
use io::{self};

pub struct Route <A: Action> {
	method: Method,
	path: Matcher,
	action: 
}

impl<A: Action>  Route<A> {

}
pub struct Router {
	base: String,
	routes: Vec<Route>,
}

impl Router {
	pub fn new(base: String) -> io::Result<Router> {
		Ok(Router{
			base: base,
			handlers: vec![],
		})
	}
	pub fn serve(&mut self, mut req: Request) -> io::Result<Response> {
		
	}
}
