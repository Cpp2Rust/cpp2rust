// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::{Rc, Weak};

use crate::{
    AsPointer, BidirectionalIterator, ByteRepr, CppIterator, Ptr, UnsafeBidirectionalIterator,
    UnsafeCppIterator, Value,
};

pub trait MapEntry<K> {
    fn key(&self) -> K;
}

impl<K: Clone, V> MapEntry<K> for (Value<K>, Value<V>) {
    fn key(&self) -> K {
        self.0.borrow().clone()
    }
}

impl<K: Clone, V> MapEntry<K> for (K, V) {
    fn key(&self) -> K {
        self.0.clone()
    }
}

pub struct MapNode<T> {
    value: Option<Value<T>>,
    prev: Weak<RefCell<MapNode<T>>>,
    next: Weak<RefCell<MapNode<T>>>,
}

impl<T> MapNode<T> {
    pub(crate) fn header() -> Value<Self> {
        let header = Rc::new(RefCell::new(MapNode {
            value: None,
            prev: Weak::new(),
            next: Weak::new(),
        }));
        {
            let mut node = header.borrow_mut();
            node.prev = Rc::downgrade(&header);
            node.next = Rc::downgrade(&header);
        }
        header
    }

    pub(crate) fn next(node: &Value<Self>) -> Value<Self> {
        node.borrow()
            .next
            .upgrade()
            .expect("ub: increment of an invalid map iterator")
    }

    pub(crate) fn prev(node: &Value<Self>) -> Value<Self> {
        node.borrow()
            .prev
            .upgrade()
            .expect("ub: decrement of an invalid map iterator")
    }

    pub(crate) fn value(node: &Value<Self>) -> Ptr<T> {
        node.borrow()
            .value
            .as_ref()
            .expect("ub: dereference of end iterator")
            .as_pointer()
    }
}

pub struct Map<K, T> {
    nodes: BTreeMap<K, Value<MapNode<T>>>,
    header: Value<MapNode<T>>,
}

impl<K, T> Default for Map<K, T> {
    fn default() -> Self {
        Self {
            nodes: BTreeMap::new(),
            header: MapNode::header(),
        }
    }
}

impl<K: 'static, T: 'static> ByteRepr for Map<K, T> {}

impl<K: Ord + Clone, V: Clone> Clone for Map<K, (Value<K>, Value<V>)> {
    fn clone(&self) -> Self {
        let mut map = Self::default();
        let mut prev = map.header.clone();
        for (key, node) in &self.nodes {
            let value = {
                let node = node.borrow();
                let entry = node
                    .value
                    .as_ref()
                    .expect("map node without a value")
                    .borrow();
                (
                    Rc::new(RefCell::new(entry.0.borrow().clone())),
                    Rc::new(RefCell::new(entry.1.borrow().clone())),
                )
            };
            prev = map.link_after(prev, key.clone(), Rc::new(RefCell::new(value)));
        }
        map
    }
}

impl<K, T> Map<K, T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn begin(&self) -> BidirectionalIterator<T> {
        BidirectionalIterator::Map(MapNode::next(&self.header))
    }

    pub fn end(&self) -> BidirectionalIterator<T> {
        BidirectionalIterator::Map(self.header.clone())
    }
}

impl<K: Ord, T> Map<K, T> {
    fn link_after(
        &mut self,
        prev: Value<MapNode<T>>,
        key: K,
        value: Value<T>,
    ) -> Value<MapNode<T>> {
        let next = MapNode::next(&prev);
        let node = Rc::new(RefCell::new(MapNode {
            value: Some(value),
            prev: Rc::downgrade(&prev),
            next: Rc::downgrade(&next),
        }));
        prev.borrow_mut().next = Rc::downgrade(&node);
        next.borrow_mut().prev = Rc::downgrade(&node);
        self.nodes.insert(key, node.clone());
        node
    }

    pub fn get(&self, key: &K) -> Option<Value<T>> {
        self.nodes.get(key).map(|node| {
            node.borrow()
                .value
                .clone()
                .expect("map node without a value")
        })
    }

