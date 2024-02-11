use crate::addr::PhysAddr;

pub trait FrameAllocator {
    fn allocate_frames(&self, num_frames: usize) -> Result<PhysAddr, FrameAllocError>;

    fn allocate_frame(&self) -> Result<PhysAddr, FrameAllocError> {
        self.allocate_frames(1)
    }

    fn deallocate_frames(&self, addr: PhysAddr, num_frames: usize) -> Result<(), FrameAllocError>;

    fn deallocate_frame(&self, addr: PhysAddr) -> Result<(), FrameAllocError> {
        self.deallocate_frames(addr, 1)
    }
}

#[derive(Debug)]
pub struct FrameAllocError;

impl<F> FrameAllocator for &F
where
    F: FrameAllocator,
{
    fn allocate_frames(&self, num_frames: usize) -> Result<PhysAddr, FrameAllocError> {
        (**self).allocate_frames(num_frames)
    }

    fn deallocate_frames(&self, addr: PhysAddr, num_frames: usize) -> Result<(), FrameAllocError> {
        (**self).deallocate_frames(addr, num_frames)
    }
}
