// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

// The integers that pointers are cast to, and the pointers that integers are
// cast back to.
//
// Objects have no addresses in the C layout, and so an allocation is given
// some when a pointer into it is first cast to an integer: a range as large as
// its size in C, after that of the previous allocation. The integer of a
// pointer is the address of its allocation plus its byte offset.
//
// An integer is cast to a pointer into the allocation that the address is
// in. There is a byte after each allocation, and so the address one past its
// end is not in the next one. An integer that is in no live allocation, e.g.,
// a sentinel, is cast to a pointer that holds only the address, and which
// can't be accessed.
//
// Allocations are the Values that pointers point into, and so the fields of a
// struct and the elements of an array are at their offset in the C layout of
// the Value that holds them. The elements of a vector that is not a Value,
// e.g., a vector field, are in an allocation of their own, as in C++, which
// is identified by the address of their buffer.
//
// Addresses are never reused, and so the allocations are kept sorted by their
// addresses as they are given them. They are held weakly, and those that have
// been freed are removed once they are as many as those that are not.

use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Weak;

use crate::rc::dangling;
use crate::reinterpret::OriginalAlloc;

// The address of the first allocation, which keeps integers that are not
// casts of pointers, e.g., small offsets, apart from addresses.
const FIRST_ADDRESS: usize = 0x10000;

// Allocations are aligned as malloc aligns them, unless their type is
// over-aligned, so that the integers of pointers have the alignment of their
// pointees.
const ALIGN: usize = 16;

// The number of allocations that are kept before the freed ones are removed
// for the first time.
const MIN_ENTRIES: usize = 64;

// The address of the Value of an allocation, which identifies it while it is
// in the registry, as the weak reference keeps the address from being reused,
// or that of the buffer of a vector that is not a Value, which is checked
// before the allocation is used, as buffers are reused once freed.
type Key = usize;

pub(crate) fn key<A: ?Sized>(value: &Weak<A>) -> Key {
    value.as_ptr() as *const () as usize
}

// An allocation, which spans `len` bytes from `address`.
struct Entry {
    address: usize,
    len: usize,
    key: Key,
    // A weak reference to the Value, and access to its bytes.
    value: OriginalAlloc,
    // The Value that holds a vector that is not a Value.
    root: Option<Weak<dyn Any>>,
}

impl Entry {
    fn is_live(&self) -> bool {
        match &self.root {
            Some(root) => root.strong_count() > 0,
            None => !self.value.is_freed(),
        }
    }

    // Whether the allocation is the one with `key` and byte size `len`: a
    // buffer may have been freed, and its address given to another vector.
    fn is(&self, key: Key, len: usize) -> bool {
        self.is_live() && (self.root.is_none() || self.value.own_buffer() == Some((key, len)))
    }
}

// The allocation that an address is in.
pub(crate) enum Found {
    Live(OriginalAlloc, usize),
    Freed,
    NoObject,
}

struct Escaped {
    // Sorted by address.
    entries: Vec<Entry>,
    // The address and the byte size of the current range of each Value.
    by_key: HashMap<Key, (usize, usize)>,
    next_address: usize,
    // The number of entries at which the freed ones are removed.
    compact_at: usize,
}

thread_local! {
    static ESCAPED: RefCell<Escaped> = RefCell::new(Escaped {
        entries: Vec::new(),
        by_key: HashMap::new(),
        next_address: FIRST_ADDRESS,
        compact_at: MIN_ENTRIES,
    });
}

// The address of byte `offset` of the Value, which is given addresses if it
// has none.
pub(crate) fn address_of(value: OriginalAlloc, offset: usize) -> usize {
    let key = value.key();
    let capacity = value.byte_capacity();
    ESCAPED.with_borrow_mut(|escaped| {
        // A vector whose capacity changed was reallocated, and so it is given
        // a new range.
        if let Some(&(address, len)) = escaped.by_key.get(&key)
            && capacity.is_none_or(|capacity| capacity == len)
            && escaped.entry(address).is(key, len)
        {
            return address.wrapping_add(offset);
        }
        let len = capacity.unwrap_or_else(|| value.total_byte_len());
        let align = value.byte_align().max(ALIGN);
        // The range ends before the next one, so that the address one past
        // the end of an allocation is not in the next one.
        let address = escaped
            .next_address
            .checked_next_multiple_of(align)
            .unwrap_or_else(|| out_of_addresses());
        let end = address
            .checked_add(len)
            .and_then(|end| end.checked_add(1))
            .unwrap_or_else(|| out_of_addresses());
        escaped.next_address = end;
        let root = value.root();
        escaped.push(Entry {
            address,
            len,
            key,
            value,
            root,
        });
        address.wrapping_add(offset)
    })
}

// The allocation that `address` is in, and the offset of the address in it.
pub(crate) fn find(address: usize) -> Found {
    ESCAPED.with_borrow(|escaped| {
        let Some(idx) = escaped
            .entries
            .partition_point(|entry| entry.address <= address)
            .checked_sub(1)
        else {
            return Found::NoObject;
        };
        let entry = &escaped.entries[idx];
        let offset = address - entry.address;
        if offset > entry.len {
            Found::NoObject
        } else if entry.is_live() {
            Found::Live(entry.value.clone(), offset)
        } else {
            Found::Freed
        }
    })
}

