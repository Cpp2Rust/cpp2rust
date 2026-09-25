extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn fill_row_0(row: Ptr<u8>, c: u8) {
    let row: Value<Ptr<u8>> = Rc::new(RefCell::new(row));
    let c: Value<u8> = Rc::new(RefCell::new(c));
    let __rhs = (*c.borrow());
    (*row.borrow()).offset((0) as isize).write(__rhs);
    (*row.borrow())
        .offset((1) as isize)
        .write((('\0' as i32) as u8));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let grid: Value<Box<[Value<Box<[u8]>>]>> = Rc::new(RefCell::new(
        (0..3)
            .map(|_| Rc::new(RefCell::new((0..6).map(|_| 0_u8).collect::<Box<[u8]>>())))
            .collect::<Box<[Value<Box<[u8]>>]>>(),
    ));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((((*i.borrow()) < 3) as i32) != 0) {
        ({
            let _row: Ptr<u8> = (((grid.as_pointer() as Ptr<Value<Box<[u8]>>>)
                .offset((*i.borrow()))
                .read()
                .as_pointer()) as Ptr<u8>);
            let _c: u8 = ((('a' as i32) + (*i.borrow())) as u8);
            fill_row_0(_row, _c)
        });
        (*i.borrow_mut()).postfix_inc();
    }
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*grid.borrow())[(0) as usize].borrow()[(0) as usize] as i32) == ('a' as i32))
                as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_1(
                        Ptr::<u8>::from_string_literal(b"grid[0][0] == \'a\'"),
                        Ptr::<u8>::from_string_literal(b"array_2d_default.c"),
                        13_u32,
                        Ptr::<u8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*grid.borrow())[(1) as usize].borrow()[(0) as usize] as i32) == ('b' as i32))
                as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_1(
                        Ptr::<u8>::from_string_literal(b"grid[1][0] == \'b\'"),
                        Ptr::<u8>::from_string_literal(b"array_2d_default.c"),
                        14_u32,
                        Ptr::<u8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*grid.borrow())[(2) as usize].borrow()[(0) as usize] as i32) == ('c' as i32))
                as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_1(
                        Ptr::<u8>::from_string_literal(b"grid[2][0] == \'c\'"),
                        Ptr::<u8>::from_string_literal(b"array_2d_default.c"),
                        15_u32,
                        Ptr::<u8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*grid.borrow())[(1) as usize].borrow()[(1) as usize] as i32) == ('\0' as i32))
                as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_1(
                        Ptr::<u8>::from_string_literal(b"grid[1][1] == \'\\0\'"),
                        Ptr::<u8>::from_string_literal(b"array_2d_default.c"),
                        16_u32,
                        Ptr::<u8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    (*grid.borrow())[(2) as usize].borrow_mut()[(5) as usize] = (('z' as i32) as u8);
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*grid.borrow())[(2) as usize].borrow()[(5) as usize] as i32) == ('z' as i32))
                as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_1(
                        Ptr::<u8>::from_string_literal(b"grid[2][5] == \'z\'"),
                        Ptr::<u8>::from_string_literal(b"array_2d_default.c"),
                        18_u32,
                        Ptr::<u8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    return 0;
}
pub fn __assert_fail_1(__assertion: Ptr<u8>, __file: Ptr<u8>, __line: u32, __function: Ptr<u8>) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
