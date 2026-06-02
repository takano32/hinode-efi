#![no_main]
#![no_std]

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use core::ffi::c_void;
use core::mem::size_of_val;
use core::ptr;

use log::info;
use uefi::boot::{self, LoadImageSource};
use uefi::prelude::*;
use uefi::proto::device_path::text::DevicePathFromText;
use uefi::proto::device_path::util::DevicePathUtilities;
use uefi::proto::device_path::DevicePath;
use uefi::proto::loaded_image::LoadedImage;
use uefi::proto::media::file::{File, FileAttribute, FileInfo, FileMode};
use uefi::proto::media::load_file::LoadFile2;
use uefi::proto::BootPolicy;
use uefi::{Handle, Identify};
use uefi_raw::protocol::device_path::DevicePathProtocol;
use uefi_raw::protocol::media::LoadFile2Protocol;
use uefi_raw::{Boolean, Status as RawStatus};

static mut INITRD_PTR: *const u8 = ptr::null();
static mut INITRD_LEN: usize = 0;

static mut INITRD_LOAD_FILE2: LoadFile2Protocol = LoadFile2Protocol {
    load_file: load_initrd,
};

// VenMedia(5568e427-68fc-4f3d-ac74-ca555231cc68)/EndEntire
static INITRD_DEVICE_PATH: [u8; 24] = [
    0x04, 0x03, 0x14, 0x00, 0x27, 0xe4, 0x68, 0x55, 0xfc, 0x68, 0x3d, 0x4f, 0xac, 0x74, 0xca, 0x55,
    0x52, 0x31, 0xcc, 0x68, 0x7f, 0xff, 0x04, 0x00,
];

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    uefi::println!("hinode-efi");
    uefi::println!("Booting Ubuntu 26.04 Live Server arm64.");
    uefi::println!("Target: aarch64-unknown-uefi");
    uefi::println!();

    match boot_ubuntu_live_server() {
        Ok(()) => Status::SUCCESS,
        Err(err) => {
            uefi::println!("boot failed: {:?}", err.status());
            err.status()
        }
    }
}

fn boot_ubuntu_live_server() -> uefi::Result {
    let image_handle = boot::image_handle();

    install_initrd_load_file2(image_handle)?;

    let loaded_image = boot::open_protocol_exclusive::<LoadedImage>(image_handle)?;
    let device_handle = loaded_image.device().ok_or(uefi::Status::UNSUPPORTED)?;
    drop(loaded_image);

    let boot_device_path = boot::open_protocol_exclusive::<DevicePath>(device_handle)?;
    let from_text_handle = boot::get_handle_for_protocol::<DevicePathFromText>()?;
    let from_text = boot::open_protocol_exclusive::<DevicePathFromText>(from_text_handle)?;
    let utilities_handle = boot::get_handle_for_protocol::<DevicePathUtilities>()?;
    let utilities = boot::open_protocol_exclusive::<DevicePathUtilities>(utilities_handle)?;

    let kernel_file_path = from_text.convert_text_to_device_path(cstr16!("\\casper\\vmlinuz"))?;
    let kernel_device_path = utilities.append_path(&boot_device_path, &kernel_file_path)?;

    uefi::println!("Kernel: \\casper\\vmlinuz");
    uefi::println!("Initrd: \\casper\\initrd");

    let kernel_handle = boot::load_image(
        image_handle,
        LoadImageSource::FromDevicePath {
            device_path: &kernel_device_path,
            boot_policy: BootPolicy::ExactMatch,
        },
    )?;

    let mut kernel_image = boot::open_protocol_exclusive::<LoadedImage>(kernel_handle)?;
    let cmdline = cstr16!(
        "efi=noruntime fsck.mode=skip systemd.mask=serial-getty@ttyAMA0.service systemd.mask=casper-md5check.service --- console=tty0"
    );
    unsafe {
        kernel_image.set_load_options(
            cmdline.as_ptr().cast(),
            size_of_val(cmdline.as_bytes()) as u32,
        );
    }
    drop(kernel_image);

    uefi::println!("Command line: {}", cmdline);
    info!("hinode-efi: starting Ubuntu Linux EFI stub");

    boot::start_image(kernel_handle)
}

fn install_initrd_load_file2(image_handle: Handle) -> uefi::Result {
    let initrd = read_initrd(image_handle)?.into_boxed_slice();

    let initrd: &'static [u8] = Box::leak(initrd);
    unsafe {
        INITRD_PTR = initrd.as_ptr();
        INITRD_LEN = initrd.len();
    }

    let initrd_handle = unsafe {
        boot::install_protocol_interface(
            None,
            &DevicePath::GUID,
            INITRD_DEVICE_PATH.as_ptr().cast::<c_void>(),
        )?
    };
    unsafe {
        boot::install_protocol_interface(
            Some(initrd_handle),
            &LoadFile2::GUID,
            (&raw mut INITRD_LOAD_FILE2).cast::<c_void>(),
        )?;
    }

    uefi::println!("Initrd LoadFile2: {} bytes", initrd.len());
    Ok(())
}

fn read_initrd(image_handle: Handle) -> uefi::Result<Vec<u8>> {
    let mut file_system = boot::get_image_file_system(image_handle)?;
    let mut root = file_system.open_volume()?;
    let mut initrd = root
        .open(
            cstr16!("\\casper\\initrd"),
            FileMode::Read,
            FileAttribute::empty(),
        )?
        .into_regular_file()
        .ok_or(Status::INVALID_PARAMETER)?;

    let initrd_info = initrd.get_boxed_info::<FileInfo>()?;
    let initrd_size = initrd_info.file_size() as usize;
    let mut initrd_buffer = vec![0u8; initrd_size];
    let mut offset = 0usize;

    while offset < initrd_size {
        let end = (offset + 1024 * 1024).min(initrd_size);
        let bytes_read = initrd.read(&mut initrd_buffer[offset..end])?;
        if bytes_read == 0 {
            return Err(Status::ABORTED.into());
        }
        offset += bytes_read;
    }

    Ok(initrd_buffer)
}

unsafe extern "efiapi" fn load_initrd(
    _this: *mut LoadFile2Protocol,
    _file_path: *const DevicePathProtocol,
    boot_policy: Boolean,
    buffer_size: *mut usize,
    buffer: *mut c_void,
) -> RawStatus {
    if boot_policy == Boolean::TRUE || buffer_size.is_null() {
        return RawStatus::INVALID_PARAMETER;
    }

    let initrd_len = unsafe { INITRD_LEN };
    let initrd_ptr = unsafe { INITRD_PTR };
    if initrd_ptr.is_null() || initrd_len == 0 {
        return RawStatus::NOT_FOUND;
    }

    if buffer.is_null() || unsafe { *buffer_size } < initrd_len {
        unsafe {
            *buffer_size = initrd_len;
        }
        return RawStatus::BUFFER_TOO_SMALL;
    }

    unsafe {
        ptr::copy_nonoverlapping(initrd_ptr, buffer.cast::<u8>(), initrd_len);
        *buffer_size = initrd_len;
    }
    RawStatus::SUCCESS
}
