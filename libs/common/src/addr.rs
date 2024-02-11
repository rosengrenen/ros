use core::fmt;

#[derive(Clone, Copy)]
pub struct PhysAddr {
    addr: u64,
}

impl PhysAddr {
    pub fn new(addr: u64) -> Self {
        Self { addr }
    }

    pub fn add(&self, offset: u64) -> Self {
        Self {
            addr: self.addr + offset,
        }
    }

    pub fn as_u64(&self) -> u64 {
        self.addr
    }

    // // TODO: remove? should probably only create pointers from virt addrs
    // pub fn as_ptr<T>(&self) -> *const T {
    //     // TODO: is alignment checking necessary?
    //     self.addr as *const T
    // }

    // pub fn as_ptr_mut<T>(&mut self) -> *mut T {
    //     // TODO: is alignment checking necessary?
    //     self.addr as *mut T
    // }

    // TODO: make this generic, can share strat with some of the page table stuffs
    // see translate_hhdm
    pub fn as_virt_ident(&self) -> VirtAddr {
        VirtAddr::new(self.as_u64())
    }
}

impl fmt::Debug for PhysAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("PhysAddr").field(&self.addr).finish()
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

// TODO: check which methods should be unsafe, all probably lol
impl VirtAddr {
    pub fn new(addr: u64) -> Self {
        Self { addr }
    }

    pub fn as_u64(&self) -> u64 {
        self.addr
    }

    pub fn as_ptr<T>(&self) -> *const T {
        // TODO: is alignment checking necessary?
        self.addr as *const T
    }

    pub fn as_ptr_mut<T>(&mut self) -> *mut T {
        // TODO: is alignment checking necessary?
        self.addr as *mut T
    }

    pub fn as_ref<T>(&self) -> &'static T {
        unsafe { &*self.as_ptr() }
    }

    pub fn as_ref_mut<T>(&mut self) -> &'static mut T {
        unsafe { &mut *self.as_ptr_mut() }
    }

    pub fn as_slice<T>(&self, len: usize) -> &'static [T] {
        unsafe { &*core::ptr::slice_from_raw_parts(self.as_ptr(), len) }
    }

    pub fn as_slice_mut<T>(&mut self, len: usize) -> &'static mut [T] {
        unsafe { &mut *core::ptr::slice_from_raw_parts_mut(self.as_ptr_mut(), len) }
    }
}

impl fmt::Debug for VirtAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("VirtAddr").field(&self.addr).finish()
    }
}

impl fmt::Display for VirtAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:#x}", self.addr)
    }
}
