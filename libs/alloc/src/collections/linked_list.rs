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

struct Ends<T> {
    head: NonNull<Node<T>>,
    tail: NonNull<Node<T>>,
}

impl<T> Clone for Ends<T> {
    fn clone(&self) -> Self {
        Self {
            head: self.head,
            tail: self.tail,
        }
    }
}

impl<T> Copy for Ends<T> {}

impl<T> Ends<T> {
    fn new(head: NonNull<Node<T>>, tail: NonNull<Node<T>>) -> Self {
        Self { head, tail }
    }

    fn tail_mut(&mut self) -> &mut Node<T> {
        unsafe { self.tail.as_mut() }
    }
}

impl<T, A: Allocator + Clone> LinkedList<T, A> {
    const NODE_LAYOUT: Layout = Layout::new::<Node<T>>();

    pub fn new(alloc: A) -> Self {
        Self {
            ends: None,
            len: 0,
            alloc,
        }
    }

    pub fn push(&mut self, value: T) -> &mut T {
        self.insert(self.len, value)
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

        self.ends = match self.ends {
            Some(mut ends) => {
                if index == self.len {
                    // Insert back
                    node.prev = Some(ends.tail);
                    ends.tail_mut().next = Some(node_ptr);
                    ends.tail = node_ptr;
                } else {
                    let mut next_node_ptr = self.get_index(index);
                    let next_node = unsafe { next_node_ptr.as_mut() };

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

        let value = self.remove_node(node_ptr);
        self.len -= 1;
        value
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

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            current: self.ends.map(|ends| ends.head),
            marker: PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            current: self.ends.map(|ends| ends.head),
            marker: PhantomData,
        }
    }

    fn remove_node(&mut self, node_ptr: NonNull<Node<T>>) -> T {
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
                self.ends.unwrap().tail = prev_node_ptr;
            }
            (None, Some(mut next_node_ptr)) => {
                let next_node = unsafe { next_node_ptr.as_mut() };
                next_node.prev = None;
                self.ends.unwrap().head = next_node_ptr;
            }
            (None, None) => {
                self.ends = None;
            }
        }

        self.destroy_node(node_ptr)
    }

    fn get_index(&self, index: usize) -> NonNull<Node<T>> {
        if index >= self.len {
            panic!("index >= len");
        }

        let ends = self.ends.as_ref().expect("empty");
        if index < self.len / 2 {
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

    fn create_node(&self, value: T) -> NonNull<Node<T>> {
        let mut node_ptr = self
            .alloc
            .allocate(Self::NODE_LAYOUT)
            .unwrap()
            .cast::<Node<T>>();
        unsafe {
            *node_ptr.as_mut() = Node::new(value);
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
