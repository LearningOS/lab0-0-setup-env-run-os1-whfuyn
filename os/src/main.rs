#![no_std]
#![no_main]
#![feature(format_args_nl)]

use os::*;

core::arch::global_asm!(
    include_str!("entry.asm")
);

#[no_mangle]
pub fn rust_main() -> ! {
    println!("Hello, world!");
    clear_bss();
    shutdown();
}
