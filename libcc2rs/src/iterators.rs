// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::{
    PostfixDec, PostfixInc, PrefixDec, PrefixInc, Ptr, UnsafePostfixDec, UnsafePostfixInc,
    UnsafePrefixDec, UnsafePrefixInc, Value,
};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::ops::Bound;
use std::rc::Rc;

pub trait MapAccess: Clone + Default {
    type Key: Ord + Clone;
    type Value;
    fn with<R>(&self, f: impl FnOnce(&BTreeMap<Self::Key, Self::Value>) -> R) -> R;
    fn with_mut<R>(&self, f: impl FnOnce(&mut BTreeMap<Self::Key, Self::Value>) -> R) -> R;
}

impl<K: Ord + Clone + 'static, V: 'static> MapAccess for Ptr<BTreeMap<K, V>> {
    type Key = K;
    type Value = V;

    fn with<R>(&self, f: impl FnOnce(&BTreeMap<K, V>) -> R) -> R {
        Ptr::with(self, f)
    }

    fn with_mut<R>(&self, f: impl FnOnce(&mut BTreeMap<K, V>) -> R) -> R {
        Ptr::with_mut(self, f)
    }
}

impl<K: Ord + Clone, V> MapAccess for *const BTreeMap<K, V> {
    type Key = K;
    type Value = V;

    fn with<R>(&self, f: impl FnOnce(&BTreeMap<K, V>) -> R) -> R {
        unsafe { f(&**self) }
    }

    fn with_mut<R>(&self, f: impl FnOnce(&mut BTreeMap<K, V>) -> R) -> R {
        unsafe { f(&mut *(*self as *mut BTreeMap<K, V>)) }
    }
}

pub struct MapIter<K, MapRef> {
    map: MapRef,
    key: Option<K>,
}

pub type RefcountMapIter<K, V> = MapIter<K, Ptr<BTreeMap<K, Value<V>>>>;
pub type UnsafeMapIterator<K, V> = MapIter<K, *const BTreeMap<K, Box<V>>>;

impl<K: Clone, MapRef: Clone> Clone for MapIter<K, MapRef> {
    fn clone(&self) -> Self {
        Self {
            map: self.map.clone(),
            key: self.key.clone(),
        }
    }
}

impl<K: PartialEq, MapRef> PartialEq for MapIter<K, MapRef> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<K: Ord + Clone, MapRef: MapAccess<Key = K>> MapIter<K, MapRef> {
    pub fn null() -> Self {
        Self {
            map: Default::default(),
            key: None,
        }
    }

    pub fn begin(map: MapRef) -> Self {
        let first_key = map.with(|m| m.keys().next().cloned());
        Self {
            map,
            key: first_key,
        }
    }

    pub fn end(map: MapRef) -> Self {
        Self { map, key: None }
    }

    pub fn find_key(map: MapRef, key: &K) -> Self {
        if map.with(|m| m.contains_key(key)) {
            Self {
                map,
                key: Some(key.clone()),
            }
        } else {
            Self::end(map)
        }
    }

    pub fn is_end(&self) -> bool {
        self.key.is_none()
    }

    pub fn key(&self) -> &Option<K> {
        &self.key
    }

    pub fn inc(&mut self) {
        self.key = match &self.key {
            Some(k) => self.map.with(|map| {
                map.range((Bound::Excluded(k), Bound::Unbounded))
                    .next()
                    .map(|(k, _)| k.clone())
            }),
            None => panic!("ub: increment past end"),
        };
    }

    pub fn dec(&mut self) {
        self.key = match &self.key {
            Some(k) => self.map.with(|map| {
                map.range((Bound::Unbounded, Bound::Excluded(k)))
                    .next_back()
                    .map(|(k, _)| k.clone())
            }),
            None => self.map.with(|map| map.keys().next_back().cloned()),
        };
    }

    pub fn erase(map: MapRef, iter: &Self) -> Self {
        let key = iter
            .key
            .as_ref()
            .expect("ub: erase of end iterator")
            .clone();
        let next_key = map.with(|m| {
            m.range((Bound::Excluded(&key), Bound::Unbounded))
                .next()
                .map(|(k, _)| k.clone())
        });
        map.with_mut(|m| m.remove(&key));
        Self { map, key: next_key }
    }
}

pub trait MapIterator {
    type First;
    type Second;
    fn first(&self) -> Self::First;
    fn second(&self) -> Self::Second;
}

impl<K: Ord + Clone + 'static, V: 'static> MapIterator for RefcountMapIter<K, V> {
    type First = Value<K>;
    type Second = Value<V>;

    fn first(&self) -> Value<K> {
        let key = self.key.as_ref().expect("ub: dereference of end iterator");
        Rc::new(RefCell::new(key.clone()))
    }

    fn second(&self) -> Value<V> {
        let key = self.key.as_ref().expect("ub: dereference of end iterator");
        self.map
            .with(|m| m.get(key).expect("ub: key not found in map").clone())
    }
}

impl<K: Ord + Clone, V> MapIterator for UnsafeMapIterator<K, V> {
    type First = *const K;
    type Second = *mut V;

    fn first(&self) -> *const K {
        self.key.as_ref().expect("ub: dereference of end iterator") as *const K
    }

    fn second(&self) -> *mut V {
        unsafe {
            let map_mut = self.map as *mut BTreeMap<K, Box<V>>;
            let key = self.key.as_ref().expect("ub: dereference of end iterator");
            (*map_mut)
                .get_mut(key)
                .expect("ub: key not found in map")
                .as_mut() as *mut V
        }
    }
}

impl<K: Ord + Clone, MapRef: MapAccess<Key = K>> Iterator for MapIter<K, MapRef> {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_end() {
            return None;
        }
        let snapshot = self.clone();
        self.inc();
        Some(snapshot)
    }
}

impl<K: Ord + Clone, MapRef: MapAccess<Key = K>> PrefixInc for MapIter<K, MapRef> {
    fn prefix_inc(&mut self) -> Self {
        self.inc();
        self.clone()
    }
}

impl<K: Ord + Clone, MapRef: MapAccess<Key = K>> PostfixInc for MapIter<K, MapRef> {
    fn postfix_inc(&mut self) -> Self {
        let ret = self.clone();
        self.inc();
        ret
    }
}

impl<K: Ord + Clone, MapRef: MapAccess<Key = K>> PrefixDec for MapIter<K, MapRef> {
    fn prefix_dec(&mut self) -> Self {
        self.dec();
        self.clone()
    }
}

impl<K: Ord + Clone, MapRef: MapAccess<Key = K>> PostfixDec for MapIter<K, MapRef> {
    fn postfix_dec(&mut self) -> Self {
        let ret = self.clone();
        self.dec();
        ret
    }
}

pub trait UnsafeIterator:
    UnsafePrefixInc + UnsafePostfixInc + UnsafePrefixDec + UnsafePostfixDec + Default + PartialEq
{
    fn offset(&self, n: isize) -> Self;
}
