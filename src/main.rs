//extern crate nix;

//use nix::sys::event::{KEvent, kqueue, kevent, EventFilter, FilterFlag};
//use nix::sys::event::{EV_ADD, EV_ENABLE};

use std::os::unix::io::AsRawFd;
use std::os::unix::io::RawFd;

//IO of the stream
use std::io::{self, Read, Write, BufReader, BufRead};

// hashmap
use std::collections::HashMap;

// sockets
use std::net::{TcpListener, TcpStream};
extern crate tinyIO;
//use tinyIO::tcpEvent;
use tinyIO::io::{EventLoop, Event, Handler};
fn main() {
  
  let mut event_loop : EventLoop= EventLoop::new().unwrap();
  let listener = TcpListener::bind("127.0.0.1:1300").unwrap();

  //let mut tcp_event = Event::new_tcp_event(&listener);
  event_loop.register_socket(& listener);

  let mut server = Server::new(listener);
  loop {
    //println!("polling..");
    event_loop.run(&mut server);
    io::stdout().flush().unwrap();

  }
}
pub struct Server {
  id: RawFd,
  listener: TcpListener,
  // server needs to maintain a list of accepted connections
  connections: HashMap<RawFd, TcpStream>,
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
        event_loop.register_socket(&socket);

        //self.handle_request(&socket);
        self.connections.insert(socket.as_raw_fd(), socket);
      },
      Err(e) => println!("couldn't get client: {:?}", e),
    }


  }
  fn receive(&mut self, id: RawFd, event: Event, event_loop: &mut EventLoop) {
    println!("receive from socket id={}", id);
    let mut buffer = vec![0; 20];
    // to get the socket, use self.connections.get(&id).unwrap()

    println!("socket {} has {} bytes.", id, event.get_data());
    // read from the socket to buffer
    if(event.is_readable()) {
      match self.connections.get(&id).unwrap().read(&mut buffer[..]) {
        Ok(0) => println!("no data being read."),
        Ok(n) => println!("read {} bytes", n),
        Err(e) => panic!("{:?}", e),
      }
    }

    // deregister from event_loop
    // also remove from connections
    // event_loop.deregister_socket(self.connections.get_mut(&id).unwrap());

  }
  fn handle_request(&self, stream: &TcpStream) {
    let mut reader = BufReader::new(stream);

    for line in reader.by_ref().lines() {
      let one_line = line.unwrap();
      println!("{}", one_line);
      if one_line == "" {
            break;
      }
    }
    self.send_response(reader.into_inner());
  }

  fn send_response(&self, mut stream: &TcpStream) {
      // Write the header and the html body
      let response = "HTTP/1.1 200 OK\n\n<html><body>Hello, World!</body></html>";
      stream.write_all(response.as_bytes()).unwrap();
  }
}
impl Handler for Server {
    fn ready(&mut self, id: RawFd, event: Event, event_loop: &mut EventLoop) {
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
        println!("socket {} is readable: {}", id, event.is_readable());
        println!("socket {} is writable: {}", id, event.is_writable());
        if(event.is_readable()) {
          self.receive(id, event, event_loop);
        } 
        else if (event.is_writable()) {
          println!("event {} is writable", id);
        } 
      }
    }
}

struct Connection {
    socket: TcpStream,
    id: RawFd,
    //buffer,
}

impl Connection {
    fn new(sock: TcpStream, id: RawFd) -> Connection {
        Connection {
            id: id,
            socket: sock,
        }
    }

    fn readable(&mut self) {
      // the connection is  EV_CLEAR and EV_ONESHOT, so we must drain
      // the entire socket receive buffer, otherwise the server will hang.
      // let mut recv_buf = ByteBuf::mut_with_capacity(2048);
      // loop {
      //     match self.sock.try_read_buf(&mut recv_buf) {
      //         // the socket receive buffer is empty, so let's move on
      //         // try_read_buf internally handles WouldBlock here too
      //         Ok(None) => {
      //             println!("connection : read 0 bytes");
      //             break;
      //         },
      //         Ok(Some(n)) => {
      //             println!("connection : we read {} bytes", n);


      //             if n < recv_buf.capacity() {
      //                 break;
      //             }
      //         },
      //         Err(e) => {
      //             error!("Failed to read buffer for id {}, error: {}", self.id, e);
      //             return Err(e);
      //         }
      //     }
      // }
    }
    fn writable(&mut self) {
      //TODO: implement me
    }
}
