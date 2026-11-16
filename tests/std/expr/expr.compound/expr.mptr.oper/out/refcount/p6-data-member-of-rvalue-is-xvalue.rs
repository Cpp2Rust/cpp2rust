extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static copyCount_0: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static moveCount_1: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(1)]
pub struct CountsCopiesAndMoves {}
impl CountsCopiesAndMoves {
    pub fn copy_from(_a0: Ptr<CountsCopiesAndMoves>) -> Self {
        let __this: CountsCopiesAndMoves = Self {};
        (*copyCount_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn move_from(_a0: Ptr<CountsCopiesAndMoves>) -> Self {
        let __this: CountsCopiesAndMoves = Self {};
        (*moveCount_1.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Clone for CountsCopiesAndMoves {
    fn clone(&self) -> Self {
        let __src: Value<CountsCopiesAndMoves> = Rc::new(RefCell::new(CountsCopiesAndMoves {}));
        CountsCopiesAndMoves::copy_from(__src.as_pointer())
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(1)]
pub struct HoldsCounted {
    #[offset(0)]
    #[byte_size(1)]
    pub member: CountsCopiesAndMoves,
}
impl HoldsCounted {
    pub fn move_from(_a0: Ptr<HoldsCounted>) -> Self {
        Self {
            member: CountsCopiesAndMoves::move_from({ field_ptr!(_a0, member) }),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut pm: isize = (offset_of_field!(HoldsCounted, member) as isize);
    let holder: Value<HoldsCounted> = Rc::new(RefCell::new(<HoldsCounted>::default()));
    let mut from_lvalue: CountsCopiesAndMoves = CountsCopiesAndMoves::copy_from({
        (holder.as_pointer()).member::<CountsCopiesAndMoves>(pm)
    });
    assert!((copyCount_0.with(|rc| *rc.borrow()) == 1));
    assert!((moveCount_1.with(|rc| *rc.borrow()) == 0));
    let mut from_xvalue: CountsCopiesAndMoves = CountsCopiesAndMoves::move_from({
        (holder.as_pointer()).member::<CountsCopiesAndMoves>(pm)
    });
    assert!((copyCount_0.with(|rc| *rc.borrow()) == 1));
    assert!((moveCount_1.with(|rc| *rc.borrow()) == 1));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = copyCount_0.with(|_| ());
    let _ = moveCount_1.with(|_| ());
}
