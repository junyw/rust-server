# carbon is a light weight asynchronous web server 

## Idea
At its core, carbon utilize 
[kqueue() system call](https://www.freebsd.org/cgi/man.cgi?query=kqueue&sektion=2) 
from BSD kernel. Kqueue() system call provides a non-blocking channel for user 
to be notified when any of the events in the k-queue happens or holds a condition. 

## Implementation

Architecture overview
```$xslt 
extern crate nix;
extern crate ansi_term;
extern crate regex;
extern crate chrono;
extern crate fnv;

pub mod io; 
pub mod http;
pub mod server;
pub mod service;
pub mod router;
pub mod view;
```

### Web server design

### Rust as a language

#### Ownership

#### Lifetime

#### Mutability

## Performance

## Limitations

### Linux support to be implemented

## Resources
