use core::{
    alloc::{AllocError, Allocator, Layout},
    fmt,
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
    ptr::Unique,
};

pub struct Box<T: ?Sized, A: Allocator> {
    ptr: Unique<T>,
    alloc: A,
}

impl<T, A: Allocator> Box<T, A> {
    pub fn new(value: T, alloc: A) -> Result<Self, AllocError> {
        let layout = Layout::new::<T>();
        let ptr = alloc.allocate(layout)?;
        let ptr: Unique<T> = ptr.cast().into();
        unsafe {
            ptr.as_ptr().write(value);
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
        let ptr = unsafe { core::ptr::read(&self.ptr) };
        let alloc = unsafe { core::ptr::read(&self.alloc) };
        core::mem::forget(self);
        Box {
            ptr: ptr.cast(),
            alloc,
        }
    }
}

impl<T: fmt::Debug, A: Allocator> fmt::Debug for Box<T, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(unsafe { self.ptr.as_ref() }, f)
    }
}

impl<T: fmt::Display, A: Allocator> fmt::Display for Box<T, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(unsafe { self.ptr.as_ref() }, f)
    }
}

impl<T: ?Sized, A: Allocator> Deref for Box<T, A> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T: ?Sized, A: Allocator> DerefMut for Box<T, A> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.ptr.as_mut() }
    }
}

impl<T: ?Sized, A: Allocator> Drop for Box<T, A> {
    fn drop(&mut self) {
        unsafe {
            core::ptr::drop_in_place(self.ptr.as_ptr());
            let layout = Layout::for_value_raw(self.ptr.as_ptr());
            self.alloc.deallocate(self.ptr.cast().into(), layout)
        }
    }
}
