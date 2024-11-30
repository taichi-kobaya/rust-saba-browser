#![no_std]
#![cfg_attr(not(target_os = "macos"), no_main)]

use noli::prelude::*;
use noli::println;
use noli::entry_point;
use noli::sys::wasabi::Api;

fn main() {
    Api::write_string("Hello World\n");
    println!("Hello from println!");
    Api::exit(42);
}

entry_point!(main);
