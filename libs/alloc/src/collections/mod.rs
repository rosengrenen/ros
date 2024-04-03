pub mod linked_list;
pub use self::hash::Entry;
pub use self::hash::HashMap;

mod hash {
    use core::alloc::Allocator;
    use core::hash::BuildHasher;
    use core::hash::Hash;
    use core::marker::PhantomData;

    use super::linked_list::LinkedList;
    use crate::vec::Vec;

    pub struct HashMap<K, V, H, A: Allocator> {
        slots: Vec<LinkedList<(K, V), A>, A>,
        key: PhantomData<K>,
        hasher_builder: H,
    }

    impl<K, V, H, A> core::fmt::Debug for HashMap<K, V, H, A>
    where
        K: core::fmt::Debug,
        V: core::fmt::Debug,
        A: Allocator,
    {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_map().entries(self.iter()).finish()
        }
    }

    impl<K: Hash + PartialEq, V, H: BuildHasher, A: Allocator + Clone> HashMap<K, V, H, A> {
        pub fn new(hasher_builder: H, alloc: A) -> Self {
            Self {
                slots: Vec::with_size_f(10, || LinkedList::new(alloc.clone()), alloc.clone())
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

        pub fn insert(&mut self, key: K, value: V) {
            self.remove(&key);
            let hash = self.hasher_builder.hash_one(&key) as usize;
            let index = hash % self.slots.len();
            let chain = unsafe { self.slots.get_unchecked_mut(index) };
            chain.push((key, value));
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

    impl<K, V, H, A: Allocator> HashMap<K, V, H, A> {
        pub fn iter(&self) -> Iter<'_, K, V, H, A> {
            Iter {
                index: 0,
                chain_iter: None,
                table: self,
            }
        }
    }

    pub struct Iter<'iter, K, V, H, A: Allocator> {
        index: usize,
        chain_iter: Option<super::linked_list::Iter<'iter, (K, V)>>,
        table: &'iter HashMap<K, V, H, A>,
    }

    impl<'iter, K, V, H, A: Allocator> Iterator for Iter<'iter, K, V, H, A> {
        type Item = (&'iter K, &'iter V);

        fn next(&mut self) -> Option<Self::Item> {
            loop {
                if self.index == self.table.slots.len() {
                    return None;
                }

                match self.chain_iter.as_mut() {
                    Some(chain_iter) => {
                        if let Some(item) = chain_iter.next() {
                            return Some((&item.0, &item.1));
                        } else {
                            self.index += 1;
                            self.chain_iter = None;
                        }
                    }
                    None => {
                        self.chain_iter =
                            Some(unsafe { self.table.slots.get_unchecked(self.index).iter() })
                    }
                };
            }
        }
    }

    impl<'iter, K, V, H, A: Allocator> IntoIterator for &'iter HashMap<K, V, H, A> {
        type Item = (&'iter K, &'iter V);

        type IntoIter = Iter<'iter, K, V, H, A>;

        fn into_iter(self) -> Self::IntoIter {
            self.iter()
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

    impl<'map, K: Hash + PartialEq + Clone, V, H: BuildHasher, A: Allocator + Clone>
        Entry<'map, K, V, H, A>
    {
        pub fn or_insert(self, value: V) -> &'map mut V {
            match self {
                Entry::Occupied { key, map } => map.get_mut(&key).unwrap(),
                Entry::Vacant { key, map } => {
                    map.insert(key.clone(), value);
                    map.get_mut(&key).unwrap()
                }
            }
        }
    }
}
