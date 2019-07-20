//! Provides support for the UEFI loaded image protocol.
//!
//! This protocol describes an image loaded into memory, including the
//! source which the image was loaded from and the location of the image
//! in memory

use crate::proto::Protocol;
use crate::table::boot::MemoryType;
use crate::unsafe_guid;
use crate::{Handle, Status};

/// The loaded image protocol describes an executable image loaded into memory.

#[repr(C)]
#[unsafe_guid("5b1b31a1-9562-11d2-8e3f-00a0c969723b")]
#[derive(Protocol)]
pub struct LoadedImage {
    revision: u32,
    parent_handle: Handle,
    // Not exposing the system_table pointer
    system_table: usize,

    // Source location of the image
    device_handle: Handle,
    file_path: usize, // EFI_DEVICE_PATH_PROTOCOL not implemented
    reserved: usize,

    // Image's load options
    load_options_size: u32,
    load_options: usize,

    // Location where the image was loaded
    image_base: usize,
    image_size: u64,
    image_code_type: MemoryType,
    image_data_type: MemoryType,

    // Function pointer to unload the image from memory
    unload: extern "win64" fn(image_handle: Handle) -> Status,
}

impl LoadedImage {
    /// Image handle for the parent which loaded this image. `NULL` if the image
    /// was loaded directly by the firmware's boot manager.
    pub fn parent(&self) -> Handle {
        self.parent_handle
    }

    /// Handle for the device this image was loaded from.
    pub fn device(&self) -> Handle {
        self.device_handle
    }

    /// Base address where the image was loaded into memory
    pub fn image_base(&self) -> usize {
        self.image_base
    }

    /// Size of the loaded image, in bytes
    pub fn image_size(&self) -> u64 {
        self.image_size
    }

    /// Memory type allocated for code sections of the image
    pub fn image_code_type(&self) -> MemoryType {
        self.image_code_type
    }

    /// Memory type allocated for data sections of the image
    pub fn image_data_type(&self) -> MemoryType {
        self.image_data_type
    }
}
