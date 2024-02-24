//! Probably need replacing Linefeed with something better.

use linefeed::Interface;
use linefeed::ReadResult;

pub fn run() {
  let reader = Interface::new("Neon").unwrap();
  reader.set_prompt(">").unwrap();

  while let ReadResult::Input(input) = reader.read_line().unwrap() {
    println!("got input {:?}", input);
  }

  println!("Goodbye.");
}
