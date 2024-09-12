#![no_main]
#![no_std]

use core::panic::PanicInfo;

mod frame;

#[no_mangle]
extern "sysv64" fn _start(fb_ptr: *mut u8, fb_size: usize) -> ! {
    for i in 0..fb_size {
        unsafe {
            *(fb_ptr.wrapping_add(i)) = (i % 256) as u8;
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
