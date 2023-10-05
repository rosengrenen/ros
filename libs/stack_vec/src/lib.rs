#![no_std]

use core::fmt::Debug;
use core::mem::MaybeUninit;
use core::ops::{Index, IndexMut};

pub struct StackVec<const N: usize, T> {
    array: [MaybeUninit<T>; N],
    len: usize,
}

impl<const N: usize, T: Debug> Debug for StackVec<N, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut list = f.debug_list();
        for item in &self.array[..self.len] {
            list.entry(unsafe { item.assume_init_ref() });
        }

        list.finish()
    }
}

impl<const N: usize, T> StackVec<N, T> {
    const UNINIT: MaybeUninit<T> = MaybeUninit::uninit();

    pub fn new() -> Self {
        Self {
            array: [Self::UNINIT; N],
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.array.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn push(&mut self, value: T) {
        if self.len == self.capacity() {
            panic!("len == cap");
        }

        self.array[self.len] = MaybeUninit::new(value);
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        self.len -= 1;
        let value = unsafe { self.array[self.len].assume_init_read() };
        self.array[self.len] = MaybeUninit::uninit();
        Some(value)
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }

        Some(unsafe { self.array[index].assume_init_ref() })
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len {
            return None;
        }

        Some(unsafe { self.array[index].assume_init_mut() })
    }

    pub fn iter(&self) -> Iter<'_, N, T> {
        Iter {
            vec: self,
            index: 0,
        }
    }
}

impl<const N: usize, T> Index<usize> for StackVec<N, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len {
            panic!("index >= len");
        }

        self.get(index).unwrap()
    }
}

impl<const N: usize, T> IndexMut<usize> for StackVec<N, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.len {
            panic!("index >= len");
        }

        self.get_mut(index).unwrap()
    }
}

pub struct Iter<'iter, const N: usize, T> {
    vec: &'iter StackVec<N, T>,
    index: usize,
}

impl<'iter, const N: usize, T> Iterator for Iter<'iter, N, T> {
    type Item = &'iter T;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.vec.get(self.index);
        self.index += 1;
        item
    }
}

// Lifetime issues, not sure how to fix it
// pub struct IterMut<'iter, const N: usize, T> {
//     vec: &'iter mut StackVec<N, T>,
//     index: usize,
// }

// impl<'iter, const N: usize, T> Iterator for IterMut<'iter, N, T> {
//     type Item = &'iter mut T;

//     fn next(&mut self) -> Option<Self::Item> {
//         let item = self.vec.get_mut(self.index);
//         self.index += 1;
//         item
//     }
// }
