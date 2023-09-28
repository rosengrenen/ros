pub mod linked_list;
pub use self::hash::{Entry, HashMap};

mod hash {
    use super::linked_list::LinkedList;
    use crate::vec::Vec;
    use core::{
        alloc::Allocator,
        hash::{BuildHasher, Hash},
        marker::PhantomData,
    };

    pub struct HashMap<K, V, H, A: Allocator> {
        slots: Vec<LinkedList<(K, V), A>, A>,
        key: PhantomData<K>,
        hasher_builder: H,
    }

    impl<K, V, H, A: Allocator> core::fmt::Debug for HashMap<K, V, H, A> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("HashMap").finish()
        }
    }

    impl<K: Hash + PartialEq, V, H: BuildHasher, A: Allocator + Clone> HashMap<K, V, H, A> {
        pub fn new(hasher_builder: H, alloc: A) -> Self {
            Self {
                slots: Vec::with_size_f(100, || LinkedList::new(alloc.clone()), alloc.clone())
                    .unwrap(),
                key: PhantomData,
                hasher_builder,
            }
        }

        pub fn get(&self, key: &K) -> Option<&V> {
            let hash = self.hasher_builder.hash_one(key) as usize;
            let index = hash % self.slots.len();
            let chain = unsafe { self.slots.get_unchecked(index) };
            for (k, v) in chain.iter() {
                if k == key {
                    return Some(v);
                }
            }

            None
        }

        pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
            let hash = self.hasher_builder.hash_one(key) as usize;
            let index = hash % self.slots.len();
            let chain = unsafe { self.slots.get_unchecked_mut(index) };
            for (k, v) in chain.iter_mut() {
                if k == key {
                    return Some(v);
                }
            }

            None
        }

        pub fn insert(&mut self, key: K, value: V) -> &mut V {
            self.remove(&key);
            let hash = self.hasher_builder.hash_one(&key) as usize;
            let index = hash % self.slots.len();
            let chain = unsafe { self.slots.get_unchecked_mut(index) };
            &mut chain.push((key, value)).1
        }

        pub fn remove(&mut self, key: &K) {
            let hash = self.hasher_builder.hash_one(key) as usize;
            let index = hash % self.slots.len();
            let chain = unsafe { self.slots.get_unchecked_mut(index) };
            chain.retain(|(k, _)| k != key);
        }

        pub fn entry(&mut self, key: K) -> Entry<'_, K, V, H, A> {
            match self.get_mut(&key) {
                Some(_) => Entry::Occupied { key, map: self },
                None => Entry::Vacant { key, map: self },
            }
        }
    }

    pub enum Entry<'map, K, V, H, A: Allocator> {
        Occupied {
            key: K,
            map: &'map mut HashMap<K, V, H, A>,
        },
        Vacant {
            key: K,
            map: &'map mut HashMap<K, V, H, A>,
        },
    }

    impl<'map, K: Hash + PartialEq, V, H: BuildHasher, A: Allocator + Clone> Entry<'map, K, V, H, A> {
        pub fn or_insert(self, value: V) -> &'map mut V {
            match self {
                Entry::Occupied { key, map } => map.get_mut(&key).unwrap(),
                Entry::Vacant { key, map } => map.insert(key, value),
            }
        }
    }
}
