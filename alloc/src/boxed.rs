use core::{
    alloc::{AllocError, Allocator, Layout},
    fmt::{self, Debug},
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
    ptr::Unique,
};

pub struct Box<'alloc, T, A: Allocator> {
    ptr: Unique<T>,
    alloc: &'alloc A,
}

impl<'alloc, T, A: Allocator> Box<'alloc, T, A> {
    pub fn new(value: T, alloc: &'alloc A) -> Result<Self, AllocError> {
        let layout = Layout::new::<T>();
        let ptr = alloc.allocate(layout)?;
        let ptr: Unique<T> = ptr.cast().into();
        unsafe {
            core::ptr::write(ptr.as_ptr(), value);
        }
        Ok(Self { ptr, alloc })
    }
}

impl<'alloc, T, A: Allocator> Box<'alloc, MaybeUninit<T>, A> {
    pub fn new_uninit(alloc: &'alloc A) -> Result<Self, AllocError> {
        let layout = Layout::new::<T>();
        let ptr = alloc.allocate(layout)?;
        let ptr = ptr.cast().into();
        Ok(Self { ptr, alloc })
    }

    pub unsafe fn assume_init(self) -> Box<'alloc, T, A> {
        let ptr = core::ptr::read(&self.ptr);
        let alloc = core::ptr::read(&self.alloc);
        core::mem::forget(self);
        Box {
            ptr: ptr.cast(),
            alloc,
        }
    }
}

impl<'alloc, T: fmt::Debug, A: Allocator> Debug for Box<'alloc, T, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(unsafe { self.ptr.as_ref() }, f)
    }
}

impl<'alloc, T, A: Allocator> Deref for Box<'alloc, T, A> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

impl<'alloc, T, A: Allocator> DerefMut for Box<'alloc, T, A> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.ptr.as_mut() }
    }
}

impl<'alloc, T, A: Allocator> Drop for Box<'alloc, T, A> {
    fn drop(&mut self) {
        let layout = Layout::new::<T>();
        unsafe { self.alloc.deallocate(self.ptr.cast().into(), layout) }
    }
}
