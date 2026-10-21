// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::cell::{Ref, RefCell};
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::ops::{Bound, Deref};
use std::rc::{Rc, Weak};

use crate::{
    AsPointer, BidirectionalIterator, ByteRepr, CppIterator, Ptr, UnsafeBidirectionalIterator,
    UnsafeCppIterator, Value,
};

// A map keeps a single copy of each key, as C++ does, since copying a key may
// have side effects. Its index is ordered by the keys of the entries, which it
// shares with them, and is searched for a borrowed key by comparing it with
// those through this trait.
pub trait MapKey<K> {
    fn key(&self) -> MapKeyRef<'_, K>;
}

pub enum MapKeyRef<'a, K> {
    Shared(Ref<'a, K>),
    Borrowed(&'a K),
}

impl<K> Deref for MapKeyRef<'_, K> {
    type Target = K;

    fn deref(&self) -> &K {
        match self {
            Self::Shared(key) => key,
            Self::Borrowed(key) => key,
        }
    }
}

impl<K> MapKey<K> for K {
    fn key(&self) -> MapKeyRef<'_, K> {
        MapKeyRef::Borrowed(self)
    }
}

impl<K> MapKey<K> for RefCell<K> {
    fn key(&self) -> MapKeyRef<'_, K> {
        MapKeyRef::Shared(self.borrow())
    }
}

impl<K: Ord> Ord for dyn MapKey<K> + '_ {
    fn cmp(&self, other: &Self) -> Ordering {
        (*self.key()).cmp(&*other.key())
    }
}

