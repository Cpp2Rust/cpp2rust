extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn len_0(s: Ptr<u8>) -> i32 {
    let n: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((((s).offset((*n.borrow()) as isize).read()) as i32) != (('\0' as u8) as i32)) {
        (*n.borrow_mut()).prefix_inc();
    }
    return (*n.borrow());
}
pub fn len5_1(s: Ptr<u8>) -> i32 {
    let s: Value<Ptr<u8>> = Rc::new(RefCell::new(s));
    let n: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((((*s.borrow()).offset((*n.borrow()) as isize).read()) as i32)
        != (('\0' as u8) as i32))
    {
        (*n.borrow_mut()).prefix_inc();
    }
    return (*n.borrow());
}
pub fn sum_2(a: Ptr<i32>) -> i32 {
    return ((((a).offset((0) as isize).read()) + ((a).offset((1) as isize).read()))
        + ((a).offset((2) as isize).read()));
}
pub fn fill_3(a: Ptr<i32>, v: i32) {
    let v: Value<i32> = Rc::new(RefCell::new(v));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < 3) {
        (a).offset((*i.borrow()) as isize).write((*v.borrow()));
        (*i.borrow_mut()).prefix_inc();
    }
}
pub fn sum_twice_4(a: Ptr<i32>) -> i32 {
    return (({ sum_2(((a).clone() as Ptr<i32>)) }) + ({ sum_2(((a).clone() as Ptr<i32>)) }));
}
pub fn sum_ptr_5(p: Ptr<i32>) -> i32 {
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new(p));
    return ((((*p.borrow()).offset((0) as isize).read())
        + ((*p.borrow()).offset((1) as isize).read()))
        + ((*p.borrow()).offset((2) as isize).read()));
}
pub fn sum_decayed_6(a: Ptr<i32>) -> i32 {
    return ({ sum_ptr_5((a).clone()) });
}
pub fn bump_ptr_7(p: Ptr<i32>) {
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new(p));
    {
        let _ptr = (*p.borrow()).offset((0) as isize);
        _ptr.write(_ptr.read() + 1)
    };
}
pub fn bump_decayed_8(a: Ptr<i32>) {
    ({ bump_ptr_7((a).clone()) });
}
pub fn fill_and_sum_9(a: Ptr<i32>, v: i32, out: Ptr<i32>) {
    let v: Value<i32> = Rc::new(RefCell::new(v));
    ({
        let _a: Ptr<i32> = ((a).clone() as Ptr<i32>);
        let _v: i32 = (*v.borrow());
        fill_3(_a, _v)
    });
    let __rhs = ({ sum_twice_4(((a).clone() as Ptr<i32>)) });
    out.write(__rhs);
}
pub fn pick_10(s: Ptr<u8>) -> Ptr<u8> {
    return ((s).clone() as Ptr<u8>);
}
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct Point {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
impl ByteRepr for Point {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.x.to_bytes(&mut buf[0..4]);
        self.y.to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: <i32>::from_bytes(&buf[0..4]),
            y: <i32>::from_bytes(&buf[4..8]),
        }
    }
}
pub fn sum_points_11(p: Ptr<Point>) -> i32 {
    return {
        let _lhs = {
            let _lhs = {
                let _lhs = { (*(p).offset((0) as isize).upgrade().deref()).x };
                _lhs + { (*(p).offset((0) as isize).upgrade().deref()).y }
            };
            _lhs + { (*(p).offset((1) as isize).upgrade().deref()).x }
        };
        _lhs + { (*(p).offset((1) as isize).upgrade().deref()).y }
    };
}
pub fn shift_points_12(p: Ptr<Point>, d: i32) {
    let d: Value<i32> = Rc::new(RefCell::new(d));
    {
        let _ptr = field!((p).offset((0) as isize), x);
        _ptr.write(_ptr.read() + (*d.borrow()))
    };
    {
        let _ptr = field!((p).offset((1) as isize), y);
        _ptr.write(_ptr.read() + (*d.borrow()))
    };
}
pub fn total_len_13(names: Ptr<Ptr<u8>>) -> i32 {
    return (({ len5_1(((names).offset((0) as isize).read()).clone()) })
        + ({ len5_1(((names).offset((1) as isize).read()).clone()) }));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ len_0(Ptr::<u8>::from_string_literal(b"beta"),) }) == 4));
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::from(*b"abcd\0")));
    assert!((({ len_0((buf.as_pointer() as Ptr<u8>),) }) == 4));
    let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3])));
    assert!((({ sum_2((arr.as_pointer() as Ptr<i32>),) }) == 6));
    ({ fill_3((arr.as_pointer() as Ptr<i32>), 7) });
    assert!((({ sum_2((arr.as_pointer() as Ptr<i32>),) }) == 21));
    assert!((({ sum_twice_4((arr.as_pointer() as Ptr<i32>),) }) == 42));
    let out: Value<i32> = Rc::new(RefCell::new(0));
    ({ fill_and_sum_9((arr.as_pointer() as Ptr<i32>), 2, out.as_pointer()) });
    assert!(((*out.borrow()) == 12));
    assert!(((*arr.borrow())[(0) as usize] == 2));
    let lit: Ptr<u8> = Ptr::<u8>::from_string_literal(b"beta");
    assert!((({ len_0(((lit).clone() as Ptr<u8>),) }) == 4));
    assert!(
        (((({ pick_10(Ptr::<u8>::from_string_literal(b"beta"),) })
            .offset((0) as isize)
            .read()) as i32)
            == (('b' as u8) as i32))
    );
    assert!((({ len_0((({ pick_10((buf.as_pointer() as Ptr<u8>),) }) as Ptr<u8>),) }) == 4));
    let pts: Value<Box<[Point]>> = Rc::new(RefCell::new(Box::new([
        Point { x: 1, y: 2 },
        Point { x: 3, y: 4 },
    ])));
    assert!((({ sum_points_11((pts.as_pointer() as Ptr<Point>),) }) == 10));
    ({ shift_points_12((pts.as_pointer() as Ptr<Point>), 10) });
    assert!(({ (*pts.borrow())[(0) as usize].x } == 11));
    assert!(({ (*pts.borrow())[(1) as usize].y } == 14));
    assert!((({ sum_points_11((pts.as_pointer() as Ptr<Point>),) }) == 30));
    assert!(
        (({ sum_decayed_6((arr.as_pointer() as Ptr<i32>),) })
            == ({ sum_2((arr.as_pointer() as Ptr<i32>),) }))
    );
    ({ bump_decayed_8((arr.as_pointer() as Ptr<i32>)) });
    assert!(((*arr.borrow())[(0) as usize] == 3));
    let names: Value<Box<[Ptr<u8>]>> = Rc::new(RefCell::new(Box::new([
        Ptr::<u8>::from_string_literal(b"ab"),
        Ptr::<u8>::from_string_literal(b"cde"),
    ])));
    assert!((({ total_len_13((names.as_pointer() as Ptr<Ptr::<u8>>),) }) == 5));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