    pub fn get_or_insert_with(&mut self, key: K, value: impl FnOnce() -> T) -> Value<T> {
        let prev = match self.nodes.range(..=&key).next_back() {
            Some((k, node)) if k.cmp(&key) == std::cmp::Ordering::Equal => {
                return node
                    .borrow()
                    .value
                    .clone()
                    .expect("map node without a value");
            }
            Some((_, node)) => node.clone(),
            None => self.header.clone(),
        };
        let value = Rc::new(RefCell::new(value()));
        self.link_after(prev, key, value.clone());
        value
    }

    pub fn find(&self, key: &K) -> BidirectionalIterator<T> {
        match self.nodes.get(key) {
            Some(node) => BidirectionalIterator::Map(node.clone()),
            None => self.end(),
        }
    }

    fn unlink(node: &Value<MapNode<T>>) -> Value<MapNode<T>> {
        let prev = MapNode::prev(node);
        let next = MapNode::next(node);
        prev.borrow_mut().next = Rc::downgrade(&next);
        next.borrow_mut().prev = Rc::downgrade(&prev);
        next
    }

    pub fn erase(&mut self, it: &BidirectionalIterator<T>) -> BidirectionalIterator<T>
    where
        T: MapEntry<K>,
    {
        let BidirectionalIterator::Map(node) = it;
        let key = node
            .borrow()
            .value
            .as_ref()
            .expect("ub: erase of end iterator")
            .borrow()
            .key();
        let next = Self::unlink(node);
        self.nodes.remove(&key);
        BidirectionalIterator::Map(next)
    }

    pub fn erase_key(&mut self, key: &K) -> usize {
        match self.nodes.remove(key) {
            Some(node) => {
                Self::unlink(&node);
                1
            }
            None => 0,
        }
    }
}

impl<K: 'static, T: 'static> Map<K, T> {
    pub fn range(map: Ptr<Self>) -> MapRange<T> {
        map.with(|map| MapRange {
            cur: map.begin(),
            end: map.end(),
        })
    }
}

pub struct MapRange<T> {
    cur: BidirectionalIterator<T>,
    end: BidirectionalIterator<T>,
}

impl<T> Iterator for MapRange<T> {
    type Item = BidirectionalIterator<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur == self.end {
            return None;
        }
        Some(self.cur.iter_post_inc())
    }
}

pub struct UnsafeMapNode<T> {
    value: Option<T>,
    prev: *mut UnsafeMapNode<T>,
    next: *mut UnsafeMapNode<T>,
}

impl<T> UnsafeMapNode<T> {
    fn header() -> Box<Self> {
        let mut header = Box::new(UnsafeMapNode {
            value: None,
            prev: std::ptr::null_mut(),
            next: std::ptr::null_mut(),
        });
        let ptr: *mut Self = &mut *header;
        header.prev = ptr;
        header.next = ptr;
        header
    }

    pub(crate) fn next(node: *mut Self) -> *mut Self {
        unsafe { (*node).next }
    }

    pub(crate) fn prev(node: *mut Self) -> *mut Self {
        unsafe { (*node).prev }
    }

    pub(crate) fn value(node: *mut Self) -> *mut T {
        unsafe {
            (*node)
                .value
                .as_mut()
                .expect("ub: dereference of end iterator") as *mut T
        }
    }
}

pub struct UnsafeMap<K, T> {
    nodes: BTreeMap<K, Box<UnsafeMapNode<T>>>,
    header: Box<UnsafeMapNode<T>>,
}

impl<K, T> Default for UnsafeMap<K, T> {
    fn default() -> Self {
        Self {
            nodes: BTreeMap::new(),
            header: UnsafeMapNode::header(),
        }
    }
}

impl<K: Ord + Clone, T: Clone> Clone for UnsafeMap<K, T> {
    fn clone(&self) -> Self {
        let mut map = Self::default();
        let mut prev = map.header_ptr();
        for (key, node) in &self.nodes {
            let value = node.value.clone().expect("map node without a value");
            prev = map.link_after(prev, key.clone(), value);
        }
        map
    }
}

