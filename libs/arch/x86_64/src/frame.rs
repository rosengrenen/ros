use crate::paging::{PhysAddr, VirtAddr};

pub enum Frame {
    Frame1GiB(Frame1GiB),
    Frame2MiB(Frame2MiB),
    Frame4KiB(Frame4KiB),
}

#[derive(Debug)]
pub struct Frame1GiB {
    addr: PhysAddr,
}

impl Frame1GiB {
    pub(crate) fn new(addr: PhysAddr) -> Self {
        Self { addr }
    }

    pub(crate) unsafe fn with_offset(&self, virt_addr: VirtAddr) -> PhysAddr {
        unsafe { self.addr.add(virt_addr.inner() & 0x3fff_ffff) }
    }

    pub fn addr(&self) -> PhysAddr {
        self.addr
    }
}

#[derive(Debug)]
pub struct Frame2MiB {
    addr: PhysAddr,
}

impl Frame2MiB {
    pub(crate) fn new(addr: PhysAddr) -> Self {
        Self { addr }
    }

    pub(crate) unsafe fn with_offset(&self, virt_addr: VirtAddr) -> PhysAddr {
        unsafe { self.addr.add(virt_addr.inner() & 0x1f_ffff) }
    }

    pub fn addr(&self) -> PhysAddr {
        self.addr
    }
}

#[derive(Debug)]
pub struct Frame4KiB {
    addr: PhysAddr,
}

impl Frame4KiB {
    pub(crate) fn new(addr: PhysAddr) -> Self {
        Self { addr }
    }

    pub(crate) unsafe fn with_offset(&self, virt_addr: VirtAddr) -> PhysAddr {
        unsafe { self.addr.add(virt_addr.inner() & 0xfff) }
    }

    pub fn addr(&self) -> PhysAddr {
        self.addr
    }
}
