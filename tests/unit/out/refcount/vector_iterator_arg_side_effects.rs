extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let v: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([1, 2, 3, 4])));
    let it: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(v.decay().begin()));
    {
        (*it.borrow_mut()).iter_inc();
    };
    let r: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
        match ({ <RandomAccessIterator<i32>>::from((*it.borrow_mut()).iter_post_inc()) },) {
            (mut __tmp_0,) => {
                let __off = __tmp_0.index();
                v.as_pointer()
                    .with_mut(|__v: &mut Vec<i32>| __v.remove(__off));
                (__tmp_0).clone()
            }
        },
    ));
    assert!(((({ (*r.borrow()).iter_deref() }).read()) == 3));
    assert!(({ ((*v.borrow()).len() == 3_usize) }));
    let it2: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(v.decay().begin()));
    {
        (*r.borrow_mut()) =
            match ({ <RandomAccessIterator<i32>>::from((*it2.borrow_mut()).iter_post_inc()) },) {
                (mut __tmp_1,) => {
                    let __off = __tmp_1.index();
                    (*v.borrow_mut()).insert(__off, 9);
                    (__tmp_1).clone()
                }
            };
    };
    assert!(
        ({
            (((({ (*r.borrow()).iter_deref() }).read()) == 9) && ({ (*v.borrow())[0_usize] } == 9))
                && ((*v.borrow()).len() == 4_usize)
        })
    );
    let c: Ptr<Vec<i32>> = v.as_pointer();
    let mut i: i32 = 1;
    assert!(
        ({
            ((match ({ (i.postfix_inc() as usize) },) {
                (mut __tmp_2,) => {
                    if __tmp_2 >= (*c.upgrade().deref()).len() {
                        panic!("out of bounds access")
                    }
                    c.decay().offset((__tmp_2) as isize)
                }
            }
            .read())
                == 1)
                && (i == 2)
        })
    );
    let w: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([10, 20, 30, 40])));
    let a: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(w.decay().end()));
    let ra: Ptr<RandomAccessIterator<i32>> = ({
        (*a.borrow_mut()).iter_add_assign(-3_isize);
        a.as_pointer()
    });
    assert!(
        ({ (ra).clone() } == { (a.as_pointer()) })
            && ((({ (*a.borrow()).iter_deref() }).read()) == 20)
    );
    let rb: Ptr<RandomAccessIterator<i32>> = {
        (*a.borrow_mut()).iter_inc();
        a.as_pointer()
    };
    assert!(
        ({ (rb).clone() } == { (a.as_pointer()) })
            && ((({ (*a.borrow()).iter_deref() }).read()) == 30)
    );
    let rc: Ptr<RandomAccessIterator<i32>> = {
        (*a.borrow_mut()).iter_dec();
        a.as_pointer()
    };
    assert!(
        ({ (rc).clone() } == { (a.as_pointer()) })
            && ((({ (*a.borrow()).iter_deref() }).read()) == 20)
    );
    let rd: Ptr<RandomAccessIterator<i32>> = ({
        (*a.borrow_mut()).iter_add_assign(2_isize);
        a.as_pointer()
    });
    assert!(
        ({ (rd).clone() } == { (a.as_pointer()) })
            && ((({ (*a.borrow()).iter_deref() }).read()) == 40)
    );
    let b: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(w.decay().begin()));
    let mut x: i32 = (({
        (*{
            (*b.borrow_mut()).iter_inc();
            b.as_pointer()
        }
        .upgrade()
        .deref())
        .iter_deref()
    })
    .read());
    assert!((x == 20));
    let k: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
        (*{
            (*b.borrow_mut()).iter_inc();
            b.as_pointer()
        }
        .upgrade()
        .deref())
        .clone(),
    ));
    {
        (*k.borrow_mut()).iter_inc();
    };
    assert!(
        ((({ (*k.borrow()).iter_deref() }).read()) == 40)
            && ((({ (*b.borrow()).iter_deref() }).read()) == 30)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
