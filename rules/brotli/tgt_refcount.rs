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
    // Compress into a buffer bounded by the input size, as the output buffer
    // may be much larger and is expensive to borrow if reinterpreted.
    let mut __out_len = a5.read();
    let __max = unsafe { ::brotli_sys::BrotliEncoderMaxCompressedSize(a3) };
    let mut __out = vec![0u8; if __max == 0 { __out_len } else { __out_len.min(__max) }];
    __out_len = __out.len();
    let __ok = a4.with_slice(a3, |__in| unsafe {
        ::brotli_sys::BrotliEncoderCompress(
            a0,
            a1,
            a2,
            a3,
            __in.as_ptr(),
            &mut __out_len,
            __out.as_mut_ptr(),
        )
    });
    if __ok != 0 {
        a6.with_slice_mut(__out_len, |__s| __s.copy_from_slice(&__out[..__out_len]));
        a5.write(__out_len);
    }
    __ok
}

fn f5(a0: usize, a1: Ptr<u8>, a2: Ptr<usize>, a3: Ptr<u8>) -> ::brotli_sys::BrotliDecoderResult {
    let __out_len = a2.read();
    a1.with_slice(a0, |__in| {
        a2.with_mut(|_v2| {
            a3.with_slice_mut(__out_len, |__out| unsafe {
                ::brotli_sys::BrotliDecoderDecompress(
                    a0,
                    __in.as_ptr(),
                    _v2 as *mut usize,
                    __out.as_mut_ptr(),
                )
            })
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
    let __in = a2.read();
    let __in_len = a1.read();
    let __r = __in.with_slice(__in_len, |__s| {
        a1.with_mut(|_v1| {
            a3.with_mut(|_v3| unsafe {
                ::brotli_sys::BrotliDecoderDecompressStream(
                    a0,
                    _v1,
                    &mut __s.as_ptr(),
                    _v3,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                )
            })
        })
    });
    a2.write(__in.offset(__in_len - a1.read()));
    __r
}

fn f9(a0: *mut ::brotli_sys::BrotliDecoderState, a1: Ptr<usize>) -> Ptr<u8> {
    unsafe {
        a1.with_mut(|_v1| {
            let output: *const u8 = ::brotli_sys::BrotliDecoderTakeOutput(a0, _v1 as *mut usize);
            let slice = std::slice::from_raw_parts(output, *_v1);
            let result: Ptr<Vec<u8>> = Ptr::alloc(slice.to_vec());
            result.decay()
        })
    }
}
