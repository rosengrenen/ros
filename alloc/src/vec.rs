use core::alloc::Allocator;

pub struct Vec<T, A: Allocator> {
    ptr: core::ptr::NonNull<T>,
    cap: usize,
    len: usize,
    alloc: A,
}

impl<T, A: Allocator> Vec<T, A> {
    pub fn new(alloc: A) -> Self {
        Self {
            ptr: core::ptr::NonNull::dangling(),
            cap: 0,
            len: 0,
            alloc,
        }
    }
}

impl<T: Clone, A: Allocator> Vec<T, A> {
    pub fn with_elem(value: T, n: usize, alloc: A) -> Result<Self, ()> {
        let layout = core::alloc::Layout::array::<T>(n).map_err(|_| ())?;
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
            ptr,
            cap: layout.size(),
            len: n,
            alloc,
        })
    }
}
