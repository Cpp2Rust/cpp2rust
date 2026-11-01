extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct Degrees {
    #[offset(0)]
    pub value: i32,
}
impl Degrees {
    pub fn new(mut v: i32) -> Self {
        Self { value: v }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let input: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3])));
    let output: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..3).map(|_| 0_i32).collect::<Box<[i32]>>()));
    match ({ (input.as_pointer() as Ptr<i32>).offset((3) as isize) },) {
        (mut __tmp_0,) => {
            let mut __first = (input.as_pointer() as Ptr<i32>);
            let mut __out = (output.as_pointer() as Ptr<i32>);
            while __first != __tmp_0 {
                __out.iter_deref().write((__first.iter_deref().read()));
                __first.iter_inc();
                __out.iter_inc();
            }
            __out
        }
    };
    let widened: Value<Vec<i64>> = Rc::new(RefCell::new(vec![0_i64; 3_usize as usize]));
    match ({ (input.as_pointer() as Ptr<i32>).offset((3) as isize) },) {
        (mut __tmp_1,) => {
            let mut __first = (input.as_pointer() as Ptr<i32>);
            let mut __out = widened.decay().begin();
            while __first != __tmp_1 {
                __out
                    .iter_deref()
                    .write(((__first.iter_deref().read()) as i64));
                __first.iter_inc();
                __out.iter_inc();
            }
            __out
        }
    };
    assert!(
        ({
            ({ (*widened.borrow())[0_usize] } == 1_i64)
                && ({ (*widened.borrow())[2_usize] } == 3_i64)
        })
    );
    let constructed: Value<Vec<Degrees>> =
        Rc::new(RefCell::new(vec![Degrees::new({ 0 },); 3_usize as usize]));
    match ({ (input.as_pointer() as Ptr<i32>).offset((3) as isize) },) {
        (mut __tmp_2,) => {
            let mut __first = (input.as_pointer() as Ptr<i32>);
            let mut __out = constructed.decay().begin();
            while __first != __tmp_2 {
                __out
                    .iter_deref()
                    .write(<Degrees as CppFrom<i32>>::cpp_from(__first.iter_deref()));
                __first.iter_inc();
                __out.iter_inc();
            }
            __out
        }
    };
    assert!(
        ({
            ({ (*constructed.borrow())[0_usize].value } == 1)
                && ({ (*constructed.borrow())[2_usize].value } == 3)
        })
    );
    let wide: Value<Box<[i64]>> = Rc::new(RefCell::new(Box::new([7_i64, 8_i64])));
    let floating: Value<Vec<f64>> = Rc::new(RefCell::new(vec![0_f64; 2_usize as usize]));
    match ({ (wide.as_pointer() as Ptr<i64>).offset((2) as isize) },) {
        (mut __tmp_3,) => {
            let mut __first = (wide.as_pointer() as Ptr<i64>);
            let mut __out = floating.decay().begin();
            while __first != __tmp_3 {
                __out
                    .iter_deref()
                    .write(((__first.iter_deref().read()) as f64));
                __first.iter_inc();
                __out.iter_inc();
            }
            __out
        }
    };
    assert!(({ ({ (*floating.borrow())[1_usize] } == 8_f64) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
