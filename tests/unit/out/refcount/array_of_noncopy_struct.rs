extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(32)]
pub struct NonCopy {
    #[offset(0)]
    #[byte_size(24)]
    pub data: Value<Vec<i32>>,
    #[offset(24)]
    pub tag: i32,
}
impl Clone for NonCopy {
    fn clone(&self) -> Self {
        Self {
            data: Rc::new(RefCell::new((*self.data.borrow()).clone())),
            tag: self.tag.clone(),
        }
    }
}
impl Default for NonCopy {
    fn default() -> Self {
        NonCopy {
            data: Rc::new(RefCell::new(Default::default())),
            tag: 0,
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let arr: Value<Box<[NonCopy]>> = Rc::new(RefCell::new(
        (0..3)
            .map(|_| <NonCopy>::default())
            .collect::<Box<[NonCopy]>>(),
    ));
    (*arr.borrow_mut())[(0) as usize].tag = 7;
    {
        let __a1 = 42;
        (*{ (*arr.borrow())[(1) as usize].data.clone() }.borrow_mut()).push(__a1)
    };
    assert!(({ (*arr.borrow())[(0) as usize].tag } == 7));
    assert!(((*{ (*arr.borrow())[(1) as usize].data.clone() }.borrow()).len() == 1_usize));
    assert!(
        ((elem!(
            ({ (*arr.borrow())[(1) as usize].data.as_pointer() } as Ptr<i32>),
            0_usize
        )
        .read())
            == 42)
    );
    assert!(({ (*arr.borrow())[(2) as usize].tag } == 0));
    assert!(((*{ (*arr.borrow())[(2) as usize].data.clone() }.borrow()).len() == 0_usize));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
