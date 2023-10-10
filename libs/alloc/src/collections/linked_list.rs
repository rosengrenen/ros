use core::{
    alloc::{Allocator, Layout},
    marker::PhantomData,
    ptr::NonNull,
};

pub struct LinkedList<T, A: Allocator> {
    ends: Option<Ends<T>>,
    len: usize,
    alloc: A,
}

impl<T: core::fmt::Debug, A: Allocator> core::fmt::Debug for LinkedList<T, A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

#[derive(Debug)]
struct Ends<T> {
    head: NonNull<Node<T>>,
    tail: NonNull<Node<T>>,
}

impl<T> Ends<T> {
    fn new(head: NonNull<Node<T>>, tail: NonNull<Node<T>>) -> Self {
        Self { head, tail }
    }

    fn tail_mut(&mut self) -> &mut Node<T> {
        unsafe { self.tail.as_mut() }
    }
}

impl<T> Clone for Ends<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Ends<T> {}

impl<T, A: Allocator> LinkedList<T, A> {
    const NODE_LAYOUT: Layout = Layout::new::<Node<T>>();

    pub fn new(alloc: A) -> Self {
        Self {
            ends: None,
            len: 0,
            alloc,
        }
    }

    pub fn push(&mut self, value: T) {
        let mut node_ptr = self.create_node(value);
        let node = unsafe { node_ptr.as_mut() };
        self.len += 1;
        match &mut self.ends {
            Some(ends) => {
                ends.tail_mut().next = Some(node_ptr);
                node.prev = Some(ends.tail);
                ends.tail = node_ptr;
            }
            None => self.ends = Some(Ends::new(node_ptr, node_ptr)),
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            Some(self.remove(self.len - 1))
        }
    }

    pub fn push_front(&mut self, value: T) -> &mut T {
        self.insert(0, value)
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            Some(self.remove(0))
        }
    }

    pub fn insert(&mut self, index: usize, value: T) -> &mut T {
        let mut node_ptr = self.create_node(value);
        let node = unsafe { node_ptr.as_mut() };

        let mut next_node_ptr = self.get_index(index);
        let next_node = unsafe { next_node_ptr.as_mut() };
        self.ends = match &mut self.ends {
            Some(mut ends) => {
                node.prev = next_node.prev;
                node.next = Some(next_node_ptr);
                next_node.prev = Some(node_ptr);

                // Check if new head needs to be set
                match node.prev {
                    Some(mut prev_node_ptr) => {
                        let prev_node = unsafe { prev_node_ptr.as_mut() };
                        prev_node.next = Some(node_ptr);
                    }
                    None => {
                        ends.head = node_ptr;
                    }
                }

                Some(ends)
            }
            None => Some(Ends::new(node_ptr, node_ptr)),
        };
        self.len += 1;
        unsafe { &mut node_ptr.as_mut().value }
    }

    pub fn remove(&mut self, index: usize) -> T {
        let node_ptr = self.get_index(index);
        self.remove_node(node_ptr)
    }

    pub fn retain<F: Fn(&T) -> bool>(&mut self, f: F) {
        if let Some(ends) = self.ends {
            let mut node_ptr = ends.head;
            loop {
                let node = unsafe { node_ptr.as_mut() };
                let next_node_ptr = node.next;
                if !f(&node.value) {
                    self.remove_node(node_ptr);
                }

                match next_node_ptr {
                    Some(next_node_ptr) => node_ptr = next_node_ptr,
                    None => return,
                }
            }
        }
    }

    pub fn clear(&mut self) {
        while !self.is_empty() {
            self.remove(0);
        }
    }

    pub fn get(&self, index: usize) -> &T {
        let node_ptr = self.get_index(index);
        unsafe { &node_ptr.as_ref().value }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn get_index(&self, index: usize) -> NonNull<Node<T>> {
        if index >= self.len {
            panic!("index >= len");
        }

        let ends = self.ends.as_ref().expect("empty");
        if index <= self.len / 2 {
            let mut node = ends.head;
            for _ in 0..index {
                node = unsafe { node.as_ref().next.unwrap() };
            }

            node
        } else {
            let mut node = ends.tail;
            for _ in index..(self.len - 1) {
                node = unsafe { node.as_ref().prev.unwrap() };
            }

            node
        }
    }

    fn remove_node(&mut self, node_ptr: NonNull<Node<T>>) -> T {
        self.len -= 1;
        let node = unsafe { node_ptr.as_ref() };
        match (node.prev, node.next) {
            (Some(mut prev_node_ptr), Some(mut next_node_ptr)) => {
                let prev_node = unsafe { prev_node_ptr.as_mut() };
                let next_node = unsafe { next_node_ptr.as_mut() };
                prev_node.next = Some(next_node_ptr);
                next_node.prev = Some(prev_node_ptr);
            }
            (Some(mut prev_node_ptr), None) => {
                let prev_node = unsafe { prev_node_ptr.as_mut() };
                prev_node.next = None;
                match &mut self.ends {
                    Some(ends) => ends.tail = prev_node_ptr,
                    None => unreachable!(),
                }
            }
            (None, Some(mut next_node_ptr)) => {
                let next_node = unsafe { next_node_ptr.as_mut() };
                next_node.prev = None;
                match &mut self.ends {
                    Some(ends) => ends.head = next_node_ptr,
                    None => unreachable!(),
                }
            }
            (None, None) => {
                self.ends = None;
            }
        }

        self.destroy_node(node_ptr)
    }

    fn create_node(&self, value: T) -> NonNull<Node<T>> {
        let node_ptr = self
            .alloc
            .allocate(Self::NODE_LAYOUT)
            .unwrap()
            .cast::<Node<T>>();
        unsafe {
            node_ptr.as_ptr().write(Node::new(value));
        }

        node_ptr
    }

    fn destroy_node(&self, node_ptr: NonNull<Node<T>>) -> T {
        let value = unsafe { node_ptr.as_ptr().read().value };
        unsafe {
            self.alloc.deallocate(node_ptr.cast(), Self::NODE_LAYOUT);
        }
        value
    }
}

