use core::{
    cell::UnsafeCell,
    fmt,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
};

pub struct Mutex<T> {
    value: UnsafeCell<T>,
    locked: AtomicBool,
}

impl<T: fmt::Debug> fmt::Debug for Mutex<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.locked.load(Ordering::Relaxed) {
            true => f.debug_tuple("Locked").field(&self.value).finish(),
            false => f
                .debug_tuple("Free")
                .field(unsafe { &*self.value.get() })
                .finish(),
        }
    }
}

impl<T> Mutex<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            locked: AtomicBool::new(false),
        }
    }

    pub fn lock(&self) -> MutexGuard<T> {
        while self
            .locked
            .swap(true, core::sync::atomic::Ordering::Acquire)
            == true
        {}
        MutexGuard { mutex: self }
    }

    fn release(&self) {
        if self
            .locked
            .swap(false, core::sync::atomic::Ordering::AcqRel)
            == false
        {
            panic!("already released");
        }
    }
}

pub struct MutexGuard<'mutex, T> {
    mutex: &'mutex Mutex<T>,
}

impl<'mutex, T> Deref for MutexGuard<'mutex, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mutex.value.get() }
    }
}

impl<'mutex, T> DerefMut for MutexGuard<'mutex, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.mutex.value.get() }
    }
}

impl<'mutex, T> Drop for MutexGuard<'mutex, T> {
    fn drop(&mut self) {
        self.mutex.release()
    }
}
