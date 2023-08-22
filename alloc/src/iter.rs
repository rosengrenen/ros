use core::alloc::{AllocError, Allocator};

pub trait FromIteratorIn<'alloc, T, A: Allocator>: Sized {
    fn from_iter_in<I: IntoIterator<Item = T>>(
        iter: I,
        alloc: &'alloc A,
    ) -> Result<Self, AllocError>;
}

pub trait IteratorCollectIn: Iterator {
    fn collect_in<'alloc, I: FromIteratorIn<'alloc, Self::Item, A>, A: Allocator>(
        self,
        alloc: &'alloc A,
    ) -> Result<I, AllocError>
    where
        Self: Sized,
    {
        FromIteratorIn::from_iter_in(self, alloc)
    }
}

impl<T: Sized, I: Iterator<Item = T>> IteratorCollectIn for I {}
