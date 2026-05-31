#![no_main]
#![no_std]

use log::info;
use uefi::prelude::*;

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    uefi::println!("hinode-efi");
    uefi::println!("A generic Rust-based UEFI project for AArch64 systems.");
    uefi::println!("Target: aarch64-unknown-uefi");
    uefi::println!("Status: sunrise");

    info!("hinode-efi initialized");

    Status::SUCCESS
}
