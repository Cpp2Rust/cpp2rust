extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct packed {
    #[offset(0)]
    pub a: i32,
    #[offset(4)]
    pub b: u8,
    #[offset(5)]
    pub c: u8,
    #[offset(6)]
    pub d: i16,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(12)]
pub struct reordered {
    #[offset(0)]
    pub a: u8,
    #[offset(4)]
    pub b: i32,
    #[offset(8)]
    pub c: u8,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct tail {
    #[offset(0)]
    pub a: u8,
    #[offset(8)]
    pub b: f64,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(24)]
pub struct nested {
    #[offset(0)]
    #[byte_size(16)]
    pub t: tail,
    #[offset(16)]
    pub c: u8,
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct array {
    #[offset(0)]
    #[byte_size(3)]
    pub name: Value<Box<[u8]>>,
    #[offset(4)]
    pub x: i32,
}
impl Clone for array {
    fn clone(&self) -> Self {
        Self {
            name: Rc::new(RefCell::new((*self.name.borrow()).clone())),
            x: self.x.clone(),
        }
    }
}
impl Default for array {
    fn default() -> Self {
        array {
            name: Rc::new(RefCell::new((0..3).map(|_| 0_u8).collect::<Box<[u8]>>())),
            x: 0_i32,
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new((0..64).map(|_| 0_u8).collect::<Box<[u8]>>()));
    let p: Value<Box<[packed]>> = Rc::new(RefCell::new(Box::new([
        packed {
            a: 1,
            b: 2_u8,
            c: 3_u8,
            d: 4_i16,
        },
        packed {
            a: 5,
            b: 6_u8,
            c: 7_u8,
            d: 8_i16,
        },
    ])));
    {
        ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any().memcpy(
            &((p.as_pointer() as Ptr<packed>) as Ptr<packed>).to_any(),
            16usize as usize,
        );
        ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any()
    };
    assert!(
        (((((*buf.borrow())[((8usize as usize).wrapping_add(4_usize)) as usize] as i32) == 6)
            as i32)
            != 0)
    );
    let p2: Value<Box<[packed]>> = Rc::new(RefCell::new(
        (0..2)
            .map(|_| <packed>::default())
            .collect::<Box<[packed]>>(),
    ));
    {
        ((p2.as_pointer() as Ptr<packed>) as Ptr<packed>)
            .to_any()
            .memcpy(
                &((buf.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any(),
                16usize as usize,
            );
        ((p2.as_pointer() as Ptr<packed>) as Ptr<packed>).to_any()
    };
    assert!(
        (((((((((((({ (*p2.borrow())[(1) as usize].a } == 5) as i32) != 0)
            && (((({ (*p2.borrow())[(1) as usize].b } as i32) == 6) as i32) != 0))
            as i32)
            != 0)
            && (((({ (*p2.borrow())[(1) as usize].c } as i32) == 7) as i32) != 0))
            as i32)
            != 0)
            && (((({ (*p2.borrow())[(1) as usize].d } as i32) == 8) as i32) != 0))
            as i32)
            != 0)
    );
    let r: Value<Box<[reordered]>> = Rc::new(RefCell::new(Box::new([
        reordered {
            a: 1_u8,
            b: 2,
            c: 3_u8,
        },
        reordered {
            a: 4_u8,
            b: 5,
            c: 6_u8,
        },
    ])));
    {
        ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any().memcpy(
            &((r.as_pointer() as Ptr<reordered>) as Ptr<reordered>).to_any(),
            24usize as usize,
        );
        ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any()
    };
    assert!(
        (((((*buf.borrow())[((12usize as usize).wrapping_add(8_usize)) as usize] as i32) == 6)
            as i32)
            != 0)
    );
    let r2: Value<Box<[reordered]>> = Rc::new(RefCell::new(
        (0..2)
            .map(|_| <reordered>::default())
            .collect::<Box<[reordered]>>(),
    ));
    {
        ((r2.as_pointer() as Ptr<reordered>) as Ptr<reordered>)
            .to_any()
            .memcpy(
                &((buf.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any(),
                24usize as usize,
            );
        ((r2.as_pointer() as Ptr<reordered>) as Ptr<reordered>).to_any()
    };
    assert!(
        (((((((((({ (*r2.borrow())[(1) as usize].a } as i32) == 4) as i32) != 0)
            && ((({ (*r2.borrow())[(1) as usize].b } == 5) as i32) != 0)) as i32)
            != 0)
            && (((({ (*r2.borrow())[(1) as usize].c } as i32) == 6) as i32) != 0))
            as i32)
            != 0)
    );
    let n: Value<Box<[nested]>> = Rc::new(RefCell::new(Box::new([
        nested {
            t: tail { a: 1_u8, b: 2.5E+0 },
            c: 3_u8,
        },
        nested {
            t: tail { a: 4_u8, b: 5.5E+0 },
            c: 6_u8,
        },
    ])));
    {
        ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any().memcpy(
            &((n.as_pointer() as Ptr<nested>) as Ptr<nested>).to_any(),
            48usize as usize,
        );
        ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any()
    };
    assert!(
        (((((*buf.borrow())[((24usize as usize).wrapping_add(16_usize)) as usize] as i32) == 6)
            as i32)
            != 0)
    );
    let n2: Value<Box<[nested]>> = Rc::new(RefCell::new(
        (0..2)
            .map(|_| <nested>::default())
            .collect::<Box<[nested]>>(),
    ));
    {
        ((n2.as_pointer() as Ptr<nested>) as Ptr<nested>)
            .to_any()
            .memcpy(
                &((buf.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any(),
                48usize as usize,
            );
        ((n2.as_pointer() as Ptr<nested>) as Ptr<nested>).to_any()
    };
    assert!(
        (((((((((({ (*n2.borrow())[(1) as usize].t.a } as i32) == 4) as i32) != 0)
            && ((({ (*n2.borrow())[(1) as usize].t.b } == 5.5E+0) as i32) != 0))
            as i32)
            != 0)
            && (((({ (*n2.borrow())[(1) as usize].c } as i32) == 6) as i32) != 0))
            as i32)
            != 0)
    );
    let a: Value<Box<[array]>> = Rc::new(RefCell::new(Box::new([
        array {
            name: Rc::new(RefCell::new(Box::from(*b"ab\0"))),
            x: 1,
        },
        array {
            name: Rc::new(RefCell::new(Box::from(*b"cd\0"))),
            x: 2,
        },
    ])));
    {
        ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any().memcpy(
            &((a.as_pointer() as Ptr<array>) as Ptr<array>).to_any(),
            16usize as usize,
        );
        ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any()
    };
    assert!(
        (((((*buf.borrow())[((8usize as usize).wrapping_add(1_usize)) as usize] as i32)
            == ('d' as i32)) as i32)
            != 0)
    );
    let a2: Value<Box<[array]>> = Rc::new(RefCell::new(
        (0..2).map(|_| <array>::default()).collect::<Box<[array]>>(),
    ));
    {
        ((a2.as_pointer() as Ptr<array>) as Ptr<array>)
            .to_any()
            .memcpy(
                &((buf.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any(),
                16usize as usize,
            );
        ((a2.as_pointer() as Ptr<array>) as Ptr<array>).to_any()
    };
    assert!(
        ((((((((((((array_field_ptr!((a2.as_pointer() as Ptr<array>).offset(1), name) as Ptr::<u8>)
            .offset((1) as isize)
            .read()) as i32)
            == ('d' as i32)) as i32)
            != 0)
            && ((((((array_field_ptr!((a2.as_pointer() as Ptr<array>).offset(1), name)
                as Ptr::<u8>)
                .offset((2) as isize)
                .read()) as i32)
                == 0) as i32)
                != 0)) as i32)
            != 0)
            && ((({ (*a2.borrow())[(1) as usize].x } == 2) as i32) != 0)) as i32)
            != 0)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
