extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn set_0(mut p: Ptr<i32>, mut v: i32) {
    p.write({ v });
}
pub fn inc_1(r: Ptr<i32>) {
    r.with_mut(|__v| __v.prefix_inc());
}
pub fn sum_2(mut arr: Ptr<i32>, mut n: i32) -> i32 {
    let mut s: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while (i < n) {
        s += { (elem!(arr, i).read()) };
        i.prefix_inc();
    }
    return s;
}
pub fn countdown_3(mut n: i32, step: Option<i32>) -> i32 {
    let mut step: i32 = step.unwrap_or(1);
    let mut steps: i32 = 0;
    'loop_: while (n > 0) {
        n -= step;
        steps.postfix_inc();
    }
    return steps;
}
pub trait Shape {
    fn scale(&mut self, factor: i32) -> i32;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct Square {
    #[offset(8)]
    pub side: i32,
}
impl Default for Square {
    fn default() -> Self {
        Square { side: 2 }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut plain: i32 = 1;
    plain += 2;
    plain.postfix_inc();
    assert!((plain == 4));
    &(::std::mem::size_of::<i32>());
    let addr: Value<i32> = Rc::new(RefCell::new(0));
    ({ set_0((addr.as_pointer()), 5) });
    assert!(((*addr.borrow()) == 5));
    let ref_: Value<i32> = Rc::new(RefCell::new(1));
    ({ inc_1(ref_.as_pointer()) });
    let alias: Ptr<i32> = ref_.as_pointer();
    alias.with_mut(|__v| __v.postfix_inc());
    assert!(((*ref_.borrow()) == 3));
    let captured: Value<i32> = Rc::new(RefCell::new(7));
    let get: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let captured: Ptr<i32> = captured.as_pointer();
        },
        || -> i32 {
            return (captured.read());
        }
    )));
    (*captured.borrow_mut()) = 8;
    assert!((({ (*get.borrow()).call() }) == 8));
    let mut arr: [i32; 4] = [1, 2, 3, 4];
    arr[(0) as usize] = { (arr[(3) as usize] * 2) };
    arr[(1) as usize].postfix_inc();
    assert!(((arr[(0) as usize] + arr[(1) as usize]) == 11));
    let elem: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0])));
    ({ set_0(((elem.as_pointer() as Ptr<i32>).offset(1)), 6) });
    assert!(((*elem.borrow())[(1) as usize] == 6));
    let decayed: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3])));
    assert!((({ sum_2((decayed.as_pointer() as Ptr::<i32>), 3,) }) == 6));
    let elem_ref: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1])));
    ({ inc_1((elem_ref.as_pointer() as Ptr<i32>).offset(0)) });
    assert!(((*elem_ref.borrow())[(0) as usize] == 2));
    let mut p: Ptr<i32> = (addr.as_pointer());
    p = ((elem.as_pointer() as Ptr<i32>).offset(0));
    p.write(9);
    assert!(((*elem.borrow())[(0) as usize] == 9));
    let mut str: [i8; 8] = b"abc\0\0\0\0\0".map(i8::from_byte);
    str[(0) as usize] = ('x' as i8);
    assert!(
        ((str[(0) as usize] as i32) == (('x' as i8) as i32))
            && ((str[(3) as usize] as i32) == (('\0' as i8) as i32))
    );
    let mut zeros: [f64; 16] = [
        0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64,
        0_f64, 0_f64, 0_f64,
    ];
    zeros[(15) as usize] = 1.5E+0;
    assert!((zeros[(0) as usize] == 0_f64) && (zeros[(15) as usize] == 1.5E+0));
    assert!((({ countdown_3(10, None,) }) == 10));
    assert!((({ countdown_3(10, Some(3),) }) == 4));
    let square: Value<Square> = Rc::new(RefCell::new(<Square>::default()));
    let mut shape: PtrDyn<dyn Shape> = (square.as_pointer()).to_dyn::<dyn Shape>(|w| w);
    assert!((({ (*shape.upgrade().deref_mut()).scale(3,) }) == 6));
    return 0;
}
impl Shape for Square {
    fn scale(&mut self, mut factor: i32) -> i32 {
        return (factor * { self.side });
    }
}
pub fn __cpp2rust_init_globals() {}
