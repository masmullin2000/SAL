#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::io::{stdin,stdout,Write};
use nix::sys::reboot::*;
use std::thread;

fn get_input() {
    let mut s = String::new();
    print!("Please enter some text: ");
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
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
		println!("You typed: {}",s);
	}

}

#[get("/")]
fn index() -> &'static str {
	"Hello, World!"
}

fn main() {
	thread::spawn(move || {
		let err = rocket::ignite()
			.mount("/", routes![index])
			.launch();
		println!("error {}", err);
	});

	//println!("{:?}", err);

	loop {
    	get_input();
    }

}