#![no_main]
#![no_std]

use core::{f32, panic::PanicInfo};
use micromath::F32Ext;
use util::Display;

mod frame;

#[no_mangle]
extern "sysv64" fn _start(mut display: Display) -> ! {
    let (width, height) = display.resolution();
    for y in 0..height {
        for x in 0..width {
            display.write_pixel(x, y, (0, 0, 0));
        }
    }
    let r: f32 = 250.;
    for degree in 0..360 {
        let (y, x) = (f32::consts::PI / 180. * degree as f32).sin_cos();
        display.write_pixel(
            ((r * x) + width as f32 / 2.) as usize,
            ((r * y) + height as f32 / 2.) as usize,
            (255, 255, 255),
        );
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
