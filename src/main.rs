use crate::hello::HelloTemplate;
use askama::Template;

mod hello;

fn main() {
    let hello = HelloTemplate { name: "world" }; // instantiate your struct
    println!("{}", hello.render().unwrap()); // then render it.
}
