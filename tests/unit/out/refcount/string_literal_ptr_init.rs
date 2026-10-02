extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
pub struct label {
    #[offset(0)]
    #[byte_size(8)]
    pub name: Ptr<u8>,
    #[offset(8)]
    #[byte_size(8)]
    pub probe: FnPtr<fn() -> i32>,
    #[offset(16)]
    pub mask: i32,
}
impl Default for label {
    fn default() -> Self {
        label {
            name: Ptr::<u8>::null(),
            probe: FnPtr::<fn() -> i32>::null(),
            mask: 0_i32,
        }
    }
}
pub fn probe_two_0() -> i32 {
    return 1;
}
thread_local!(
    pub static table_1: Value<Box<[label]>> = Rc::new(RefCell::new(Box::new([
        label {
            name: Ptr::<u8>::from_string_literal(b"first"),
            probe: FnPtr::<fn() -> i32>::null(),
            mask: (1 << 4),
        },
        label {
            name: Ptr::<u8>::from_string_literal(b"second"),
            probe: (FnPtr::<fn() -> i32>::new(probe_two_0)),
            mask: (1 << 5),
        },
    ])));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(
        (((((elem!(
            {
                (*table_1.with(Value::clone).borrow())[(0) as usize]
                    .name
                    .clone()
            },
            0
        )
        .read()) as i32)
            == ('f' as i32)) as i32)
            != 0)
    );
    assert!(
        (((((elem!(
            {
                (*table_1.with(Value::clone).borrow())[(0) as usize]
                    .name
                    .clone()
            },
            4
        )
        .read()) as i32)
            == ('t' as i32)) as i32)
            != 0)
    );
    assert!(
        (((({
            (*table_1.with(Value::clone).borrow())[(0) as usize]
                .probe
                .clone()
        })
        .is_null()) as i32)
            != 0)
    );
    assert!(((({ (*table_1.with(Value::clone).borrow())[(0) as usize].mask } == 16) as i32) != 0));
    assert!(
        (((((elem!(
            {
                (*table_1.with(Value::clone).borrow())[(1) as usize]
                    .name
                    .clone()
            },
            0
        )
        .read()) as i32)
            == ('s' as i32)) as i32)
            != 0)
    );
    assert!(
        (((({
            {
                (*table_1.with(Value::clone).borrow())[(1) as usize]
                    .probe
                    .clone()
            }
            .call()
        }) == 1) as i32)
            != 0)
    );
    assert!(((({ (*table_1.with(Value::clone).borrow())[(1) as usize].mask } == 32) as i32) != 0));
    let tail: Value<Ptr<u8>> = Rc::new(RefCell::new(
        (Ptr::<u8>::from_string_literal(b"ab.cd").offset(2)),
    ));
    assert!((((((elem!((*tail.borrow()), 0).read()) as i32) == ('.' as i32)) as i32) != 0));
    assert!((((((elem!((*tail.borrow()), 1).read()) as i32) == ('c' as i32)) as i32) != 0));
    assert!((((((elem!((*tail.borrow()), 2).read()) as i32) == ('d' as i32)) as i32) != 0));
    let have: Value<i32> = Rc::new(RefCell::new(0));
    let p: Value<AnyPtr> = Rc::new(RefCell::new(if ((*have.borrow()) != 0) {
        ({
            (*table_1.with(Value::clone).borrow())[(0) as usize]
                .name
                .clone()
        })
        .to_any()
    } else {
        Ptr::<u8>::from_string_literal(b"").to_any()
    }));
    assert!(
        (((((elem!(((*p.borrow()).reinterpret_cast::<u8>()), 0).read()) as i32) == ('\0' as i32))
            as i32)
            != 0)
    );
    (*have.borrow_mut()) = 1;
    (*p.borrow_mut()) = if ((*have.borrow()) != 0) {
        ({
            (*table_1.with(Value::clone).borrow())[(0) as usize]
                .name
                .clone()
        })
        .to_any()
    } else {
        Ptr::<u8>::from_string_literal(b"").to_any()
    };
    assert!(
        (((((elem!(((*p.borrow()).reinterpret_cast::<u8>()), 0).read()) as i32) == ('f' as i32))
            as i32)
            != 0)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = table_1.with(|_| ());
}