impl<K: Ord> PartialOrd for dyn MapKey<K> + '_ {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: Ord> PartialEq for dyn MapKey<K> + '_ {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<K: Ord> Eq for dyn MapKey<K> + '_ {}

impl<'a, K: Ord + 'a> std::borrow::Borrow<dyn MapKey<K> + 'a> for Value<K> {
    fn borrow(&self) -> &(dyn MapKey<K> + 'a) {
        &**self
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
    nodes: BTreeMap<Value<K>, Value<MapNode<T>>>,
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
        for node in self.nodes.values() {
            let entry = {
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
            prev = map.link_after(prev, entry);
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

type Entry<K, V> = (Value<K>, Value<V>);
type EntryNode<K, V> = Value<MapNode<Entry<K, V>>>;

impl<K: Ord, V> Map<K, Entry<K, V>> {
    fn link_after(
        &mut self,
        prev: Value<MapNode<Entry<K, V>>>,
        entry: Entry<K, V>,
    ) -> Value<MapNode<Entry<K, V>>> {
        let key = entry.0.clone();
        let next = MapNode::next(&prev);
        let node = Rc::new(RefCell::new(MapNode {
            value: Some(Rc::new(RefCell::new(entry))),
            prev: Rc::downgrade(&prev),
            next: Rc::downgrade(&next),
        }));
        prev.borrow_mut().next = Rc::downgrade(&node);
        next.borrow_mut().prev = Rc::downgrade(&node);
        self.nodes.insert(key, node.clone());
        node
    }

    // The node with the given key, or else the node after which it goes.
    fn search(&self, key: &K) -> Result<EntryNode<K, V>, EntryNode<K, V>> {
        let key: &dyn MapKey<K> = key;
        match self
            .nodes
            .range::<dyn MapKey<K>, _>((Bound::Unbounded, Bound::Included(key)))
            .next_back()
        {
            Some((k, node)) if (&**k as &dyn MapKey<K>) == key => Ok(node.clone()),
            Some((_, node)) => Err(node.clone()),
            None => Err(self.header.clone()),
        }
    }

    fn mapped(node: &Value<MapNode<Entry<K, V>>>) -> Ptr<V> {
        node.borrow()
            .value
            .as_ref()
            .expect("map node without a value")
            .borrow()
            .1
            .as_pointer()
    }

    fn insert_after(&mut self, prev: Value<MapNode<Entry<K, V>>>, key: K) -> Ptr<V>
    where
        V: Default,
    {
        let entry = (
            Rc::new(RefCell::new(key)),
            Rc::new(RefCell::new(V::default())),
        );
        Self::mapped(&self.link_after(prev, entry))
    }

    pub fn get(&self, key: &K) -> Option<Value<Entry<K, V>>> {
        self.nodes.get(key as &dyn MapKey<K>).map(|node| {
            node.borrow()
                .value
                .clone()
                .expect("map node without a value")
        })
    }

    pub fn find(&self, key: &K) -> BidirectionalIterator<Entry<K, V>> {
        match self.nodes.get(key as &dyn MapKey<K>) {
            Some(node) => BidirectionalIterator::Map(node.clone()),
            None => self.end(),
        }
    }

    fn unlink(node: &Value<MapNode<Entry<K, V>>>) -> Value<MapNode<Entry<K, V>>> {
        let prev = MapNode::prev(node);
        let next = MapNode::next(node);
        prev.borrow_mut().next = Rc::downgrade(&next);
        next.borrow_mut().prev = Rc::downgrade(&prev);
        next
    }

}

// The functions that modify a map take a pointer to it, and borrow it after
// their arguments are evaluated, which may access the map.
impl<K: Ord + 'static, V: 'static> Map<K, Entry<K, V>> {
    // operator[] with a key that is copied if inserted.
    pub fn subscript(map: Ptr<Self>, key: &K) -> Ptr<V>
    where
        K: Clone,
        V: Default,
    {
        map.with_mut(|map| match map.search(key) {
            Ok(node) => Self::mapped(&node),
            Err(prev) => map.insert_after(prev, key.clone()),
        })
    }

    // operator[] with a key that is moved, by new_key, if inserted.
    pub fn subscript_move(map: Ptr<Self>, key: Ptr<K>, new_key: impl FnOnce(Ptr<K>) -> K) -> Ptr<V>
    where
        K: ByteRepr,
        V: Default,
    {
        map.with_mut(|map| match key.with(|key| map.search(key)) {
            Ok(node) => Self::mapped(&node),
            Err(prev) => map.insert_after(prev, new_key(key)),
        })
    }

    pub fn erase(
        map: Ptr<Self>,
        it: &BidirectionalIterator<Entry<K, V>>,
    ) -> BidirectionalIterator<Entry<K, V>> {
        let BidirectionalIterator::Map(node) = it;
        let key = node
            .borrow()
            .value
            .as_ref()
            .expect("ub: erase of end iterator")
            .borrow()
            .0
            .clone();
        let next = Self::unlink(node);
        map.with_mut(|map| map.nodes.remove(&key));
        BidirectionalIterator::Map(next)
    }

    pub fn erase_key(map: Ptr<Self>, key: &K) -> usize {
        match map.with_mut(|map| map.nodes.remove(key as &dyn MapKey<K>)) {
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

// The index of an UnsafeMap points to the keys of the entries, which are in
// the nodes, so that each key has a single copy, as in Map.
struct UnsafeMapKey<K>(*const K);

impl<K: Ord> Ord for UnsafeMapKey<K> {
    fn cmp(&self, other: &Self) -> Ordering {
        unsafe { (*self.0).cmp(&*other.0) }
    }
}

impl<K: Ord> PartialOrd for UnsafeMapKey<K> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: Ord> PartialEq for UnsafeMapKey<K> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<K: Ord> Eq for UnsafeMapKey<K> {}

pub struct UnsafeMap<K, T> {
    nodes: BTreeMap<UnsafeMapKey<K>, Box<UnsafeMapNode<T>>>,
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

impl<K: Ord + Clone, V: Clone> Clone for UnsafeMap<K, (K, V)> {
    fn clone(&self) -> Self {
        let mut map = Self::default();
        let mut prev = map.header_ptr();
        for node in self.nodes.values() {
            let entry = node.value.clone().expect("map node without a value");
            prev = map.link_after(prev, entry);
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

type UnsafeEntryNode<K, V> = *mut UnsafeMapNode<(K, V)>;

impl<K: Ord, V> UnsafeMap<K, (K, V)> {
    fn link_after(
        &mut self,
        prev: *mut UnsafeMapNode<(K, V)>,
        entry: (K, V),
    ) -> *mut UnsafeMapNode<(K, V)> {
        let next = UnsafeMapNode::next(prev);
        let mut node = Box::new(UnsafeMapNode {
            value: Some(entry),
            prev,
            next,
        });
        let ptr: *mut UnsafeMapNode<(K, V)> = &mut *node;
        unsafe {
            (*prev).next = ptr;
            (*next).prev = ptr;
        }
        let key = UnsafeMapKey(unsafe { &(*UnsafeMapNode::value(ptr)).0 });
        self.nodes.insert(key, node);
        ptr
    }

    // The node with the given key, or else the node after which it goes.
    fn search(&self, key: &K) -> Result<UnsafeEntryNode<K, V>, UnsafeEntryNode<K, V>> {
        let key = UnsafeMapKey(key);
        match self.nodes.range(..=&key).next_back() {
            Some((k, node)) => {
                let node = &**node as *const UnsafeMapNode<(K, V)> as *mut UnsafeMapNode<(K, V)>;
                if *k == key { Ok(node) } else { Err(node) }
            }
            None => Err(self.header_ptr()),
        }
    }

    fn mapped(node: *mut UnsafeMapNode<(K, V)>) -> *mut V {
        unsafe { &raw mut (*UnsafeMapNode::value(node)).1 }
    }

    // operator[] with a key that is copied if inserted.
    pub fn subscript(&mut self, key: &K) -> *mut V
    where
        K: Clone,
        V: Default,
    {
        match self.search(key) {
            Ok(node) => Self::mapped(node),
            Err(prev) => Self::mapped(self.link_after(prev, (key.clone(), V::default()))),
        }
    }

    /// operator[] with a key that is moved, by new_key, if inserted.
    ///
    /// # Safety
    /// `key` must point to a live key.
    pub unsafe fn subscript_move(&mut self, key: *mut K, new_key: impl FnOnce(*mut K) -> K) -> *mut V
    where
        V: Default,
    {
        match self.search(unsafe { &*key }) {
            Ok(node) => Self::mapped(node),
            Err(prev) => Self::mapped(self.link_after(prev, (new_key(key), V::default()))),
        }
    }

    pub fn get(&self, key: &K) -> Option<*mut (K, V)> {
        self.nodes.get(&UnsafeMapKey(key)).map(|node| {
            UnsafeMapNode::value(&**node as *const UnsafeMapNode<(K, V)> as *mut UnsafeMapNode<(K, V)>)
        })
    }

    pub fn find(&self, key: &K) -> UnsafeBidirectionalIterator<(K, V)> {
        match self.nodes.get(&UnsafeMapKey(key)) {
            Some(node) => UnsafeBidirectionalIterator::Map(
                &**node as *const UnsafeMapNode<(K, V)> as *mut UnsafeMapNode<(K, V)>,
            ),
            None => self.end(),
        }
    }

    fn unlink(node: *mut UnsafeMapNode<(K, V)>) -> *mut UnsafeMapNode<(K, V)> {
        let prev = UnsafeMapNode::prev(node);
        let next = UnsafeMapNode::next(node);
        unsafe {
            (*prev).next = next;
            (*next).prev = prev;
        }
        next
    }

    pub fn erase(
        &mut self,
        it: &UnsafeBidirectionalIterator<(K, V)>,
    ) -> UnsafeBidirectionalIterator<(K, V)> {
        let UnsafeBidirectionalIterator::Map(node) = *it;
        let key = UnsafeMapKey(unsafe { &(*UnsafeMapNode::value(node)).0 });
        let next = Self::unlink(node);
        self.nodes.remove(&key);
        UnsafeBidirectionalIterator::Map(next)
    }

    pub fn erase_key(&mut self, key: &K) -> usize {
        match self.nodes.remove(&UnsafeMapKey(key)) {
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
