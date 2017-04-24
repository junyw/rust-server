# carbon is a light weight asynchronous web server 

## Idea
At its core, carbon utilize 
[kqueue() system call](https://www.freebsd.org/cgi/man.cgi?query=kqueue&sektion=2) 
from BSD kernel. Kqueue() system call provides a non-blocking channel for user 
to be notified when any of the events in the k-queue happens or holds a condition. 

## Implementation

Architecture overview

```aidl
'' extern crate nix;
'' extern crate ansi_term;
'' extern crate regex;
'' extern crate chrono;
'' extern crate fnv;
'' 
'' pub mod io; 
'' pub mod http;
'' pub mod server;
'' pub mod service;
'' pub mod router;
'' pub mod view;

```

![img](assets/app_layout.png)

Figure 1 is an overview of the Carbon, the main components are: 
- IO
- Router
- Server
- Service
- View

### IO

`IO` implements two `modulos`: `Event` and `Notification`.

#### Event is a vehicle for kevents 

```aidl
'' use nix::sys::event::{KEvent, EventFilter, FilterFlag};
'' use nix::sys::event::{EV_ADD, EV_ENABLE, EV_DELETE, EV_ERROR};
'' use std::os::unix::io::RawFd;
'' 
'' use io::notification::Identifier;
'' pub struct Event {
'' 	pub kevent: KEvent,
'' }
'' impl Event {
'' 	pub fn new(id: &Identifier) -> Event {} 
'' 	pub fn new_from_kevent(kevent: KEvent) -> Event {}
'' 	pub fn get_data(&self) -> u32 {}
'' 	pub fn is_readable(&self) -> bool {}		
'' 	pub fn is_writable(&self) -> bool {}
'' 	pub fn is_error(&self) -> bool {}
'' 	pub fn is_hup(&self)  {}
'' 	pub fn ev_set_add(&mut self) {}
'' 	pub fn ev_set_write(&mut self) {}
'' 	pub fn ev_set_delete(&mut self) {}
'' 	fn new_kevent(id: & RawFd) -> KEvent {}
'' 	pub fn new_timer_event(id: usize, timer: isize) -> Event {}
'' }

```

#### Notification provide an `EventLoop` and `Handler` to produce `Events` and perform Asynchronous request/response

```aidl
'' pub trait Handler {
''     fn ready(&mut self, id:RawFd, ev_set : EventSet, event_loop : &mut EventLoop);
'' }
'' 
'' pub struct EventLoop {
'' 	kqueue: RawFd,
'' 	// ev_list is used for retrival
'' 	ev_list: Vec<KEvent>,
'' }
'' impl EventLoop {
'' 	pub fn new() -> io::Result<EventLoop> {}
'' 	fn ev_register(&self, event: Event) {}
'' 	pub fn register(&self, id: &Identifier) {}
'' 	pub fn reregister() {}
'' 	pub fn deregister(&self, id: &Identifier) {}
'' 	pub fn run<H: Handler>(&mut self, handler: &mut H) {}
'' 	fn poll<H: Handler>(&mut self, handler: &mut H) {}
'' }
'' 
'' pub struct Identifier {
'' 	fd: RawFd,
'' 	filter: Interest,
'' }
'' impl Identifier {
'' 	pub fn new(fd: RawFd, interest: Interest) -> Identifier{}
'' 	pub fn get_fd(&self) -> RawFd {}
'' 	pub fn readable(&self) -> bool {}
'' 	pub fn writable(&self) -> bool {}
'' 
'' pub enum Interest {
''     Read,
''     Write,
'' }
'' pub struct EventSet(usize, usize);
'' impl EventSet {
'' 	pub fn new() -> EventSet {}
'' 	pub fn readable() -> EventSet {}
'' 	pub fn writable() -> EventSet {}
'' 	pub fn set_data(&mut self, data :usize) {}
'' 	pub fn get_data(&self) -> usize {}
'' 	pub fn is_readable(&self) -> bool {}
'' 	pub fn is_writable(&self) -> bool {}
'' }
''  

```


### Router provides rules for handle request and query resources

```aidl
'' pub struct RouterBuilder {
'' 	regexs: Vec<&'static str>,
'' 	methods: Vec<Method>,
'' 	views: Vec<Box<View>>,
'' }
'' impl RouterBuilder {
'' 	pub fn new() -> RouterBuilder {}
'' 	fn rule(self, method: Method, uri: &'static str, view: Box<View>) -> RouterBuilder {}
'' 	pub fn get(self, uri: &'static str, view: Box<View>) -> RouterBuilder {}
'' 	pub fn post(self, uri: &'static str, view: Box<View>) -> RouterBuilder {}
'' 	pub fn build(self) -> Router {}
'' }
'' 
'' pub struct Router  {
'' 	regexs: RegexSet,
'' 	methods: Vec<Method>,
'' 	views: Vec<Box<View>>,
'' 	cache: FnvHashMap<String, String>,
'' }
'' impl Router {
'' 	pub fn response(&mut self, method: Method, path: &str) -> Response {
'' 		}
'' 	}
'' 	fn route(&self, method: Method, path: &str) -> Option<usize> {}
'' }


```
### Service provides a bootstrap `trait` `Service`

```aidl
'' pub trait Service {}
'' 

```

### View loads resources

```aidl
'' pub trait View {
''     fn render(&self, cache: &mut FnvHashMap<String, String>) -> Response;
'' }
'' 
'' pub struct NotFound;
'' impl View for NotFound {
'' 	fn render(&self, cache: &mut FnvHashMap<String, String>) -> Response {}
'' }
'' pub struct Page {
'' 	url: PathBuf,
'' }
'' impl Page {
'' 	pub fn new(path: &'static str) -> Page {}
'' }
'' impl View for Page {
'' 	fn render(&self, cache: &mut FnvHashMap<String, String>) -> Response {}
'' }

```

### Server binds connection ports and handles request

```aidl
'' pub struct Server<T: Service> {
''   event_loop: EventLoop,
''   dispatcher: Dispatcher<T>,
'' } 
'' 
'' impl<T: Service> Server<T> {
''   pub fn new(tcp: TcpListener, service: T) -> Server<T> {
''     Server {}
''   }
''   fn initialize(&mut self) {}
''   pub fn run(&mut self) {}
'' }
'' pub struct Dispatcher<T: Service> {
''   id: RawFd,
''   listener: TcpListener,
''   // callbacks?
''   connections: FnvHashMap<RawFd, Client>,  // server needs to maintain a list of accepted connections
''   service: T,
'' }
'' 
'' impl<T: Service>  Dispatcher<T> {
''   pub fn new(tcp: TcpListener, service: T) -> Dispatcher<T> {}
''   pub fn as_raw_fd(&self) -> RawFd {}
'' 
''   // Accept a new client connection.
''   fn accept(&mut self, event_loop: &mut EventLoop) {}
''   fn receive(&mut self, id: RawFd, ev_set: EventSet, event_loop: &mut EventLoop) {}
'' }
'' 
'' impl<T: Service> Handler for Dispatcher<T> {
''     fn ready(&mut self, id: RawFd, ev_set: EventSet, event_loop: &mut EventLoop) {}
'' }
'' struct Client {
''     socket: TcpStream,
''     send_queue: Vec<Message>,
'' }
'' 
'' impl Client {
''     pub fn new(sock: TcpStream) -> Client {}
''     pub fn peer_addr(&self) -> Option<SocketAddr> {}
''     pub fn as_raw_fd(&self) -> RawFd {}
''     pub fn get_message(&mut self, len: &u32) -> Message {}
''     pub fn send_message(&mut self, message: Message) -> () {}
''     pub fn write_message(&mut self) -> () {}
'' 
'' #[derive(Clone, Debug)]
'' pub struct Message {
''   pub buf: Vec<u8>,
'' }
'' impl Message {
''   pub fn new() -> Message {}
'' 
''   pub fn length(&self) -> usize {}
''   pub fn from_sock(&mut self, sock: &mut TcpStream, len: u32) {}
''   pub fn to_str(&self) -> &str {}
''   pub fn print(&self) {}
'' }
'' 
'' impl Write for Message {
''    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {}
''    fn flush(&mut self) -> io::Result<()> {}
'' }
'' 

```
## Performance
node.js

Running 5s test @ http://127.0.0.1:8000/
  2 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    10.74ms    3.29ms  74.18ms   97.53%
    Req/Sec     9.35k     1.26k   10.03k    95.00%
  93035 requests in 5.01s, 13.84MB read
  Socket errors: connect 0, read 131, write 0, timeout 0
Requests/sec:  18580.64
Transfer/sec:      2.76MB

## Limitations

* Exceptional speed in crude status of a server is only a start
* `kqueue` is not portable
* Support for databases needs to be implemented

### Linux support to be implemented

## Resources
