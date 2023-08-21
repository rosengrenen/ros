use core::{
    alloc::{AllocError, Allocator, Layout},
    fmt::Debug,
    ops::{Deref, DerefMut},
    ptr::Unique,
};

use crate::iter::FromIteratorIn;

pub struct Vec<T, A: Allocator> {
    ptr: Unique<T>,
    cap: usize,
    len: usize,
    alloc: A,
}

impl<T: Debug, A: Allocator> Debug for Vec<T, A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(&**self, f)
    }
}

impl<T, A: Allocator> Vec<T, A> {
    pub fn new(alloc: A) -> Self {
        Self {
            ptr: Unique::dangling(),
            cap: 0,
            len: 0,
            alloc,
        }
    }

    pub fn as_ptr(&self) -> *const T {
        self.ptr.as_ptr()
    }

    pub fn push(&mut self, item: T) -> Result<(), AllocError> {
        if self.len == self.cap {
            let current_layout = Layout::array::<T>(self.cap).unwrap();
            // TODO: refactor and don't grow x2, there is some better number
            let new_layout = Layout::array::<T>((self.cap * 2).max(1)).unwrap();
            let new_ptr: Unique<T> = self.alloc.allocate(new_layout)?.cast().into();

            // Copy elements to new memory region
            for i in 0..self.len {
                unsafe {
                    core::ptr::write(new_ptr.as_ptr(), core::ptr::read(self.as_ptr().add(i)));
                }
            }

            unsafe {
                self.alloc
                    .deallocate(self.ptr.cast().into(), current_layout);
            }
            self.ptr = new_ptr;
        }

        unsafe { core::ptr::write(self.ptr.as_ptr().add(self.len), item) }
        self.len += 1;
        Ok(())
    }
}

impl<T: Clone, A: Allocator> Vec<T, A> {
    pub fn with_elem(value: T, n: usize, alloc: A) -> Result<Self, ()> {
        let layout = Layout::array::<T>(n).map_err(|_| ())?;
        let ptr = alloc.allocate(layout).map_err(|_| ())?.cast::<T>();

        for i in 1..n {
            unsafe {
                let ptr = ptr.as_ptr().add(i);
                core::ptr::write(ptr, value.clone());
            }
        }

        if n > 0 {
            unsafe {
                core::ptr::write(ptr.as_ptr(), value);
            }
        }

        Ok(Self {
            ptr: ptr.into(),
            cap: layout.size(),
            len: n,
            alloc,
        })
    }
}

impl<T, A: Allocator> FromIteratorIn<T, A> for Vec<T, A> {
    fn from_iter_in<I: IntoIterator<Item = T>>(iter: I, alloc: A) -> Result<Self, AllocError> {
        let mut vec = Vec::new(alloc);
        for item in iter.into_iter() {
            vec.push(item)?;
        }

        Ok(vec)
    }
}

impl<T, A: Allocator> Deref for Vec<T, A> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { core::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }
}

impl<T, A: Allocator> DerefMut for Vec<T, A> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { core::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
    }
}

impl<T, A: Allocator> Drop for Vec<T, A> {
    fn drop(&mut self) {
        let layout = Layout::array::<T>(self.cap)
            .expect("layout was used to allocate memory, so should always be valid");
        unsafe { self.alloc.deallocate(self.ptr.cast().into(), layout) }
    }
}
