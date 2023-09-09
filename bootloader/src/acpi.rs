use core::mem::size_of;

#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct Rsdp {
    pub signature: [u8; 8],
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub revision: u8,
    pub rsdt_addr: u32,
    // Revision 2+
    pub length: u32,
    pub xsdt_addr: u64,
    pub extended_checksum: u8,
    _reserved: [u8; 3],
}

impl Rsdp {
    pub unsafe fn from_addr(addr: u64) -> Self {
        unsafe { *(addr as *const Self) }
    }

    pub fn table_ptrs(&self) -> &[*const DefinitionHeader] {
        unsafe {
            let xsdt_ptr = self.xsdt_addr as *const DefinitionHeader;
            let xsdt = xsdt_ptr.read();
            let end = xsdt_ptr.add(1).cast();
            let len = (xsdt.length as usize - size_of::<DefinitionHeader>())
                / size_of::<*const DefinitionHeader>();
            // let len =
            core::slice::from_raw_parts(end, len)
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct DefinitionHeader {
    pub signature: [u8; 4],
    pub length: u32,
    pub revision: u8,
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub oem_table_id: u64,
    pub oem_revision: u32,
    pub creator_id: u32,
    pub creator_revision: u32,
}

pub trait AcpiTable {
    const SIGNATURE: [u8; 4];

    fn header(&self) -> &DefinitionHeader;
}

#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct Fadt {
    pub header: DefinitionHeader,
    pub firmware_ctrl: u32,
    pub dsdt: u32,

    // field used in ACPI 1.0; no longer in use, for compatibility only
    reserved: u8,

    pub preferred_power_management_profile: u8,
    pub sci_interrupt: u16,
    pub smi_command_port: u32,
    pub acpi_enable: u8,
    pub acpi_disable: u8,
    pub s4bios_req: u8,
    pub pstate_control: u8,
    pub pm1a_event_block: u32,
    pub pm1b_event_block: u32,
    pub pm1a_control_block: u32,
    pub pm1b_control_block: u32,
    pub pm2_control_block: u32,
    pub pm_timer_block: u32,
    pub gpe0_block: u32,
    pub gpe1_block: u32,
    pub pm1_event_length: u8,
    pub pm1_control_length: u8,
    pub pm2_control_length: u8,
    pub pm_timer_length: u8,
    pub gpe0_length: u8,
    pub gpe1_length: u8,
    pub gpe1_base: u8,
    pub c_state_control: u8,
    pub worst_c2_latency: u16,
    pub worst_c3_latency: u16,
    pub flush_size: u16,
    pub flush_stride: u16,
    pub duty_offset: u8,
    pub duty_width: u8,
    pub day_alarm: u8,
    pub month_alarm: u8,
    pub century: u8,

    // reserved in ACPI 1.0; used since ACPI 2.0+
    pub boot_architecture_flags: u16,

    reserved2: u8,
    pub flags: u32,
    // 12 byte structure; see below for details
    pub reset_reg: GenericAddressStructure,

    pub reset_value: u8,
    reserved3: [u8; 3],

    // 64bit pointers - Available on ACPI 2.0+
    pub x_firmware_control: u64,
    pub x_dsdt: u64,

    pub x_pm1a_event_block: GenericAddressStructure,
    pub x_pm1b_event_block: GenericAddressStructure,
    pub x_pm1a_control_block: GenericAddressStructure,
    pub x_pm1b_control_block: GenericAddressStructure,
    pub x_pm2_control_block: GenericAddressStructure,
    pub x_pm_timer_block: GenericAddressStructure,
    pub x_gpe0_block: GenericAddressStructure,
    pub x_gpe1_block: GenericAddressStructure,
    pub sleep_ctrl_reg: GenericAddressStructure,
    pub sleep_status_reg: GenericAddressStructure,
    pub hypervisor_vendor_id: u64,
}

#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct GenericAddressStructure {
    address_space: u8,
    bit_width: u8,
    bit_offset: u8,
    access_size: u8,
    address: u64,
}
