use alloc::vec::Vec;
use core::alloc::Allocator;
use core::ffi::c_void;

use crate::Handle;
use crate::Status;
use crate::TableHeader;

impl BootServices {
    pub fn allocate_pages(
        &self,
        allocate_type: AllocateType,
        memory_type: MemoryType,
        pages: usize,
    ) -> Result<u64, usize> {
        let mut memory_address = 0;
        let status = (self.allocate_pages)(allocate_type, memory_type, pages, &mut memory_address);
        if status != 0 {
            return Err(status);
        }

        Ok(memory_address)
    }

    pub fn free_pages(&self, memory_address: u64, pages: usize) -> Result<(), usize> {
        let status = (self.free_pages)(memory_address, pages);
        if status != 0 {
            return Err(status);
        }

        Ok(())
    }

    pub fn get_memory_map<A: Allocator + Clone>(&self, alloc: A) -> Result<MemoryMap<A>, usize> {
        let (mut memory_map_size, mut entry_size) = self.get_memory_map_size()?;
        let mut buffer =
            Vec::with_size_default(memory_map_size, alloc.clone()).map_err(|_| 0usize)?;
        let mut map_key = 0;
        loop {
            let mut descriptor_version = 0;
            let mut status = (self.get_memory_map)(
                &mut memory_map_size,
                buffer.as_mut_ptr() as _,
                &mut map_key,
                &mut entry_size,
                &mut descriptor_version,
            );
            status &= 0xFFFFFFFF;

            if status == 0 {
                break;
            }

            if status != 5 {
                return Err(status);
            }

            buffer = Vec::with_size_default(memory_map_size, alloc.clone()).map_err(|_| 0usize)?;
        }

        Ok(MemoryMap {
            buffer,
            key: map_key,
            entry_size,
            len: memory_map_size / entry_size,
        })
    }

    pub fn get_memory_map_size(&self) -> Result<(usize, usize), usize> {
        let mut memory_map_size = 0;
        let mut map_key = 0;
        let mut descriptor_size = 0;
        let mut descriptor_version = 0;
        let mut status = (self.get_memory_map)(
            &mut memory_map_size,
            core::ptr::null_mut(),
            &mut map_key,
            &mut descriptor_size,
            &mut descriptor_version,
        );
        status &= 0xFFFFFFFF;
        if status != 5 {
            return Err(status);
        }

        Ok((memory_map_size, descriptor_size))
    }

    pub fn allocate_pool(&self, pool_type: MemoryType, size: usize) -> Result<*mut c_void, usize> {
        let mut buffer = core::ptr::null_mut();
        let status = (self.allocate_pool)(pool_type, size, &mut buffer);
        if status != 0 {
            return Err(status);
        }

        Ok(buffer)
    }

    pub fn free_pool(&self, buffer: *const c_void) -> Result<(), usize> {
        let status = (self.free_pool)(buffer);
        if status != 0 {
            return Err(status);
        }

        Ok(())
    }

    pub(crate) fn exit_boot_services(
        &self,
        image_handle: Handle,
        map_key: usize,
    ) -> Result<(), usize> {
        let status = (self.exit_boot_services)(image_handle, map_key);
        // TODO: do this everywhere uefi functions are called, but errors have a high bit set and warnings do not, and success is always 0
        // so this could technically be just a warning
        if status != 0 {
            return Err(status);
        }

        Ok(())
    }

    pub fn open_protocol<T: UefiProtocol>(&self, handle: Handle) -> Result<&mut T, usize> {
        let mut interface = core::ptr::null();
        let status = (self.open_protocol)(
            handle,
            &T::GUID,
            &mut interface as *mut _,
            handle,
            core::ptr::null(),
            0x20, // EFI_OPEN_PROTOCOL_EXCLUSIVE
        );
        if status != 0 {
            return Err(status);
        }

        let interface = unsafe { &mut *(interface as *mut T) };
        Ok(interface)
    }

