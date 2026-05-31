#![no_main]
#![no_std]

use log::info;
use uefi::boot;
use uefi::mem::memory_map::{MemoryMap, MemoryType};
use uefi::prelude::*;

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    uefi::println!("hinode-efi");
    uefi::println!("A generic Rust-based UEFI project for AArch64 systems.");
    uefi::println!("Target: aarch64-unknown-uefi");
    uefi::println!();

    let vendor = uefi::system::firmware_vendor();
    let fw_rev = uefi::system::firmware_revision();
    let uefi_rev = uefi::system::uefi_revision();

    uefi::println!("Firmware vendor:   {}", vendor);
    uefi::println!("Firmware revision: {:#010x}", fw_rev);
    uefi::println!("UEFI revision:     {}", uefi_rev);
    uefi::println!();

    let mmap = boot::memory_map(MemoryType::LOADER_DATA).expect("memory map");
    let total_pages: u64 = mmap.entries().map(|e| e.page_count).sum();
    let total_mib = total_pages * 4096 / (1024 * 1024);

    uefi::println!("Memory map entries: {}", mmap.len());
    uefi::println!(
        "Total memory:       {} MiB ({} pages)",
        total_mib,
        total_pages
    );

    info!("hinode-efi: boot complete");

    Status::SUCCESS
}
