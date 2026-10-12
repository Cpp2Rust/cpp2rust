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
pub struct Point {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(48)]
pub struct Shape {
    #[offset(0)]
    pub id: i32,
    #[offset(4)]
    #[byte_size(16)]
    pub coords: Value<Box<[i32]>>,
    #[offset(20)]
    #[byte_size(24)]
    pub points: Value<Box<[Point]>>,
    #[offset(44)]
    pub tail: i32,
}
impl Default for Shape {
    fn default() -> Self {
        {
            Shape {
                id: 0_i32,
                coords: Rc::new(RefCell::new((0..4).map(|_| 0_i32).collect::<Box<[i32]>>())),
                points: Rc::new(RefCell::new(
                    (0..3).map(|_| <Point>::default()).collect::<Box<[Point]>>(),
                )),
                tail: 0_i32,
            }
        }
    }
}
pub fn sum_0(mut p: Ptr<i32>, mut n: i32) -> i32 {
    let mut s: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while (i < n) {
        s += { (elem!(p, i).read()) };
        i.prefix_inc();
    }
    return s;
}
pub fn set_y_1(mut p: Ptr<Point>, mut y: i32) {
    field!(p, y).write(y);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<Shape> = Rc::new(RefCell::new(Shape {
        id: 1,
        coords: Rc::new(RefCell::new(Box::new([10, 20, 30, 40]))),
        points: Rc::new(RefCell::new(Box::new([
            Point { x: 1, y: 2 },
            Point { x: 3, y: 4 },
            Point { x: 5, y: 6 },
        ]))),
        tail: 99,
    }));
    let mut c: Ptr<i32> =
        ((array_field_ptr!(s.as_pointer(), coords) as Ptr<i32>).offset((1) as isize));
    assert!(((c.read()) == 20));
    c.write(21);
    assert!(((elem!((array_field_ptr!(s.as_pointer(), coords) as Ptr::<i32>), 1).read()) == 21));
    c += 2;
    assert!(((c.read()) == 40));
    assert!(
        (((c).clone()
            - ((array_field_ptr!(s.as_pointer(), coords) as Ptr::<i32>).offset((0) as isize)))
            as i64
            == 3_i64)
    );
    assert!(((elem!(c, -1_i32).read()) == 30));
    assert!((({ sum_0((array_field_ptr!(s.as_pointer(), coords) as Ptr::<i32>), 4,) }) == 101));
    assert!(
        (({
            sum_0(
                ((array_field_ptr!(s.as_pointer(), coords) as Ptr<i32>).offset((2) as isize)),
                2,
            )
        }) == 70)
    );
    let mut p: Ptr<Point> =
        ((array_field_ptr!(s.as_pointer(), points) as Ptr<Point>).offset((1) as isize));
    assert!((p.with(|__s| __s.x) == 3));
    ({ set_y_1(p.offset((1) as isize), 60) });
    assert!(
        ({
            (*elem!((array_field_ptr!(s.as_pointer(), points) as Ptr<Point>), 2)
                .upgrade()
                .deref())
            .y
        } == 60)
    );
    let mut py: Ptr<i32> = (field_ptr!(
        (array_field_ptr!(s.as_pointer(), points) as Ptr<Point>).offset((0) as isize),
        y
    ));
    py.write(7);
    assert!(
        ({
            (*elem!((array_field_ptr!(s.as_pointer(), points) as Ptr<Point>), 0)
                .upgrade()
                .deref())
            .y
        } == 7)
    );
    let mut px: Ptr<i32> = (field_ptr!((p.offset((1) as isize)), x));
    {
        px.with_mut(|__v| *__v = *__v + 50)
    };
    assert!(
        ({
            (*elem!((array_field_ptr!(s.as_pointer(), points) as Ptr<Point>), 2)
                .upgrade()
                .deref())
            .x
        } == 55)
    );
    let mut sp: Ptr<Shape> = (s.as_pointer());
    let mut d: Ptr<i32> = (array_field_ptr!(sp, coords) as Ptr<i32>).offset((3) as isize);
    d.write(41);
    assert!(((elem!((array_field_ptr!(s.as_pointer(), coords) as Ptr::<i32>), 3).read()) == 41));
    field!(elem!((array_field_ptr!(sp, points) as Ptr<Point>), 1), y).write({
        ({ (elem!((array_field_ptr!(sp, coords) as Ptr::<i32>), 0).read()) } + {
            {
                (*elem!((array_field_ptr!(sp, points) as Ptr<Point>), 0)
                    .upgrade()
                    .deref())
                .x
            }
        })
    });
    assert!(
        ({
            (*elem!((array_field_ptr!(s.as_pointer(), points) as Ptr<Point>), 1)
                .upgrade()
                .deref())
            .y
        } == 11)
    );
    let mut q: Ptr<Point> = (array_field_ptr!(sp, points) as Ptr<Point>);
    field!(elem!(q, 2), x).write(8);
    assert!(
        ({
            (*elem!((array_field_ptr!(sp, points) as Ptr<Point>), 2)
                .upgrade()
                .deref())
            .x
        } == 8)
    );
    let mut t: Shape = (*s.borrow()).clone();
    elem!((t.coords.as_pointer() as Ptr::<i32>), 0).write(0);
    field!(elem!((t.points.as_pointer() as Ptr<Point>), 0), x).write(0);
    assert!(
        ((elem!((array_field_ptr!(s.as_pointer(), coords) as Ptr::<i32>), 0).read()) == 10)
            && ({
                (*elem!((array_field_ptr!(s.as_pointer(), points) as Ptr<Point>), 0)
                    .upgrade()
                    .deref())
                .x
            } == 1)
    );
    assert!(
        (((elem!((t.coords.as_pointer() as Ptr::<i32>), 1).read()) == 21)
            && ({
                (*elem!((t.points.as_pointer() as Ptr<Point>), 2)
                    .upgrade()
                    .deref())
                .y
            } == 60))
            && (t.tail == 99)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
