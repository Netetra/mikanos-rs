#![no_main]
#![no_std]

extern crate alloc;

use core::{
    cmp::{max, min},
    mem,
};

use alloc::{vec, vec::Vec};
use goblin::elf::{self, Elf};
use uefi::{
    boot::{AllocateType, MemoryType},
    prelude::*,
    println,
    proto::{
        console::gop::GraphicsOutput,
        media::file::{Directory, File, FileAttribute, FileInfo, FileMode},
    },
    table::boot::PAGE_SIZE,
    CStr16,
};

#[entry]
fn main(_image_handle: Handle, system_table: SystemTable<Boot>) -> Status {
    uefi::helpers::init().unwrap();

    let boot_services = system_table.boot_services();

    println!("Hello UEFI!");

    let uefi_rev = system_table.uefi_revision();
    let firmware_vendor = system_table.firmware_vendor();
    println!("UEFI Version: {}.{}", uefi_rev.major(), uefi_rev.minor());
    println!("Firmware Vendor: {}", firmware_vendor);

    let buf = open_file(
        boot_services,
        cstr16!("\\kernel.elf"),
        FileMode::Read,
        FileAttribute::READ_ONLY,
    )
    .unwrap();
    let elf = Elf::parse(&buf).unwrap();
    println!("Entry point: 0x{:x}", elf.entry);
    let _ = load_elf(boot_services, &buf, &elf);
    println!("Loaded kernel");

    let (fb_ptr, fb_size) = get_frame_buffer_ptr(boot_services).unwrap();

    // Exit boot service
    let (_system_table, _memory_map) =
        unsafe { system_table.exit_boot_services(MemoryType::LOADER_DATA) };

    // Execute Kernel
    let entry_point: extern "sysv64" fn(*mut u8, usize) = unsafe { mem::transmute(elf.entry) };
    entry_point(fb_ptr, fb_size);

    loop {}

    #[allow(unreachable_code)]
    Status::SUCCESS
}

fn load_elf(boot_services: &BootServices, buff: &[u8], elf: &Elf) -> Result<(), uefi::Error> {
    let (first, last) = calc_load_addr_range(elf);
    let pages = (last - first + PAGE_SIZE - 1) / PAGE_SIZE;
    let _ = alloc_pages(boot_services, first, pages)?;
    copy_load_segments(buff, elf);
    Ok(())
}

fn calc_load_addr_range(elf: &Elf) -> (usize, usize) {
    let mut first = usize::MAX;
    let mut last = usize::MIN;

    for ph in elf.program_headers.iter() {
        if ph.p_type != elf::program_header::PT_LOAD {
            continue;
        }
        first = min(first, ph.p_vaddr as usize);
        last = max(last, (ph.p_vaddr + ph.p_memsz) as usize);
    }

    (first, last)
}

fn copy_load_segments(buff: &[u8], elf: &Elf) {
    for ph in elf.program_headers.iter() {
        if ph.p_type != elf::program_header::PT_LOAD {
            continue;
        }
        let offset = ph.p_offset as usize;
        let filesz = ph.p_filesz as usize;
        let memsz = ph.p_memsz as usize;
        let dest = unsafe { core::slice::from_raw_parts_mut(ph.p_vaddr as *mut u8, memsz) };
        dest[..filesz].copy_from_slice(&buff[offset..(offset + filesz)]);
        dest[filesz..].fill(0);
    }
}

fn get_frame_buffer_ptr(boot_services: &BootServices) -> Result<(*mut u8, usize), uefi::Error> {
    let handle = boot_services.get_handle_for_protocol::<GraphicsOutput>()?;
    let mut gop = boot_services.open_protocol_exclusive::<GraphicsOutput>(handle)?;
    let mut fb = gop.frame_buffer();
    let fb_ptr = fb.as_mut_ptr();
    let fb_size = fb.size();

    Ok((fb_ptr, fb_size))
}

fn open_file(
    boot_services: &BootServices,
    path: &CStr16,
    mode: FileMode,
    attr: FileAttribute,
) -> Result<Vec<u8>, uefi::Error> {
    let mut root = open_root(boot_services)?;
    let handle = root.open(path, mode, attr)?;
    let mut file = match handle.into_regular_file() {
        Some(f) => Ok(f),
        None => Err(uefi::Error::new(Status::NOT_FOUND, ())),
    }?;
    let size = file.get_boxed_info::<FileInfo>()?.file_size();
    let mut buf = vec![0u8; size as usize];
    file.read(&mut buf)?;
    Ok(buf)
}

fn open_root(boot_services: &BootServices) -> Result<Directory, uefi::Error> {
    let handle = boot_services.image_handle();
    let mut sfs = boot_services.get_image_file_system(handle)?;
    let directory = sfs.open_volume()?;
    Ok(directory)
}

#[inline]
fn alloc_pages(
    boot_services: &BootServices,
    address: usize,
    pages: usize,
) -> Result<usize, uefi::Error> {
    let addr = boot_services.allocate_pages(
        AllocateType::Address(address as u64),
        MemoryType::LOADER_DATA,
        pages,
    )?;
    Ok(addr as usize)
}
