use nix::sys::reboot::*;
use rocket::{get, launch, Config};
use std::{io::{stdin, stdout, Write}, net::Ipv4Addr};

#[macro_use] extern crate rocket;


fn get_input() {
    let mut s = String::new();
    print!("Please enter some text: ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    if s == "quit" {
        //reboot();
        match reboot(RebootMode::RB_POWER_OFF) {
            Err(e) => println!("error {}", e),
            _ => {}
        }
    } else if s == "hello there" {
        println!("General Kenobi!");
    } else {
        println!("You typed: {}", s);
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

#[launch]
fn rocket() -> _ {
    let config = rocket::Config {
        port: 80,
        address: Ipv4Addr::new(0,0,0,0).into(),
        ..Config::default()
    };
    let err = rocket::build()
        .configure(config)
        .mount("/", routes![index]);
    err
}
