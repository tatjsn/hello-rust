use askama::Template;
use crate::hello::HelloTemplate;

mod hello;

fn main() {
  let hello = HelloTemplate { name: "world" }; // instantiate your struct
  println!("{}", hello.render().unwrap()); // then render it.
}
