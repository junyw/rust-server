use std::str;
use std::os::unix::io::{RawFd, AsRawFd};
use std::collections::HashMap;
use std::net::{TcpListener, TcpStream, Shutdown};
use io::notification::{EventLoop, Handler, Identifier, Interest, EventSet};
use std::io::{self, Read, Write};
use ansi_term::Colour::*;
use service::Service;

pub struct Server<T: Service> {

  event_loop: EventLoop,
  dispatcher: Dispatcher<T>,
} 

impl<T: Service> Server<T> {
  pub fn new(tcp: TcpListener, service: T) -> Server<T> {
    Server {
      event_loop: EventLoop::new().unwrap(),
      dispatcher: Dispatcher::new(tcp, service),
    }
  }
  fn initialize(&mut self) {
    let identifier = Identifier::new(self.dispatcher.as_raw_fd(), Interest::Read);
    self.event_loop.register(&identifier);
    println!("{} {}", Green.bold().paint("Listening on"), self.dispatcher.listener.local_addr().unwrap());
  }
  pub fn run(&mut self) {
    self.initialize();
    loop {
      self.event_loop.run(&mut self.dispatcher);
    }
  }
}
pub struct Dispatcher<T: Service> {
  id: RawFd,
  listener: TcpListener,
  // server needs to maintain a list of accepted connections
  connections: HashMap<RawFd, Client>,
  service: T,
}

impl<T: Service>  Dispatcher<T> {
  pub fn new(tcp: TcpListener, service: T) -> Dispatcher<T> {
    Dispatcher {
      id: tcp.as_raw_fd(),
      listener: tcp,
      connections: HashMap::new(),
      service: service,
    }
  }
  pub fn as_raw_fd(&self) -> RawFd {
    self.id
  }

  // Accept a new client connection.
  fn accept(&mut self, event_loop: &mut EventLoop) {
    // Accept a new incoming connection from this listener with function accept(). 
    // fn accept(&self) -> Result<(TcpStream, SocketAddr)>
    //let stream = self.listener.accept().unwrap().0;
    match self.listener.accept() {
      Ok((socket, addr)) => {
        println!("{} {}", Green.bold().paint("Accept new connection"), addr);

        //println!("new client: {:?}", addr);
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
    //println!("receive from socket id={}", id);
    // to get the socket, use self.connections.get(&id).unwrap()

    //println!("socket {} has {} bytes.", id, ev_set.get_data());

    //message.print();
    //add message to client's send_queue
    if ev_set.is_readable() {
      let mut message: Message = self.connections.get_mut(&id).unwrap().get_message(&id, &(ev_set.get_data() as u32));
      
      // if a socket is readable but there is nothing, it means the connection is closed;
      if message.length() == 0 {
        let identifier = Identifier::new(id, Interest::Read);
        event_loop.deregister(&identifier);
        let identifier2 = Identifier::new(id, Interest::Write);
        event_loop.deregister(&identifier2);

        //close(id);
        {
          let client = self.connections.get_mut(&id).unwrap();
          client.shutdown();
          println!("{} {}", Green.bold().paint("Close connection"), client.socket.peer_addr().unwrap());
        }       
        self.connections.remove(&id);

      } else {
        let return_message = self.service.ready(message.clone());

        self.connections.get_mut(&id).unwrap().send_message(return_message);

        let identifier = Identifier::new(id, Interest::Read);
        event_loop.deregister(&identifier);

        let identifier2 = Identifier::new(id, Interest::Write);

        event_loop.register(&identifier2);
      }
    }


  }
}

impl<T: Service> Handler for Dispatcher<T> {
    fn ready(&mut self, id: RawFd, ev_set: EventSet, event_loop: &mut EventLoop) {
      // called by event_loop to handle a incoming request.
      //println!("Socket id={} is ready", id);
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
        //println!("socket {} is readable: {}", id, ev_set.is_readable());
        //println!("socket {} is writable: {}", id, ev_set.is_writable());
        if ev_set.is_readable()  {
          self.receive(id, ev_set, event_loop);
        } 
        else if ev_set.is_writable() {
          //println!("event {} is writable", id);

          self.connections.get_mut(&id).unwrap().write_message();
          // deregister from event_loop
          // TODO: also remove from connections?
          let identifier = Identifier::new(id, Interest::Write);
          let identifier2 = Identifier::new(id, Interest::Read);

          event_loop.deregister(&identifier);
          // keep reading from this socket;
          event_loop.register(&identifier2);

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
          self.socket.write_all(&buf[..]).unwrap();
          self.socket.flush();
        }
        None => (),
      }
      // let mut message: Message = self.send_queue.pop().unwrap();
      // let mut buf:Vec<u8> = message.buf;
      // self.socket.write(&buf[..]);
    }
    pub fn shutdown(&mut self) -> () {
      self.socket.shutdown(Shutdown::Both);
    }
}

#[derive(Clone)]
pub struct Message {
  buf: Vec<u8>,
}
impl Message {
  pub fn new() -> Message {
    Message {
      buf: Vec::new(),
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
      if count == 0 {break;}
    }     
  }
  pub fn to_str(&self) -> &str {
    let s = match str::from_utf8(&self.buf) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    s
  }

  
  pub fn print(&self) {
    println!("{}", self.to_str());
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
