#![no_std]

use uefi::proto::console::gop::PixelFormat;

#[repr(C)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

impl From<(u8, u8, u8)> for Pixel {
    fn from(value: (u8, u8, u8)) -> Self {
        Self {
            r: value.0,
            g: value.1,
            b: value.2,
        }
    }
}

#[repr(C)]
pub struct DisplayInfo {
    pub width: usize,
    pub height: usize,
    pub stride: usize,
    pub format: PixelFormat,
}

#[repr(C)]
pub struct Display {
    fb_ptr: *mut u8,
    fb_size: usize,
    info: DisplayInfo,
}

impl Display {
    pub fn new(fb_ptr: *mut u8, fb_size: usize, info: DisplayInfo) -> Self {
        Self {
            fb_ptr,
            fb_size,
            info,
        }
    }
    pub fn resolution(&self) -> (usize, usize) {
        (self.info.width, self.info.height)
    }
    pub fn write_pixel(&mut self, x: usize, y: usize, pixel: impl Into<Pixel>) {
        if x > self.info.width {
            return;
        }
        if y > self.info.height {
            return;
        }

        match self.info.format {
            PixelFormat::Rgb => {
                let index = self.info.stride * y * 4 + x * 4;
                let pixel = pixel.into();
                unsafe {
                    *(self.fb_ptr.add(index)) = pixel.r;
                    *(self.fb_ptr.add(index + 1)) = pixel.g;
                    *(self.fb_ptr.add(index + 2)) = pixel.b
                }
            }
            PixelFormat::Bgr => {
                let index = self.info.stride * y * 4 + x * 4;
                let pixel = pixel.into();
                unsafe {
                    *(self.fb_ptr.add(index)) = pixel.b;
                    *(self.fb_ptr.add(index + 1)) = pixel.g;
                    *(self.fb_ptr.add(index + 2)) = pixel.r
                }
            }
            PixelFormat::Bitmask => {}
            PixelFormat::BltOnly => {}
        }
    }
}
