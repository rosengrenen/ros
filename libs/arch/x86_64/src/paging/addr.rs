use core::fmt;

#[derive(Clone, Copy, Debug)]
pub struct PhysAddr {
    addr: u64,
}

impl PhysAddr {
    pub fn new(addr: u64) -> Self {
        Self { addr }
    }

    pub fn inner(&self) -> u64 {
        self.addr
    }

    pub fn add(&self, offset: u64) -> Self {
        Self {
            addr: self.addr + offset,
        }
    }

    pub fn as_ptr<T>(&self) -> *const T {
        // TODO: is alignment checking necessary?
        self.addr as *const T
    }

    pub fn as_ptr_mut<T>(&mut self) -> *mut T {
        // TODO: is alignment checking necessary?
        self.addr as *mut T
    }
}

impl fmt::Display for PhysAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.addr)
    }
}

#[derive(Clone, Copy)]
pub struct VirtAddr {
    addr: u64,
}

impl VirtAddr {
    pub fn new(addr: u64) -> Self {
        Self { addr }
    }

    pub fn inner(&self) -> u64 {
        self.addr
    }

    pub fn pml4_index(&self) -> u64 {
        self.addr >> 39 & 0x1ff
    }

    pub fn pml3_index(&self) -> u64 {
        self.addr >> 30 & 0x1ff
    }

    pub fn pml2_index(&self) -> u64 {
        self.addr >> 21 & 0x1ff
    }

    pub fn pml1_index(&self) -> u64 {
        self.addr >> 12 & 0x1ff
    }
}

impl fmt::Debug for VirtAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VirtAddr")
            .field("address", &self.addr)
            .field("pml4_index", &self.pml4_index())
            .field("pml3_index", &self.pml3_index())
            .field("pml2_index", &self.pml2_index())
            .field("pml1_index", &self.pml1_index())
            .finish()
    }
}

impl fmt::Display for VirtAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.addr)
    }
}
