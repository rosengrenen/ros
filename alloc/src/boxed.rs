use core::{
    alloc::{AllocError, Allocator, Layout},
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
    ptr::Unique,
};

pub struct Box<T, A: Allocator> {
    ptr: Unique<T>,
    alloc: A,
}

impl<T, A: Allocator> Box<T, A> {
    pub fn new(value: T, alloc: A) -> Result<Self, AllocError> {
        let layout = Layout::new::<T>();
        let ptr = alloc.allocate(layout)?;
        let ptr: Unique<T> = ptr.cast().into();
        unsafe {
            core::ptr::write(ptr.as_ptr(), value);
        }
        Ok(Self { ptr, alloc })
    }
}

impl<T, A: Allocator> Box<MaybeUninit<T>, A> {
    pub fn new_uninit(alloc: A) -> Result<Self, AllocError> {
        let layout = Layout::new::<T>();
        let ptr = alloc.allocate(layout)?;
        let ptr = ptr.cast().into();
        Ok(Self { ptr, alloc })
    }

    pub unsafe fn assume_init(self) -> Box<T, A> {
        let ptr = core::ptr::read(&self.ptr);
        let alloc = core::ptr::read(&self.alloc);
        core::mem::forget(self);
        Box {
            ptr: ptr.cast(),
            alloc,
        }
    }
}

impl<T, A: Allocator> Deref for Box<T, A> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T, A: Allocator> DerefMut for Box<T, A> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.ptr.as_mut() }
    }
}

impl<T, A: Allocator> Drop for Box<T, A> {
    fn drop(&mut self) {
        let layout = Layout::new::<T>();
        unsafe { self.alloc.deallocate(self.ptr.cast().into(), layout) }
    }
}
