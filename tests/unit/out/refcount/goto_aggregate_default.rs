extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn agg_0(n: i32) -> i32 {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    let mut buf: Value<Box<[u8]>> = Rc::new(RefCell::new(
        (0..40).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
    ));
    let mut total: Value<i32> = <Value<i32>>::default();
    goto_block!({
        '__entry: {
            total = Rc::new(RefCell::new(0));
            if ((((*n.borrow()) < 0) as i32) != 0) {
                goto!('out);
            }
            {
                ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>)
                    .to_any()
                    .memset((1) as u8, ::std::mem::size_of::<[u8; 40]>() as u64 as usize);
                ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any().clone()
            };
            let i: Value<i32> = Rc::new(RefCell::new(0));
            'loop_: while ((((*i.borrow()) < 40) as i32) != 0) {
                (*total.borrow_mut()) += ((*buf.borrow())[(*i.borrow()) as usize] as i32);
                (*i.borrow_mut()).postfix_inc();
            }
        }
        'out: {
            return (*total.borrow());
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(
        (((({
            let _n: i32 = -1_i32;
            agg_0(_n)
        }) == 0) as i32)
            != 0)
    );
    assert!(
        (((({
            let _n: i32 = 2;
            agg_0(_n)
        }) == 40) as i32)
            != 0)
    );
    return 0;
}
