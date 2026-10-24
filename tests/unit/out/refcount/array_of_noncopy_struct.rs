extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(32)]
pub struct NonCopy {
    #[offset(0)]
    #[byte_size(24)]
    pub data: Vec<i32>,
    #[offset(24)]
    pub tag: i32,
}
impl NonCopy {
    pub fn move_from(_a0: Ptr<NonCopy>) -> Self {
        Self {
            data: field!(_a0, data).with_mut(std::mem::take),
            tag: { (*_a0.upgrade().deref()).tag },
        }
    }
}
impl Default for NonCopy {
    fn default() -> Self {
        {
            NonCopy {
                data: Default::default(),
                tag: 0,
            }
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut arr: [NonCopy; 3] = std::array::from_fn::<_, 3, _>(|_| <NonCopy>::default());
    arr[(0) as usize].tag = 7;
    {
        let __a1 = 42;
        arr[(1) as usize].data.push(__a1)
    };
    assert!((arr[(0) as usize].tag == 7));
    assert!((arr[(1) as usize].data.len() == 1_usize));
    assert!((arr[(1) as usize].data[0_usize] == 42));
    assert!((arr[(2) as usize].tag == 0));
    assert!((arr[(2) as usize].data.len() == 0_usize));
    return 0;
}
pub trait NonCopyImpl {
    fn move_assign(&self, _a0: Ptr<NonCopy>) -> Ptr<NonCopy>;
}
impl NonCopyImpl for Ptr<NonCopy> {
    fn move_assign(&self, _a0: Ptr<NonCopy>) -> Ptr<NonCopy> {
        field_ref!((*self), data)
            .as_pointer()
            .write(field!(_a0, data).with_mut(std::mem::take));
        field!((*self), tag).write({ { (*_a0.upgrade().deref()).tag } });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
