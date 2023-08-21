use core::alloc::{AllocError, Allocator};

pub trait FromIteratorIn<T, A: Allocator>: Sized {
    fn from_iter_in<I: IntoIterator<Item = T>>(iter: I, alloc: A) -> Result<Self, AllocError>;
}

pub trait IteratorCollectIn: Iterator {
    fn collect_in<I: FromIteratorIn<Self::Item, A>, A: Allocator>(
        self,
        alloc: A,
    ) -> Result<I, AllocError>
    where
        Self: Sized,
    {
        FromIteratorIn::from_iter_in(self, alloc)
    }
}

impl<T: Sized, I: Iterator<Item = T>> IteratorCollectIn for I {}