impl<T, A: Allocator> LinkedList<T, A> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            current: self.ends.as_ref().map(|ends| ends.head),
            marker: PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            current: self.ends.as_ref().map(|ends| ends.head),
            marker: PhantomData,
        }
    }
}

impl<T, A: Allocator> Drop for LinkedList<T, A> {
    fn drop(&mut self) {
        self.clear()
    }
}

pub struct Iter<'iter, T> {
    current: Option<NonNull<Node<T>>>,
    marker: PhantomData<&'iter T>,
}

impl<'iter, T> Iterator for Iter<'iter, T> {
    type Item = &'iter T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node_ptr| {
            let node = unsafe { node_ptr.as_ref() };
            self.current = node.next;
            &node.value
        })
    }
}

pub struct IterMut<'iter, T> {
    current: Option<NonNull<Node<T>>>,
    marker: PhantomData<&'iter mut T>,
}

impl<'iter, T> Iterator for IterMut<'iter, T> {
    type Item = &'iter mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|mut node_ptr| {
            let node = unsafe { node_ptr.as_mut() };
            self.current = node.next;
            &mut node.value
        })
    }
}

#[derive(Debug)]
struct Node<T> {
    value: T,
    prev: Option<NonNull<Node<T>>>,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            prev: None,
            next: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_back() {
        let mut list = LinkedList::<u64, _>::new(&std::alloc::Global);
        list.push(1);
        list.push(2);
        assert_eq!(list.get(0), &1);
        assert_eq!(list.get(1), &2);
        assert_eq!(list.iter().count(), 2);
    }

    #[test]
    fn remove() {
        let mut list = LinkedList::<u64, _>::new(&std::alloc::Global);
        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);
        list.push(5);
        list.remove(2);
        list.remove(0);
        assert_eq!(list.get(0), &2);
        assert_eq!(list.get(1), &4);
        assert_eq!(list.get(2), &5);
        assert_eq!(list.iter().count(), 3);
    }

    #[test]
    fn clear() {
        let mut list = LinkedList::<u64, _>::new(&std::alloc::Global);
        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);
        list.push(5);
        list.clear();
        assert!(list.is_empty());
        assert_eq!(list.iter().count(), 0);
    }

    #[test]
    fn insert() {
        let mut list = LinkedList::<u64, _>::new(&std::alloc::Global);
        list.push(1);
        list.push(2);
        list.push(3);
        list.insert(0, 4);
        list.insert(3, 5);
        assert_eq!(list.get(0), &4);
        assert_eq!(list.get(1), &1);
        assert_eq!(list.get(2), &2);
        assert_eq!(list.get(3), &5);
        assert_eq!(list.get(4), &3);
        assert_eq!(list.iter().count(), 5);
    }

    #[test]
    fn retain() {
        let mut list = LinkedList::<u64, _>::new(&std::alloc::Global);
        list.push(1);
        list.push(2);
        list.push(3);
        list.insert(0, 4);
        list.insert(3, 5);
        list.retain(|e| *e % 2 == 0);
        assert_eq!(list.get(0), &4);
        assert_eq!(list.get(1), &2);
        assert_eq!(list.iter().count(), 2);
    }
}
