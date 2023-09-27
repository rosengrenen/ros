use core::{alloc::Allocator, marker::PhantomData};

pub use self::hash::{HashMap, Entry};

mod hash {

pub struct HashMap<K,V,A:Allocator> {
  k: PhantomData<K>,
  v: PhantomData<V>,
  alloc: A,
}

impl<K,V,A:Allocator> core::fmt::Debug for HashMap<K,V,A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("HashMap").finish()
    }
}

impl<K,V,A:Allocator> HashMap<K,V,A> {
  pub fn new(alloc: A) -> Self {
    Self {k:PhantomData,v:PhantomData,alloc}
  }
}

// insert

// get

// entry

  pub struct Entry;

// entry.or_insert
}
