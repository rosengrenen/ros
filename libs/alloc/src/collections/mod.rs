pub mod linked_list;
pub use self::hash::{Entry, HashMap};

mod hash {
    use crate::vec::Vec;
    use core::{alloc::Allocator, hash::BuildHasher, marker::PhantomData};

    pub struct HashMap<K, V, H, A: Allocator> {
        values: Vec<Option<(K, V)>, A>,
        key: PhantomData<K>,
        hasher_builder: H,
        alloc: A,
    }

    impl<K, V, H, A: Allocator> core::fmt::Debug for HashMap<K, V, H, A> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("HashMap").finish()
        }
    }

    impl<K: core::hash::Hash + core::cmp::PartialEq, V, H: BuildHasher, A: Allocator + Clone>
        HashMap<K, V, H, A>
    {
        pub fn new(hasher_builder: H, alloc: A) -> Self {
            Self {
                values: Vec::new(alloc.clone()),
                key: PhantomData,
                hasher_builder,
                alloc,
            }
        }

        fn get_index(&self, key: &K) -> Option<usize> {
            if self.values.is_empty() {
                return None;
            }

            let table_len = self.values.len();
            let hash = self.hasher_builder.hash_one(key) as usize;
            let start_index = hash % table_len;
            let end_index = (start_index + table_len - 1) % table_len;
            let mut index = start_index;
            while index != end_index {
                if let Some((k, _)) = self.values[index] {
                    if key == &k {
                        return Some(index);
                    }
                }

                index += 1;
            }

            None
        }

        pub fn get(&self, key: &K) -> Option<&V> {
            if let Some(index) = self.get_index(key) {
                return unsafe { self.values.get_unchecked(index) }.map(|(_, v)| &v);
            }

            None
        }

        pub fn insert(&mut self, key: K, value: V) {
            // TODO: remove key first
        }

        pub fn entry(&mut self, key: K) -> Entry<K, V, H, A> {
            Entry { map: self, key }
        }
    }

    pub enum Entry<'map, K, V, H, A: Allocator> {
        Occupied(OccupiedEntry<'map, K, V, H, A>),
        Vacant(VacantEntry<'map, K, V, H, A>),
    }

    impl<'map, K, V, H, A: Allocator> Entry<'map, K, V, H, A> {}

    struct OccupiedEntry<'map, K, V, H, A: Allocator> {
        key: K,
        index: usize,
        map: &'map mut HashMap<K, V, H, A>,
    }

    struct VacantEntry<'map, K, V, H, A: Allocator> {
        key: K,
        map: &'map mut HashMap<K, V, H, A>,
    }
}