// The access through a pointer that holds only `address`, whose allocation
// was freed, or which is in none.
#[cold]
#[inline(never)]
#[track_caller]
pub(crate) fn invalid(address: usize) -> ! {
    match find(address) {
        Found::Freed => dangling(),
        _ => no_object(address),
    }
}

#[cold]
#[inline(never)]
#[track_caller]
fn no_object(address: usize) -> ! {
    panic!("ub: no object at address 0x{address:x}")
}

#[cold]
#[inline(never)]
fn out_of_addresses() -> ! {
    panic!("the addresses of the objects whose pointers are cast to integers ran out")
}

impl Escaped {
    fn push(&mut self, entry: Entry) {
        if self.entries.len() >= self.compact_at {
            self.compact();
        }
        self.by_key.insert(entry.key, (entry.address, entry.len));
        self.entries.push(entry);
    }

    // The allocation at `address`, which is in the registry.
    fn entry(&self, address: usize) -> &Entry {
        let idx = self
            .entries
            .binary_search_by_key(&address, |entry| entry.address)
            .unwrap();
        &self.entries[idx]
    }

    // Removes the allocations that have been freed, and the old ranges of
    // vectors, which takes constant time per entry, on average.
    fn compact(&mut self) {
        let by_key = &mut self.by_key;
        self.entries.retain(|entry| {
            let current = by_key
                .get(&entry.key)
                .is_some_and(|&(address, _)| address == entry.address);
            if entry.is_live() {
                return current;
            }
            if current {
                by_key.remove(&entry.key);
            }
            false
        });
        self.compact_at = (2 * self.entries.len()).max(MIN_ENTRIES);
        if self.entries.capacity() > 2 * self.compact_at {
            self.entries.shrink_to(self.compact_at);
            self.by_key.shrink_to(self.compact_at);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rc::Ptr;

    fn entries() -> usize {
        ESCAPED.with_borrow(|escaped| escaped.entries.len())
    }

    #[test]
    fn freed_allocations_are_removed() {
        let ptrs: Vec<Ptr<u32>> = (0..1000)
            .map(|i| Ptr::alloc_array(vec![i; 3].into_boxed_slice()))
            .collect();
        let addresses: Vec<usize> = ptrs.iter().map(|p| p.to_int()).collect();
        assert!(addresses.is_sorted());
        for (i, &address) in addresses.iter().enumerate() {
            let p = Ptr::<u32>::from_int(address + 8);
            assert_eq!(p.read(), i as u32);
        }
        for p in ptrs.iter().step_by(2) {
            p.delete();
        }
        // Allocations that are freed out of order are removed as others are
        // added, without lookups.
        for i in 0..10000 {
            let p = Ptr::alloc_array(vec![i; 3].into_boxed_slice());
            p.to_int();
            p.delete();
        }
        assert!(entries() <= 2 * 500 + MIN_ENTRIES);
        for (i, &address) in addresses.iter().enumerate().skip(1).step_by(2) {
            assert_eq!(Ptr::<u32>::from_int(address).read(), i as u32);
        }
        for p in ptrs.iter().skip(1).step_by(2) {
            p.delete();
        }
    }

    #[test]
    fn one_past_the_end_is_not_the_next_allocation() {
        let a = Ptr::alloc_array(vec![1u32; 4].into_boxed_slice());
        let b = Ptr::alloc_array(vec![2u32; 4].into_boxed_slice());
        let end = a.to_int() + 16;
        assert!(b.to_int() > end);
        let p = Ptr::<u32>::from_int(end);
        assert_eq!((&p - 1usize).read(), 1);
        assert!(p == a.offset(4));
        assert!(p != b);
        a.delete();
        b.delete();
    }

    #[test]
    fn integers_in_no_object() {
        let a = Ptr::alloc_array(vec![1u32; 4].into_boxed_slice());
        let p = Ptr::<u32>::from_int(0x42);
        assert_eq!(p.to_int(), 0x42);
        assert_eq!((&p + 1usize).to_int(), 0x46);
        assert!(p != a && p != Ptr::null());
        assert!(Ptr::null() < p && p < a);
        assert!(p == Ptr::<u32>::from_int(0x42));
        // The gap after an object is in no object.
        let gap = Ptr::<u8>::from_int(a.to_int() + 17);
        assert!(gap != a.reinterpret_cast::<u8>().offset(17));
        a.delete();
    }

    #[test]
    #[should_panic(expected = "no object")]
    fn access_in_no_object() {
        Ptr::<u32>::from_int(0x42).read();
    }

    #[test]
    #[should_panic(expected = "dangling")]
    fn access_in_freed_object() {
        let a = Ptr::alloc_array(vec![1u32; 4].into_boxed_slice());
        let p = Ptr::<u32>::from_int(a.to_int());
        a.delete();
        p.read();
    }

    #[test]
    #[should_panic(expected = "ran out")]
    fn addresses_run_out() {
        ESCAPED.with_borrow_mut(|escaped| escaped.next_address = usize::MAX - 8);
        Ptr::alloc_array(vec![0u32; 4].into_boxed_slice()).to_int();
    }
}
