#![no_std]
#![no_main]
#![feature(format_args_nl)]

use os::{
    sys_exit, println,
};

#[no_mangle]
extern "C" fn _start() {
    println!("Hello, world!");
    sys_exit(9);
}
