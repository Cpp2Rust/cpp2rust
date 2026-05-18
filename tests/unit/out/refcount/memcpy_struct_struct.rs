extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Entry {
    pub bits: Value<u8>,
    pub value: Value<u16>,
}
impl Clone for Entry {
    fn clone(&self) -> Self {
        let mut this = Self {
            bits: Rc::new(RefCell::new((*self.bits.borrow()))),
            value: Rc::new(RefCell::new((*self.value.borrow()))),
        };
        this
    }
}
impl ByteRepr for Entry {
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.bits.borrow()).to_bytes(&mut buf[0..1]);
        (*self.value.borrow()).to_bytes(&mut buf[2..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            bits: Rc::new(RefCell::new(<u8>::from_bytes(&buf[0..1]))),
            value: Rc::new(RefCell::new(<u16>::from_bytes(&buf[2..4]))),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let table: Value<Box<[Entry]>> = Rc::new(RefCell::new(Box::new([
        Entry {
            bits: Rc::new(RefCell::new(1_u8)),
            value: Rc::new(RefCell::new(4369_u16)),
        },
        Entry {
            bits: Rc::new(RefCell::new(2_u8)),
            value: Rc::new(RefCell::new(8738_u16)),
        },
        Entry {
            bits: Rc::new(RefCell::new(3_u8)),
            value: Rc::new(RefCell::new(13107_u16)),
        },
        Entry {
            bits: Rc::new(RefCell::new(4_u8)),
            value: Rc::new(RefCell::new(17476_u16)),
        },
        Entry {
            bits: Rc::new(RefCell::new(0_u8)),
            value: Rc::new(RefCell::new(0_u16)),
        },
        Entry {
            bits: Rc::new(RefCell::new(0_u8)),
            value: Rc::new(RefCell::new(0_u16)),
        },
        Entry {
            bits: Rc::new(RefCell::new(0_u8)),
            value: Rc::new(RefCell::new(0_u16)),
        },
        Entry {
            bits: Rc::new(RefCell::new(0_u8)),
            value: Rc::new(RefCell::new(0_u16)),
        },
    ])));
    let table_size: Value<u64> = Rc::new(RefCell::new(4_u64));
    {
        (((table.as_pointer() as Ptr<Entry>).offset((*table_size.borrow()) as isize))
            as Ptr<Entry>)
            .to_any()
            .memcpy(
                &(((table.as_pointer() as Ptr<Entry>).offset(0 as isize)) as Ptr<Entry>).to_any(),
                (*table_size.borrow()).wrapping_mul(::std::mem::size_of::<Entry>() as u64 as u64)
                    as usize,
            );
        (((table.as_pointer() as Ptr<Entry>).offset((*table_size.borrow()) as isize)) as Ptr<Entry>)
            .to_any()
            .clone()
    };
    assert!(
        (((*(*table.borrow())[(4) as usize].bits.borrow()) as i32) == 1)
            && (((*(*table.borrow())[(4) as usize].value.borrow()) as i32) == 4369)
    );
    assert!(
        (((*(*table.borrow())[(5) as usize].bits.borrow()) as i32) == 2)
            && (((*(*table.borrow())[(5) as usize].value.borrow()) as i32) == 8738)
    );
    assert!(
        (((*(*table.borrow())[(6) as usize].bits.borrow()) as i32) == 3)
            && (((*(*table.borrow())[(6) as usize].value.borrow()) as i32) == 13107)
    );
    assert!(
        (((*(*table.borrow())[(7) as usize].bits.borrow()) as i32) == 4)
            && (((*(*table.borrow())[(7) as usize].value.borrow()) as i32) == 17476)
    );
    return 0;
}