    pub fn locate_protocol<T: UefiProtocol>(&self) -> Result<&mut T, usize> {
        let mut interface = core::ptr::null();
        let status = (self.locate_protocol)(&T::GUID, core::ptr::null(), &mut interface as *mut _);
        if status != 0 {
            return Err(status);
        }

        let interface = unsafe { &mut *(interface as *mut T) };
        Ok(interface)
    }
}

pub trait UefiProtocol {
    const GUID: Guid;
}

#[derive(Debug)]
pub struct MemoryMap<A: Allocator> {
    pub buffer: Vec<u8, A>,
    pub key: usize,
    pub entry_size: usize,
    pub len: usize,
}

impl<A: Allocator> MemoryMap<A> {
    pub fn iter(&self) -> MemoryMapIter<A> {
        MemoryMapIter {
            memory_map: self,
            index: 0,
        }
    }
}

pub struct MemoryMapIter<'iter, A: Allocator> {
    memory_map: &'iter MemoryMap<A>,
    index: usize,
}

impl<'iter, A: Allocator> Iterator for MemoryMapIter<'iter, A> {
    type Item = &'iter MemoryDescriptor;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.memory_map.len {
            return None;
        }

        let memory_descriptor = unsafe {
            let ptr_with_offset = self
                .memory_map
                .buffer
                .as_ptr()
                .add(self.index * self.memory_map.entry_size);
            &*(ptr_with_offset as *const MemoryDescriptor)
        };
        self.index += 1;
        Some(memory_descriptor)
    }
}

impl<'iter, A: Allocator> ExactSizeIterator for MemoryMapIter<'iter, A> {
    fn len(&self) -> usize {
        self.memory_map.len
    }
}

pub struct MemoryMapIntoIter<A: Allocator> {
    memory_map: MemoryMap<A>,
    index: usize,
}

impl<A: Allocator> IntoIterator for MemoryMap<A> {
    type Item = MemoryDescriptor;
    type IntoIter = MemoryMapIntoIter<A>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            memory_map: self,
            index: 0,
        }
    }
}

impl<A: Allocator> Iterator for MemoryMapIntoIter<A> {
    type Item = MemoryDescriptor;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.memory_map.len {
            return None;
        }

        let memory_descriptor = unsafe {
            *self
                .memory_map
                .buffer
                .as_ptr()
                .add(self.index * self.memory_map.entry_size)
                .cast()
        };
        self.index += 1;
        Some(memory_descriptor)
    }
}

impl<A: Allocator> ExactSizeIterator for MemoryMapIntoIter<A> {
    fn len(&self) -> usize {
        self.memory_map.len
    }
}

/// UEFI Spec 2.10 section 4.4.1
#[repr(C)]
pub struct BootServices {
    pub header: TableHeader,

    // Task Priority Services
    pub raise_tpl: extern "efiapi" fn() -> Status, // EFI 1.0+
    pub restore_tpl: extern "efiapi" fn() -> Status, // EFI 1.0+

    // Memory Services
    /// UEFI Spec 2.10 section 7.2.1
    pub allocate_pages: extern "efiapi" fn(
        allocate_type: AllocateType,
        memory_type: MemoryType,
        pages: usize,
        physical_address: &mut u64,
    ) -> Status, // EFI 1.0+
    /// UEFI Spec 2.10 section 7.2.2
    pub free_pages: extern "efiapi" fn(memory: u64, pages: usize) -> Status, // EFI 1.0+
    /// UEFI Spec 2.10 section 7.2.3
    pub get_memory_map: extern "efiapi" fn(
        memory_map_size: &mut usize,
        memory_map: *mut MemoryDescriptor,
        map_key: &mut usize,
        descriptor_size: &mut usize,
        descriptor_version: &mut u32,
    ) -> Status, // EFI 1.0+
    pub allocate_pool:
        extern "efiapi" fn(pool_type: MemoryType, size: usize, buffer: *mut *mut c_void) -> Status, // EFI 1.0+
    pub free_pool: extern "efiapi" fn(buffer: *const c_void) -> Status, // EFI 1.0+

