#![no_main]
#![no_std]

use uefi::{prelude::*, println};

#[entry]
fn main(_image_handle: Handle, system_table: SystemTable<Boot>) -> Status {
    uefi::helpers::init().unwrap();

    let uefi_rev = system_table.uefi_revision();
    let firmware_vendor = system_table.firmware_vendor();

    println!("UEFI Version: {}.{}", uefi_rev.major(), uefi_rev.minor());
    println!("Firmware Vendor: {}", firmware_vendor);
    println!("Hello World!");

    loop {}
}
