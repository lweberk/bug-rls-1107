//!
//! Trying to reproduce crash for (https://github.com/rust-lang-nursery/rls/issues/1107)
//!
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String
{
    format!("Hello, {} year old named {}!", age, name)
}


fn main()
{
    rocket::ignite().mount("/", routes![hello]).launch();
}