    // Event & Timer Services
    pub create_event: extern "efiapi" fn() -> Status, // EFI 1.0+
    pub set_timer: extern "efiapi" fn() -> Status,    // EFI 1.0+
    pub wait_for_event: extern "efiapi" fn() -> Status, // EFI 1.0+
    pub signal_event: extern "efiapi" fn() -> Status, // EFI 1.0+
    pub close_event: extern "efiapi" fn() -> Status,  // EFI 1.0+
    pub check_event: extern "efiapi" fn() -> Status,  // EFI 1.0+

    // Protocol Handler Services
    pub install_protocol_interface: extern "efiapi" fn() -> Status, // EFI 1.
    pub reinstall_protocol_interface: extern "efiapi" fn() -> Status, // EFI 1.
    pub uninstall_protocol_interface: extern "efiapi" fn() -> Status, // EFI 1.
    pub handle_protocol: extern "efiapi" fn() -> Status,            // EFI 1.
    pub reserved: *const c_void,                                    // EFI 1.0+
    pub register_protocol_notify: extern "efiapi" fn() -> Status,   // EFI 1.
    pub locate_handle: extern "efiapi" fn() -> Status,              // EFI 1.
    pub locate_device_path: extern "efiapi" fn() -> Status,         // EFI 1.
    pub install_configuration_table: extern "efiapi" fn() -> Status, // EFI 1.

    // Image Services
    pub load_image: extern "efiapi" fn() -> Status, // EFI 1.0+
    pub start_image: extern "efiapi" fn() -> Status, // EFI 1.0+
    pub exit: extern "efiapi" fn() -> Status,       // EFI 1.0+
    pub unload_image: extern "efiapi" fn() -> Status, // EFI 1.0+
    pub exit_boot_services: extern "efiapi" fn(image_handle: Handle, map_key: usize) -> Status, // EFI 1.0+

    // Miscellaneous Services
    pub get_next_monotonic_count: extern "efiapi" fn() -> Status, // EFI 1.0+
    pub stall: extern "efiapi" fn() -> Status,                    // EFI 1.0+
    pub set_watchdog_timer: extern "efiapi" fn() -> Status,       // EFI 1.0+

    // DriverSupport Services
    pub connect_controller: extern "efiapi" fn() -> Status, // EFI 1.1
    pub disconnect_controller: extern "efiapi" fn() -> Status, // EFI 1.1+

    // Open and Close Protocol Services
    pub open_protocol: extern "efiapi" fn(
        handle: Handle,
        protocol: &Guid,
        interface: *mut *const c_void,
        agent_handle: Handle,
        controller_handle: Handle,
        attributes: u32,
    ) -> Status, // EFI 1.1+
    pub close_protocol: extern "efiapi" fn() -> Status, // EFI 1.1+
    pub open_protocol_information: extern "efiapi" fn() -> Status, // EFI 1.1+

    // Library Services
    pub protocols_per_handle: extern "efiapi" fn() -> Status, // EFI 1.1+
    pub locate_handle_buffer: extern "efiapi" fn() -> Status, // EFI 1.1+
    /// UEFI Spec 2.10 section 7.3.16
    pub locate_protocol: extern "efiapi" fn(
        protocol: &Guid,
        registration: *const c_void,
        interface: *mut *const c_void,
    ) -> Status, // EFI 1.1+
    pub install_multiple_protocol_interfaces: extern "efiapi" fn() -> Status, // EFI 1.1+
    pub uninstall_multiple_protocol_interfaces: extern "efiapi" fn() -> Status, // EFI 1.1+*

    // 32-bit CRC Services
    pub calculate_crc32: extern "efiapi" fn() -> Status, // EFI 1.1+

