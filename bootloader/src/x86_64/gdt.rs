use core::fmt::Debug;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct GdtDesc {
    limit_low: u16,
    base_low: u16,
    base_mid: u8,
    access: GdtDescAccess,
    flags_limit: GdtDescFlags,
    base_high: u8,
}

impl GdtDesc {
    pub fn table_iter() -> GdtTableIter {
        let gdtr = Gdtr::read();
        GdtTableIter {
            base: gdtr.base as *const GdtDesc,
            len: gdtr.limit as usize / core::mem::size_of::<GdtDesc>(),
            index: 0,
        }
    }
}

#[derive(Debug)]
pub struct GdtTableIter {
    base: *const GdtDesc,
    len: usize,
    index: usize,
}

impl Iterator for GdtTableIter {
    type Item = GdtDesc;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.len {
            return None;
        }

        let desc = unsafe { *self.base.add(self.index) };
        self.index += 1;
        Some(desc)
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct GdtDescAccess(u8);

impl Debug for GdtDescAccess {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("GdtDescAccess")
            .field("a", &((self.0 << 0) != 0))
            .field("rw", &((self.0 << 1) != 0))
            .field("dc", &((self.0 << 2) != 0))
            .field("e", &((self.0 << 3) != 0))
            .field("s", &((self.0 << 4) != 0))
            .field("dpl", &((self.0 << 5) & 0xF))
            .field("p", &((self.0 << 7) != 0))
            .finish()
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct GdtDescFlags(u8);

impl Debug for GdtDescFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("GdtDescAccess")
            .field("l", &((self.0 << 5) != 0))
            .field("db", &((self.0 << 6) != 0))
            .field("g", &((self.0 << 7) != 0))
            .finish()
    }
}

#[derive(Debug)]
#[repr(C, packed)]
pub struct Gdtr {
    limit: u16,
    base: u64,
}

impl Gdtr {
    pub fn read() -> Self {
        unsafe {
            let mut gdtr: Self = Self { limit: 0, base: 0 };
            core::arch::asm!("sgdt [{}]", in(reg) &mut gdtr);
            gdtr
        }
    }
}
