// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::cell::RefCell;
use std::rc::Rc;

fn f2(
    a0: libc::c_int,
    a1: libc::c_int,
    a2: brotli_sys::BrotliEncoderMode,
    a3: usize,
    a4: Ptr<u8>,
    a5: Ptr<usize>,
    a6: Ptr<u8>,
) -> libc::c_int {
    a5.with_mut(|_v5| {
        a6.with_mut(|_v6| unsafe {
            ::brotli_sys::BrotliEncoderCompress(
                a0,
                a1,
                a2,
                a3,
                &*a4.upgrade().deref() as *const u8,
                _v5 as *mut usize,
                _v6,
            )
        })
    })
}

fn f5(a0: usize, a1: Ptr<u8>, a2: Ptr<usize>, a3: Ptr<u8>) -> ::brotli_sys::BrotliDecoderResult {
    a2.with_mut(|_v2| {
        a3.with_mut(|_v3| unsafe {
            ::brotli_sys::BrotliDecoderDecompress(
                a0,
                &*a1.upgrade().deref(),
                _v2 as *mut usize,
                _v3,
            )
        })
    })
}

fn f6(
    a0: ::brotli_sys::brotli_alloc_func,
    a1: ::brotli_sys::brotli_free_func,
    a2: *mut std::ffi::c_void,
) -> *mut ::brotli_sys::BrotliDecoderState {
    unsafe { ::brotli_sys::BrotliDecoderCreateInstance(None, None, std::ptr::null_mut()) }
}

fn f7(a0: *mut ::brotli_sys::BrotliDecoderState) {
    unsafe { ::brotli_sys::BrotliDecoderDestroyInstance(a0) }
}

fn f8(
    a0: *mut ::brotli_sys::BrotliDecoderState,
    a1: Ptr<usize>,
    a2: Ptr<Ptr<u8>>,
    a3: Ptr<usize>,
    a4: Ptr<Ptr<u8>>,
    a5: Ptr<usize>,
) -> ::brotli_sys::BrotliDecoderResult {
    unsafe {
        let _a2: Ptr<*const u8> =
            Ptr::alloc((&*(*a2.upgrade().deref()).upgrade().deref()) as *const u8);

        a1.with_mut(|_v1| {
            _a2.with_mut(|_v2| {
                a3.with_mut(|_v3| {
                    ::brotli_sys::BrotliDecoderDecompressStream(
                        a0,
                        _v1 as *mut usize,
                        _v2 as *mut *const u8,
                        _v3 as *mut usize,
                        std::ptr::null_mut(),
                        std::ptr::null_mut(),
                    )
                })
            })
        })
    }
}

fn f9(a0: *mut ::brotli_sys::BrotliDecoderState, a1: Ptr<usize>) -> Ptr<u8> {
    unsafe {
        a1.with_mut(|_v1| {
            let output: *const u8 = ::brotli_sys::BrotliDecoderTakeOutput(a0, _v1 as *mut usize);
            let slice = std::slice::from_raw_parts(output, *_v1);
            let result: Ptr<Vec<u8>> = Ptr::alloc(slice.to_vec());
            (result.to_strong().as_pointer() as Ptr<u8>).clone()
        })
    }
}