    // Miscellaneous Services
    pub copy_mem: extern "efiapi" fn() -> Status, // EFI 1.1+
    pub set_mem: extern "efiapi" fn() -> Status,  // EFI 1.1+
    pub create_event_ex: extern "efiapi" fn() -> Status, // UEFI 2.0+
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Guid(pub u32, pub u16, pub u16, pub [u8; 8]);

impl core::fmt::Debug for Guid {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Guid")
            .field(&format_args!(
                "{:08x}-{:04x}-{:04x}-{:04x}-{:04x}{:08x}",
                self.0,
                self.1,
                self.2,
                u16::from_be_bytes([self.3[0], self.3[1]]),
                u16::from_be_bytes([self.3[2], self.3[3]]),
                u32::from_be_bytes([self.3[4], self.3[5], self.3[6], self.3[7]]),
            ))
            .finish()
    }
}

/// UEFI Spec 2.10 section 7.3.2
#[repr(C)]
pub enum InterfaceType {
    NativeInterface,
}

/// UEFI Spec 2.10 section 7.2.3
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct MemoryDescriptor {
    pub ty: MemoryType,
    pub physical_start: u64,
    pub virtual_start: u64,
    pub number_of_pages: u64,
    pub attribute: u64,
}

/// UEFI Spec 2.10 section 7.2.1
#[repr(C)]
pub enum AllocateType {
    AllocateAnyPages,
    AllocateMaxAddress,
    AllocateAddress,
    MaxAllocateType,
}

/// UEFI Spec 2.10 section 7.2.1
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum MemoryType {
    EfiReservedMemoryType,
    EfiLoaderCode,
    EfiLoaderData,
    EfiBootServicesCode,
    EfiBootServicesData,
    EfiRuntimeServicesCode,
    EfiRuntimeServicesData,
    EfiConventionalMemory,
    EfiUnusableMemory,
    EfiACPIReclaimMemory,
    EfiACPIMemoryNVS,
    EfiMemoryMappedIO,
    EfiMemoryMappedIOPortSpace,
    EfiPalCode,
    EfiPersistentMemory,
    EfiUnacceptedMemoryType,
}

impl MemoryType {
    pub fn usable_by_loader(&self) -> bool {
        match self {
            MemoryType::EfiReservedMemoryType => false,
            MemoryType::EfiLoaderCode => false,
            MemoryType::EfiLoaderData => true,
            MemoryType::EfiBootServicesCode => true,
            MemoryType::EfiBootServicesData => true,
            MemoryType::EfiRuntimeServicesCode => false,
            MemoryType::EfiRuntimeServicesData => false,
            MemoryType::EfiConventionalMemory => true,
            MemoryType::EfiUnusableMemory => false,
            MemoryType::EfiACPIReclaimMemory => false,
            MemoryType::EfiACPIMemoryNVS => false,
            MemoryType::EfiMemoryMappedIO => false,
            MemoryType::EfiMemoryMappedIOPortSpace => false,
            MemoryType::EfiPalCode => false,
            MemoryType::EfiPersistentMemory => true,
            MemoryType::EfiUnacceptedMemoryType => false,
        }
    }

    pub fn usable_by_kernel(&self) -> bool {
        match self {
            MemoryType::EfiReservedMemoryType => false,
            MemoryType::EfiLoaderCode => true,
            MemoryType::EfiLoaderData => true,
            MemoryType::EfiBootServicesCode => true,
            MemoryType::EfiBootServicesData => true,
            MemoryType::EfiRuntimeServicesCode => false,
            MemoryType::EfiRuntimeServicesData => false,
            MemoryType::EfiConventionalMemory => true,
            MemoryType::EfiUnusableMemory => false,
            MemoryType::EfiACPIReclaimMemory => false, // TODO: must set up acpi in bootloader, then set to trup
            MemoryType::EfiACPIMemoryNVS => false,
            MemoryType::EfiMemoryMappedIO => false,
            MemoryType::EfiMemoryMappedIOPortSpace => false,
            MemoryType::EfiPalCode => false,
            MemoryType::EfiPersistentMemory => true,
            MemoryType::EfiUnacceptedMemoryType => false,
        }
    }
}
