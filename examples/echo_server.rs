extern crate tinyIO;
use tinyIO::io::notification::{EventLoop, Handler, Identifier, Interest, EventSet};

use std::os::unix::io::{RawFd, AsRawFd};
// growable array
use std::vec::Vec;
//IO of the stream
use std::io::{self, Read, Write, BufReader, BufRead};

use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
fn main() {
  
  let mut event_loop : EventLoop= EventLoop::new().unwrap();
  let listener = TcpListener::bind("127.0.0.1:1300").unwrap();
  let identifier = Identifier::new(listener.as_raw_fd(), Interest::Read);
  //let mut tcp_event = Event::new_tcp_event(&listener);
  event_loop.register(&identifier);

  let mut server = Server::new(listener);
  loop {
    //println!("polling..");
    event_loop.run(&mut server);

  }
}
pub struct Server {
  id: RawFd,
  listener: TcpListener,
  // server needs to maintain a list of accepted connections
  connections: HashMap<RawFd, Client>,
}
impl Server {
  fn new(tcp: TcpListener) -> Server {
    Server {
      id: tcp.as_raw_fd(),
      listener: tcp,
      connections: HashMap::new(),
    }
  }

  // Accept a new client connection.
  fn accept(&mut self, event_loop: &mut EventLoop) {
    println!("accept new connection.");
    // Accept a new incoming connection from this listener with function accept(). 
    // fn accept(&self) -> Result<(TcpStream, SocketAddr)>
    //let stream = self.listener.accept().unwrap().0;
    match self.listener.accept() {
      Ok((socket, addr)) => {
        println!("new client: {:?}", addr);
        //let mut tcp_stream = Event::new_tcp_stream(&socket);
        let identifier = Identifier::new(socket.as_raw_fd(), Interest::Read);

        event_loop.register(&identifier);
        let client = Client::new(socket.as_raw_fd(), socket);
        //self.handle_request(&socket);
        self.connections.insert(client.as_raw_fd(), client);
      },
      Err(e) => println!("couldn't get client: {:?}", e),
    }


  }
  fn receive(&mut self, id: RawFd, ev_set: EventSet, event_loop: &mut EventLoop) {
    println!("receive from socket id={}", id);
    let mut buffer = vec![0; 20];
    // to get the socket, use self.connections.get(&id).unwrap()

    println!("socket {} has {} bytes.", id, ev_set.get_data());
    // read from the socket to buffer
    // if(event.is_readable()) {
    //   match self.connections.get(&id).unwrap().read(&mut buffer[..]) {
    //     Ok(0) => println!("no data being read."),
    //     Ok(n) => {
    //       println!("read {} bytes", n);
    //     }
    //     Err(e) => panic!("{:?}", e),
    //   }
    // }

    //message.print();
    //add message to client's send_queue
    if(ev_set.is_readable()) {
      let mut message: Message = self.connections.get_mut(&id).unwrap().get_message(&id, &(ev_set.get_data() as u32));

      self.connections.get_mut(&id).unwrap().send_message(message);

      let identifier = Identifier::new(id, Interest::Read);
      event_loop.deregister(&identifier);

      let identifier2 = Identifier::new(id, Interest::Write);

      event_loop.register(&identifier2);
    }


  }
}
impl Handler for Server {
    fn ready(&mut self, id: RawFd, ev_set: EventSet, event_loop: &mut EventLoop) {
      // called by event_loop to handle a incoming request.
      println!("Socket id={} is ready", id);
      // if events.is_error() {
      //       return;
      // }
      // if events.is_hup() {
      //       return;
      // }
      // if events.is_writable() {
      //     //Do something.
      // }

      // if the TcpListener is ready, call accept to establish a new TcpStream
      // else, a TcpStream is ready.
      if self.id == id {
        self.accept(event_loop);
      } else {
        println!("socket {} is readable: {}", id, ev_set.is_readable());
        println!("socket {} is writable: {}", id, ev_set.is_writable());
        if(ev_set.is_readable()) {
          self.receive(id, ev_set, event_loop);
        } 
        else if (ev_set.is_writable()) {
          println!("event {} is writable", id);

          self.connections.get_mut(&id).unwrap().write_message();
          // deregister from event_loop
          // TODO: also remove from connections
          let identifier = Identifier::new(id, Interest::Write);

          event_loop.deregister(&identifier);

        } 
      }
    }
}
// TODO: reimplement connections;
// TODO: let connections manage reading message and writing message
struct Client {
    id: RawFd,
    socket: TcpStream,
    send_queue: Vec<Message>,
}

impl Client {
    pub fn new(id: RawFd, sock: TcpStream) -> Client {
        Client {
            id: id,
            socket: sock,
            send_queue: Vec::with_capacity(1024),
        }
    }
    pub fn as_raw_fd(&self) -> RawFd {
      self.socket.as_raw_fd()
    }
    pub fn get_message(&mut self, id: &RawFd, len: &u32) -> Message {
      let mut buffer = vec![0; *len as usize];
      self.socket.read(&mut buffer[..]);
      Message {
        buf: buffer,
      }   
    }
    pub fn send_message(&mut self, message: Message) -> () {
      self.send_queue.push(message);
    }
    pub fn write_message(&mut self) -> () {
      match self.send_queue.pop() {
        Some(message) => {
          let mut buf:Vec<u8> = message.buf;
          self.socket.write(&buf[..]);
        }
        None => (),
      }
      // let mut message: Message = self.send_queue.pop().unwrap();
      // let mut buf:Vec<u8> = message.buf;
      // self.socket.write(&buf[..]);
    }
}
struct Message {
  buf: Vec<u8>,
}
impl Message {
  pub fn new() -> Message {
    Message {
      buf: Vec::with_capacity(1024),
    }
  }

  pub fn length(&self) -> usize {
    self.buf.len()
  }
  pub fn from_sock(&mut self, sock: &mut TcpStream, len: u32) {
    let mut count = len.clone();
    for byte in sock.bytes() {

      count -= 1;
      
      match byte {
        Ok(c) => {
          //println!("{}", c);
          self.buf.push(c);
        }
        _ => break,
      }
      if(count == 0) {break;}
    }     
  }
  pub fn print(&self) {
    println!("message print...");
    println!("{:?}", self.buf);
  }
}


impl Write for Message {
   fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
      self.buf.write(buf) 
   }
   fn flush(&mut self) -> io::Result<()> {
      self.buf.flush()
   }
}

