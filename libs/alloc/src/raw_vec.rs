use core::{
    fmt::Debug,
    ops::{Deref, DerefMut},
    ptr::Unique,
};

pub struct RawVec<T> {
    ptr: Unique<T>,
    cap: usize,
    len: usize,
}

impl<T: Debug> Debug for RawVec<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(&**self, f)
    }
}

impl<T> RawVec<T> {
    pub unsafe fn from_raw_parts(ptr: *mut T, cap: usize) -> Self {
        Self {
            ptr: Unique::from(unsafe { &mut *ptr }),
            cap,
            len: 0,
        }
    }

    pub fn as_ptr(&self) -> *const T {
        self.ptr.as_ptr()
    }

    pub fn push(&mut self, item: T) -> Result<(), PushError> {
        if self.len == self.cap {
            return Err(PushError::MaxCapacity);
        }

        unsafe { core::ptr::write(self.ptr.as_ptr().add(self.len), item) }
        self.len += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        self.len -= 1;
        let item = unsafe { self.as_ptr().add(self.len).read() };
        Some(item)
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }
}

#[derive(Clone, Debug)]
pub enum PushError {
    MaxCapacity,
}

// impl<T: Clone> RawVec<T> {
//     pub fn with_size_elem(size: usize, value: T) -> Result<Self, ()> {
//         Self::with_size_f(size, || value.clone())
//     }
// }

// impl<T: Default> RawVec<T> {
//     pub fn with_size_default(size: usizelloc: A) -> Result<Self, ()> {
//         Self::with_size_f(size, Default::defaultlloc)
//     }
// }

// impl<T> RawVec<T> {
//     pub fn with_size_f<F: Fn() -> T>(size: usize, f: Flloc: A) -> Result<Self, ()> {
//         let layout = Layout::array::<T>(size).map_err(|_| ())?;
//         let ptr = alloc.allocate(layout).map_err(|_| ())?.cast::<T>();

//         for i in 0..size {
//             unsafe {
//                 let ptr = ptr.as_ptr().add(i);
//                 core::ptr::write(ptr, f());
//             }
//         }

//         Ok(Self {
//             ptr: ptr.into(),
//             cap: size,
//             len: size,
//             alloc,
//         })
//     }
// }

impl<T> Deref for RawVec<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { core::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }
}

impl<T> DerefMut for RawVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { core::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
    }
}

impl<'vec, T> IntoIterator for &'vec RawVec<T> {
    type Item = &'vec T;
    type IntoIter = core::slice::Iter<'vec, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'vec, T> IntoIterator for &'vec mut RawVec<T> {
    type Item = &'vec mut T;
    type IntoIter = core::slice::IterMut<'vec, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl core::fmt::Write for RawVec<u8> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.bytes() {
            self.push(b).unwrap();
        }

        Ok(())
    }
}

impl core::fmt::Write for RawVec<char> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.chars() {
            self.push(c).unwrap();
        }

        Ok(())
    }
}
