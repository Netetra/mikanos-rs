#![no_main]
#![no_std]

use core::fmt::Write;

use uefi::prelude::*;

#[entry]
fn main(_image_handle: Handle, system_table: SystemTable<Boot>) -> Status {
    let mut system_table = system_table;
    system_table.stdout().write_str("Hello World!");
    loop {}
    Status::SUCCESS
}
