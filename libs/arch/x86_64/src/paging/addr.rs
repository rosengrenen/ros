use common::addr::PhysAddr;
use common::addr::VirtAddr;

pub(crate) trait VirtAddrExt {
    fn p4_index(&self) -> usize;

    fn p3_index(&self) -> usize;

    fn p2_index(&self) -> usize;

    fn p1_index(&self) -> usize;
}

impl VirtAddrExt for VirtAddr {
    fn p4_index(&self) -> usize {
        self.as_u64() as usize >> 39 & 0x1ff
    }

    fn p3_index(&self) -> usize {
        self.as_u64() as usize >> 30 & 0x1ff
    }

    fn p2_index(&self) -> usize {
        self.as_u64() as usize >> 21 & 0x1ff
    }

    fn p1_index(&self) -> usize {
        self.as_u64() as usize >> 12 & 0x1ff
    }
}