impl<K, T> UnsafeMap<K, T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    fn header_ptr(&self) -> *mut UnsafeMapNode<T> {
        &*self.header as *const UnsafeMapNode<T> as *mut UnsafeMapNode<T>
    }

    pub fn begin(&self) -> UnsafeBidirectionalIterator<T> {
        UnsafeBidirectionalIterator::Map(UnsafeMapNode::next(self.header_ptr()))
    }

    pub fn end(&self) -> UnsafeBidirectionalIterator<T> {
        UnsafeBidirectionalIterator::Map(self.header_ptr())
    }

    /// # Safety
    /// `map` must point to a live map.
    pub unsafe fn range(map: *const Self) -> UnsafeMapRange<T> {
        unsafe {
            UnsafeMapRange {
                cur: (*map).begin(),
                end: (*map).end(),
            }
        }
    }
}

impl<K: Ord, T> UnsafeMap<K, T> {
    fn link_after(
        &mut self,
        prev: *mut UnsafeMapNode<T>,
        key: K,
        value: T,
    ) -> *mut UnsafeMapNode<T> {
        let next = UnsafeMapNode::next(prev);
        let mut node = Box::new(UnsafeMapNode {
            value: Some(value),
            prev,
            next,
        });
        let ptr: *mut UnsafeMapNode<T> = &mut *node;
        unsafe {
            (*prev).next = ptr;
            (*next).prev = ptr;
        }
        self.nodes.insert(key, node);
        ptr
    }

    pub fn get(&self, key: &K) -> Option<*mut T> {
        self.nodes.get(key).map(|node| {
            UnsafeMapNode::value(&**node as *const UnsafeMapNode<T> as *mut UnsafeMapNode<T>)
        })
    }

    pub fn get_or_insert_with(&mut self, key: K, value: impl FnOnce() -> T) -> *mut T {
        let prev = match self.nodes.range(..=&key).next_back() {
            Some((k, node)) => {
                let node = &**node as *const UnsafeMapNode<T> as *mut UnsafeMapNode<T>;
                if k.cmp(&key) == std::cmp::Ordering::Equal {
                    return UnsafeMapNode::value(node);
                }
                node
            }
            None => self.header_ptr(),
        };
        UnsafeMapNode::value(self.link_after(prev, key, value()))
    }

    pub fn find(&self, key: &K) -> UnsafeBidirectionalIterator<T> {
        match self.nodes.get(key) {
            Some(node) => UnsafeBidirectionalIterator::Map(
                &**node as *const UnsafeMapNode<T> as *mut UnsafeMapNode<T>,
            ),
            None => self.end(),
        }
    }

    fn unlink(node: *mut UnsafeMapNode<T>) -> *mut UnsafeMapNode<T> {
        let prev = UnsafeMapNode::prev(node);
        let next = UnsafeMapNode::next(node);
        unsafe {
            (*prev).next = next;
            (*next).prev = prev;
        }
        next
    }

    pub fn erase(&mut self, it: &UnsafeBidirectionalIterator<T>) -> UnsafeBidirectionalIterator<T>
    where
        T: MapEntry<K>,
    {
        let UnsafeBidirectionalIterator::Map(node) = *it;
        let key = unsafe { (*UnsafeMapNode::value(node)).key() };
        let next = Self::unlink(node);
        self.nodes.remove(&key);
        UnsafeBidirectionalIterator::Map(next)
    }

    pub fn erase_key(&mut self, key: &K) -> usize {
        match self.nodes.remove(key) {
            Some(mut node) => {
                Self::unlink(&mut *node);
                1
            }
            None => 0,
        }
    }
}

pub struct UnsafeMapRange<T> {
    cur: UnsafeBidirectionalIterator<T>,
    end: UnsafeBidirectionalIterator<T>,
}

impl<T> Iterator for UnsafeMapRange<T> {
    type Item = UnsafeBidirectionalIterator<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur == self.end {
            return None;
        }
        Some(self.cur.iter_post_inc())
    }
}
