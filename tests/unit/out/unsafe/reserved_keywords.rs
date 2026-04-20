extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(Copy, Clone, Default)]
pub struct S {
    pub as_: i32,
    pub async_: i32,
    pub await_: i32,
    pub crate_: i32,
    pub dyn_: i32,
    pub fn_: i32,
    pub impl_: i32,
    pub in_: i32,
    pub let_: i32,
    pub loop_: i32,
    pub match_: i32,
    pub mod_: i32,
    pub move_: i32,
    pub mut_: i32,
    pub pub_: i32,
    pub ref_: i32,
    pub self_: i32,
    pub Self_: i32,
    pub super_: i32,
    pub trait_: i32,
    pub type_: i32,
    pub unsafe_: i32,
    pub use_: i32,
    pub where_: i32,
    pub abstract_: i32,
    pub become_: i32,
    pub box_: i32,
    pub final_: i32,
    pub gen_: i32,
    pub macro_: i32,
    pub override_: i32,
    pub priv_: i32,
    pub unsized_: i32,
    pub yield_: i32,
    pub macro_rules_: i32,
    pub raw_: i32,
    pub safe_: i32,
    pub vec_: i32,
}
pub unsafe fn foo_0(
    mut as_: i32,
    mut async_: i32,
    mut await_: i32,
    mut crate_: i32,
    mut dyn_: i32,
    mut fn_: i32,
    mut impl_: i32,
    mut in_: i32,
    mut let_: i32,
    mut loop_: i32,
    mut match_: i32,
    mut mod_: i32,
    mut move_: i32,
    mut mut_: i32,
    mut pub_: i32,
    mut ref_: i32,
    mut self_: i32,
    mut Self_: i32,
    mut super_: i32,
    mut trait_: i32,
    mut type_: i32,
    mut unsafe_: i32,
    mut use_: i32,
    mut where_: i32,
    mut abstract_: i32,
    mut become_: i32,
    mut box_: i32,
    mut final_: i32,
    mut gen_: i32,
    mut macro_: i32,
    mut override_: i32,
    mut priv_: i32,
    mut unsized_: i32,
    mut yield_: i32,
    mut macro_rules_: i32,
    mut raw_: i32,
    mut safe_: i32,
    mut vec_: i32,
    mut dummy: i32,
) -> i32 {
    return 0;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: S = S {
        as_: 0,
        async_: 0,
        await_: 0,
        crate_: 0,
        dyn_: 0,
        fn_: 0,
        impl_: 0,
        in_: 0,
        let_: 0,
        loop_: 0,
        match_: 0,
        mod_: 0,
        move_: 0,
        mut_: 0,
        pub_: 0,
        ref_: 0,
        self_: 0,
        Self_: 0,
        super_: 0,
        trait_: 0,
        type_: 0,
        unsafe_: 0,
        use_: 0,
        where_: 0,
        abstract_: 0,
        become_: 0,
        box_: 0,
        final_: 0,
        gen_: 0,
        macro_: 0,
        override_: 0,
        priv_: 0,
        unsized_: 0,
        yield_: 0,
        macro_rules_: 0,
        raw_: 0,
        safe_: 0,
        vec_: 0,
    };
    let mut as_: i32 = 0;
    let mut async_: i32 = 0;
    let mut await_: i32 = 0;
    let mut crate_: i32 = 0;
    let mut dyn_: i32 = 0;
    let mut fn_: i32 = 0;
    let mut impl_: i32 = 0;
    let mut in_: i32 = 0;
    let mut let_: i32 = 0;
    let mut loop_: i32 = 0;
    let mut match_: i32 = 0;
    let mut mod_: i32 = 0;
    let mut move_: i32 = 0;
    let mut mut_: i32 = 0;
    let mut pub_: i32 = 0;
    let mut ref_: i32 = 0;
    let mut self_: i32 = 0;
    let mut Self_: i32 = 0;
    let mut super_: i32 = 0;
    let mut trait_: i32 = 0;
    let mut type_: i32 = 0;
    let mut unsafe_: i32 = 0;
    let mut use_: i32 = 0;
    let mut where_: i32 = 0;
    let mut abstract_: i32 = 0;
    let mut become_: i32 = 0;
    let mut box_: i32 = 0;
    let mut final_: i32 = 0;
    let mut gen_: i32 = 0;
    let mut macro_: i32 = 0;
    let mut override_: i32 = 0;
    let mut priv_: i32 = 0;
    let mut unsized_: i32 = 0;
    let mut yield_: i32 = 0;
    let mut macro_rules_: i32 = 0;
    let mut raw_: i32 = 0;
    let mut safe_: i32 = 0;
    let mut vec_: i32 = 0;
    return (unsafe {
        let _as: i32 = 0;
        let _async: i32 = 0;
        let _await: i32 = 0;
        let _crate: i32 = 0;
        let _dyn: i32 = 0;
        let _fn: i32 = 0;
        let _impl: i32 = 0;
        let _in: i32 = 0;
        let _let: i32 = 0;
        let _loop: i32 = 0;
        let _match: i32 = 0;
        let _mod: i32 = 0;
        let _move: i32 = 0;
        let _mut: i32 = 0;
        let _pub: i32 = 0;
        let _ref: i32 = 0;
        let _self: i32 = 0;
        let _Self: i32 = 0;
        let _super: i32 = 0;
        let _trait: i32 = 0;
        let _type: i32 = 0;
        let _unsafe: i32 = 0;
        let _use: i32 = 0;
        let _where: i32 = 0;
        let _abstract: i32 = 0;
        let _become: i32 = 0;
        let _box: i32 = 0;
        let _final: i32 = 0;
        let _gen: i32 = 0;
        let _macro: i32 = 0;
        let _override: i32 = 0;
        let _priv: i32 = 0;
        let _unsized: i32 = 0;
        let _yield: i32 = 0;
        let _macro_rules: i32 = 0;
        let _raw: i32 = 0;
        let _safe: i32 = 0;
        let _vec: i32 = 0;
        let _dummy: i32 = 0;
        foo_0(
            _as,
            _async,
            _await,
            _crate,
            _dyn,
            _fn,
            _impl,
            _in,
            _let,
            _loop,
            _match,
            _mod,
            _move,
            _mut,
            _pub,
            _ref,
            _self,
            _Self,
            _super,
            _trait,
            _type,
            _unsafe,
            _use,
            _where,
            _abstract,
            _become,
            _box,
            _final,
            _gen,
            _macro,
            _override,
            _priv,
            _unsized,
            _yield,
            _macro_rules,
            _raw,
            _safe,
            _vec,
            _dummy,
        )
    });
}
