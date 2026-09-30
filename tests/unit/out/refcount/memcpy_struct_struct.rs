extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct Entry {
    #[offset(0)]
    pub bits: u8,
    #[offset(2)]
    pub value: u16,
}
impl ByteRepr for Entry {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.bits.to_bytes(&mut buf[0..1]);
        self.value.to_bytes(&mut buf[2..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            bits: <u8>::from_bytes(&buf[0..1]),
            value: <u16>::from_bytes(&buf[2..4]),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let table: Value<Box<[Entry]>> = Rc::new(RefCell::new(Box::new([
        Entry {
            bits: 1_u8,
            value: 4369_u16,
        },
        Entry {
            bits: 2_u8,
            value: 8738_u16,
        },
        Entry {
            bits: 3_u8,
            value: 13107_u16,
        },
        Entry {
            bits: 4_u8,
            value: 17476_u16,
        },
        Entry {
            bits: 0_u8,
            value: 0_u16,
        },
        Entry {
            bits: 0_u8,
            value: 0_u16,
        },
        Entry {
            bits: 0_u8,
            value: 0_u16,
        },
        Entry {
            bits: 0_u8,
            value: 0_u16,
        },
    ])));
    let table_size: Value<usize> = Rc::new(RefCell::new(4_usize));
    {
        (((table.as_pointer() as Ptr<Entry>).offset((*table_size.borrow()))) as Ptr<Entry>)
            .to_any()
            .memcpy(
                &(((table.as_pointer() as Ptr<Entry>).offset(0)) as Ptr<Entry>).to_any(),
                (((*table_size.borrow()) as u64).wrapping_mul((4usize as u64)) as usize) as usize,
            );
        (((table.as_pointer() as Ptr<Entry>).offset((*table_size.borrow()))) as Ptr<Entry>).to_any()
    };
    assert!(
        (({ (*table.borrow())[(4) as usize].bits } as i32) == 1)
            && (({ (*table.borrow())[(4) as usize].value } as i32) == 4369)
    );
    assert!(
        (({ (*table.borrow())[(5) as usize].bits } as i32) == 2)
            && (({ (*table.borrow())[(5) as usize].value } as i32) == 8738)
    );
    assert!(
        (({ (*table.borrow())[(6) as usize].bits } as i32) == 3)
            && (({ (*table.borrow())[(6) as usize].value } as i32) == 13107)
    );
    assert!(
        (({ (*table.borrow())[(7) as usize].bits } as i32) == 4)
            && (({ (*table.borrow())[(7) as usize].value } as i32) == 17476)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
