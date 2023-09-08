use core::{
    alloc::{AllocError, Allocator, Layout, LayoutError},
    fmt::Debug,
    ops::{Deref, DerefMut},
    ptr::Unique,
};

use crate::iter::FromIteratorIn;

pub struct Vec<'alloc, T, A: Allocator> {
    ptr: Unique<T>,
    cap: usize,
    len: usize,
    alloc: &'alloc A,
}

impl<'alloc, T: Debug, A: Allocator> Debug for Vec<'alloc, T, A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(&**self, f)
    }
}

impl<'alloc, T, A: Allocator> Vec<'alloc, T, A> {
    pub fn new(alloc: &'alloc A) -> Self {
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

    pub fn push(&mut self, item: T) -> Result<(), PushError> {
        if self.len == self.cap {
            // TODO: refactor and don't grow x2, there is some better number
            let new_cap = (self.cap * 2).max(1);
            let new_layout = Layout::array::<T>(new_cap)?;
            let new_ptr: Unique<T> = self.alloc.allocate(new_layout)?.cast().into();

            // Copy elements to new memory region
            for i in 0..self.len {
                unsafe {
                    core::ptr::write(
                        new_ptr.as_ptr().add(i),
                        core::ptr::read(self.as_ptr().add(i)),
                    );
                }
            }

            if self.cap != 0 {
                let current_layout = Layout::array::<T>(self.cap)?;
                unsafe {
                    self.alloc
                        .deallocate(self.ptr.cast().into(), current_layout);
                }
            }

            self.ptr = new_ptr;
            self.cap = new_cap;
        }

        unsafe { core::ptr::write(self.ptr.as_ptr().add(self.len), item) }
        self.len += 1;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub enum PushError {
    AllocError(AllocError),
    LayoutError(LayoutError),
}

impl From<AllocError> for PushError {
    fn from(value: AllocError) -> Self {
        Self::AllocError(value)
    }
}

impl From<LayoutError> for PushError {
    fn from(value: LayoutError) -> Self {
        Self::LayoutError(value)
    }
}

impl<'alloc, T: Clone, A: Allocator> Vec<'alloc, T, A> {
    pub fn with_elem(value: T, n: usize, alloc: &'alloc A) -> Result<Self, ()> {
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

impl<'alloc, T, A: Allocator> FromIteratorIn<'alloc, T, A> for Vec<'alloc, T, A> {
    fn from_iter_in<I: IntoIterator<Item = T>>(
        iter: I,
        alloc: &'alloc A,
    ) -> Result<Self, AllocError> {
        let mut vec = Vec::new(alloc);
        for item in iter.into_iter() {
            vec.push(item).unwrap();
        }

        Ok(vec)
    }
}

impl<'alloc, T, A: Allocator> Deref for Vec<'alloc, T, A> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { core::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }
}

impl<'alloc, T, A: Allocator> DerefMut for Vec<'alloc, T, A> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { core::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
    }
}

impl<'alloc, T, A: Allocator> Drop for Vec<'alloc, T, A> {
    fn drop(&mut self) {
        let layout = Layout::array::<T>(self.cap)
            .expect("layout was used to allocate memory, so should always be valid");
        unsafe { self.alloc.deallocate(self.ptr.cast().into(), layout) }
    }
}

impl<'alloc, 'vec, T, A: Allocator> IntoIterator for &'vec Vec<'alloc, T, A> {
    type Item = &'vec T;
    type IntoIter = core::slice::Iter<'vec, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'alloc, 'vec, T, A: Allocator> IntoIterator for &'vec mut Vec<'alloc, T, A> {
    type Item = &'vec mut T;
    type IntoIter = core::slice::IterMut<'vec, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<'alloc, A: Allocator> core::fmt::Write for Vec<'alloc, u8, A> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.bytes() {
            self.push(b).unwrap();
        }

        Ok(())
    }
}

impl<'alloc, A: Allocator> core::fmt::Write for Vec<'alloc, char, A> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.chars() {
            self.push(c).unwrap();
        }

        Ok(())
    }
}