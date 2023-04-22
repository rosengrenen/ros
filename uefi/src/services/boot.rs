use core::ffi::c_void;

use crate::{Handle, Status, TableHeader};

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

    pub fn get_memory_map(&self) -> Result<MemoryMap, usize> {
        let (mut memory_map_size, mut entry_size) = self.get_memory_map_size()?;
        let mut buffer = vec![0u8; memory_map_size];
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

            buffer = vec![0u8; memory_map_size];
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
        if status != 0 {
            return Err(status);
        }

        crate::allocator::disable();

        Ok(())
    }

    pub fn locate_protocol<T>(&self, protocol: &Guid) -> Result<&'static T, usize> {
        let mut interface = core::ptr::null();
        let status = (self.locate_protocol)(protocol, core::ptr::null(), &mut interface as *mut _);
        if status != 0 {
            return Err(status);
        }

        let interface = unsafe { &*(interface as *const T) };
        Ok(interface)
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct MemoryMap {
    pub buffer: alloc::vec::Vec<u8>,
    pub key: usize,
    pub entry_size: usize,
    pub len: usize,
}

impl MemoryMap {
    pub fn iter(&self) -> MemoryMapIter {
        MemoryMapIter {
            memory_map: self,
            index: 0,
        }
    }
}

#[repr(C)]
pub struct MemoryMapIter<'iter> {
    memory_map: &'iter MemoryMap,
    index: usize,
}

impl<'iter> Iterator for MemoryMapIter<'iter> {
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

impl<'iter> ExactSizeIterator for MemoryMapIter<'iter> {
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
    pub open_protocol: extern "efiapi" fn() -> Status, // EFI 1.1+
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

#[repr(C)]
pub struct Guid(pub u32, pub u16, pub u16, pub [u8; 8]);

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
#[derive(Clone, Copy, Debug)]
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
    EfiMaxMemoryType,
}
