// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#![allow(non_snake_case)]

use crate::{ByteRepr, Ptr};
use brotli::enc::{BrotliCompress, BrotliEncoderParams, StandardAlloc};
use brotli_decompressor::{BrotliDecompress, BrotliDecompressStream, BrotliResult};

pub type BrotliDecoderResult = u32;
pub type BrotliDecoderErrorCode = i32;
pub type BrotliEncoderMode = u32;

pub const BROTLI_DECODER_RESULT_ERROR: BrotliDecoderResult = 0;
pub const BROTLI_DECODER_RESULT_SUCCESS: BrotliDecoderResult = 1;
pub const BROTLI_DECODER_RESULT_NEEDS_MORE_INPUT: BrotliDecoderResult = 2;
pub const BROTLI_DECODER_RESULT_NEEDS_MORE_OUTPUT: BrotliDecoderResult = 3;

pub const BROTLI_DECODER_NO_ERROR: BrotliDecoderErrorCode = 0;
pub const BROTLI_DECODER_ERROR_FORMAT_EXUBERANT_NIBBLE: BrotliDecoderErrorCode = -1;
pub const BROTLI_DECODER_ERROR_FORMAT_EXUBERANT_META_NIBBLE: BrotliDecoderErrorCode = -3;
pub const BROTLI_DECODER_ERROR_FORMAT_SIMPLE_HUFFMAN_ALPHABET: BrotliDecoderErrorCode = -4;
pub const BROTLI_DECODER_ERROR_FORMAT_SIMPLE_HUFFMAN_SAME: BrotliDecoderErrorCode = -5;
pub const BROTLI_DECODER_ERROR_FORMAT_CL_SPACE: BrotliDecoderErrorCode = -6;
pub const BROTLI_DECODER_ERROR_FORMAT_HUFFMAN_SPACE: BrotliDecoderErrorCode = -7;
pub const BROTLI_DECODER_ERROR_FORMAT_CONTEXT_MAP_REPEAT: BrotliDecoderErrorCode = -8;
pub const BROTLI_DECODER_ERROR_FORMAT_BLOCK_LENGTH_1: BrotliDecoderErrorCode = -9;
pub const BROTLI_DECODER_ERROR_FORMAT_BLOCK_LENGTH_2: BrotliDecoderErrorCode = -10;
pub const BROTLI_DECODER_ERROR_FORMAT_TRANSFORM: BrotliDecoderErrorCode = -11;
pub const BROTLI_DECODER_ERROR_FORMAT_DICTIONARY: BrotliDecoderErrorCode = -12;
pub const BROTLI_DECODER_ERROR_FORMAT_WINDOW_BITS: BrotliDecoderErrorCode = -13;
pub const BROTLI_DECODER_ERROR_FORMAT_PADDING_1: BrotliDecoderErrorCode = -14;
pub const BROTLI_DECODER_ERROR_FORMAT_PADDING_2: BrotliDecoderErrorCode = -15;
pub const BROTLI_DECODER_ERROR_INVALID_ARGUMENTS: BrotliDecoderErrorCode = -20;
pub const BROTLI_DECODER_ERROR_ALLOC_CONTEXT_MODES: BrotliDecoderErrorCode = -21;
pub const BROTLI_DECODER_ERROR_ALLOC_TREE_GROUPS: BrotliDecoderErrorCode = -22;
pub const BROTLI_DECODER_ERROR_ALLOC_CONTEXT_MAP: BrotliDecoderErrorCode = -25;
pub const BROTLI_DECODER_ERROR_ALLOC_RING_BUFFER_1: BrotliDecoderErrorCode = -26;
pub const BROTLI_DECODER_ERROR_ALLOC_RING_BUFFER_2: BrotliDecoderErrorCode = -27;
pub const BROTLI_DECODER_ERROR_ALLOC_BLOCK_TYPE_TREES: BrotliDecoderErrorCode = -30;

pub const BROTLI_MODE_GENERIC: BrotliEncoderMode = 0;
pub const BROTLI_MODE_TEXT: BrotliEncoderMode = 1;
pub const BROTLI_MODE_FONT: BrotliEncoderMode = 2;

pub struct BrotliDecoderState {
    pub state: brotli_decompressor::BrotliState<StandardAlloc, StandardAlloc, StandardAlloc>,
    taken: Ptr<u8>,
}

impl ByteRepr for BrotliDecoderState {}

impl Default for BrotliDecoderState {
    fn default() -> Self {
        Self {
            state: brotli_decompressor::BrotliState::new_strict(
                StandardAlloc::default(),
                StandardAlloc::default(),
                StandardAlloc::default(),
            ),
            taken: Ptr::null(),
        }
    }
}

impl BrotliDecoderState {
    fn release_taken(&mut self) {
        self.taken.delete();
        self.taken = Ptr::null();
    }
}

fn decoder_result(result: BrotliResult) -> BrotliDecoderResult {
    match result {
        BrotliResult::ResultSuccess => BROTLI_DECODER_RESULT_SUCCESS,
        BrotliResult::NeedsMoreInput => BROTLI_DECODER_RESULT_NEEDS_MORE_INPUT,
        BrotliResult::NeedsMoreOutput => BROTLI_DECODER_RESULT_NEEDS_MORE_OUTPUT,
        BrotliResult::ResultFailure => BROTLI_DECODER_RESULT_ERROR,
    }
}

// Returns the compressed input, or None if it does not fit in capacity bytes.
fn compress(
    quality: i32,
    lgwin: i32,
    mode: BrotliEncoderMode,
    input: &[u8],
    capacity: usize,
) -> Option<Vec<u8>> {
    use brotli::enc::backward_references::BrotliEncoderMode as Mode;
    let params = BrotliEncoderParams {
        quality,
        lgwin,
        mode: match mode {
            BROTLI_MODE_TEXT => Mode::BROTLI_MODE_TEXT,
            BROTLI_MODE_FONT => Mode::BROTLI_MODE_FONT,
            _ => Mode::BROTLI_MODE_GENERIC,
        },
        size_hint: input.len(),
        large_window: lgwin > 24,
        ..Default::default()
    };
    let mut out = Vec::new();
    match BrotliCompress(&mut &*input, &mut out, &params) {
        Ok(_) if out.len() <= capacity => Some(out),
        _ => None,
    }
}

// Returns the decompressed input, or None if it is invalid or does not fit in
// capacity bytes.
fn decompress(input: &[u8], capacity: usize) -> Option<Vec<u8>> {
    let mut out = Vec::new();
    match BrotliDecompress(&mut &*input, &mut out) {
        Ok(()) if out.len() <= capacity => Some(out),
        _ => None,
    }
}

pub fn BrotliEncoderCompress(
    quality: i32,
    lgwin: i32,
    mode: BrotliEncoderMode,
    input_size: usize,
    input_buffer: Ptr<u8>,
    encoded_size: Ptr<usize>,
    encoded_buffer: Ptr<u8>,
) -> i32 {
    let input = input_buffer.with_slice(input_size, |s| s.to_vec());
    match compress(quality, lgwin, mode, &input, encoded_size.read()) {
        Some(out) => {
            encoded_buffer.with_slice_mut(out.len(), |d| d.copy_from_slice(&out));
            encoded_size.write(out.len());
            1
        }
        None => {
            encoded_size.write(0);
            0
        }
    }
}

pub fn BrotliDecoderDecompress(
    encoded_size: usize,
    encoded_buffer: Ptr<u8>,
    decoded_size: Ptr<usize>,
    decoded_buffer: Ptr<u8>,
) -> BrotliDecoderResult {
    let input = encoded_buffer.with_slice(encoded_size, |s| s.to_vec());
    match decompress(&input, decoded_size.read()) {
        Some(out) => {
            decoded_buffer.with_slice_mut(out.len(), |d| d.copy_from_slice(&out));
            decoded_size.write(out.len());
            BROTLI_DECODER_RESULT_SUCCESS
        }
        None => BROTLI_DECODER_RESULT_ERROR,
    }
}

pub fn BrotliDecoderDestroyInstance(state: Ptr<BrotliDecoderState>) {
    if state.is_null() {
        return;
    }
    state.with_mut(|s| s.release_taken());
    state.delete();
}

pub fn BrotliDecoderDecompressStream(
    state: Ptr<BrotliDecoderState>,
    available_in: Ptr<usize>,
    next_in: Ptr<Ptr<u8>>,
    available_out: Ptr<usize>,
    next_out: Ptr<Ptr<u8>>,
    total_out: Ptr<usize>,
) -> BrotliDecoderResult {
    let in_ptr = next_in.read();
    let input = in_ptr.with_slice(available_in.read(), |s| s.to_vec());
    let out_ptr = if next_out.is_null() {
        Ptr::null()
    } else {
        next_out.read()
    };
    let mut out = vec![
        0u8;
        if out_ptr.is_null() {
            0
        } else {
            available_out.read()
        }
    ];
    let mut avail_in = input.len();
    let mut avail_out = available_out.read();
    let mut input_offset = 0;
    let mut output_offset = 0;
    let mut total = 0;
    let result = state.with_mut(|s| {
        s.release_taken();
        BrotliDecompressStream(
            &mut avail_in,
            &mut input_offset,
            &input,
            &mut avail_out,
            &mut output_offset,
            &mut out,
            &mut total,
            &mut s.state,
        )
    });
    if output_offset > 0 {
        out_ptr.with_slice_mut(output_offset, |d| d.copy_from_slice(&out[..output_offset]));
        next_out.write(out_ptr.offset(output_offset));
    }
    if input_offset > 0 {
        next_in.write(in_ptr.offset(input_offset));
    }
    available_in.write(avail_in);
    available_out.write(avail_out);
    if !total_out.is_null() {
        total_out.write(total);
    }
    decoder_result(result)
}

pub fn BrotliDecoderTakeOutput(state: Ptr<BrotliDecoderState>, size: Ptr<usize>) -> Ptr<u8> {
    let mut n = size.read();
    let taken = state.with_mut(|s| {
        s.release_taken();
        let out = brotli_decompressor::BrotliDecoderTakeOutput(&mut s.state, &mut n);
        s.taken = Ptr::alloc_array(out.to_vec().into_boxed_slice());
        s.taken.clone()
    });
    size.write(n);
    taken
}

pub fn BrotliDecoderVersion() -> u32 {
    brotli::enc::encode::BrotliEncoderVersion()
}

// Raw pointer versions of the above for the unsafe model.

unsafe fn raw_slice<'a>(ptr: *const u8, len: usize) -> &'a [u8] {
    if len == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(ptr, len) }
    }
}

unsafe fn raw_slice_mut<'a>(ptr: *mut u8, len: usize) -> &'a mut [u8] {
    if len == 0 {
        &mut []
    } else {
        unsafe { std::slice::from_raw_parts_mut(ptr, len) }
    }
}

/// # Safety
///
/// Same contract as C's `BrotliEncoderCompress`.
pub unsafe fn BrotliEncoderCompress_unsafe(
    quality: i32,
    lgwin: i32,
    mode: BrotliEncoderMode,
    input_size: usize,
    input_buffer: *const u8,
    encoded_size: *mut usize,
    encoded_buffer: *mut u8,
) -> i32 {
    unsafe {
        let input = raw_slice(input_buffer, input_size);
        match compress(quality, lgwin, mode, input, *encoded_size) {
            Some(out) => {
                raw_slice_mut(encoded_buffer, out.len()).copy_from_slice(&out);
                *encoded_size = out.len();
                1
            }
            None => {
                *encoded_size = 0;
                0
            }
        }
    }
}

/// # Safety
///
/// Same contract as C's `BrotliDecoderDecompress`.
pub unsafe fn BrotliDecoderDecompress_unsafe(
    encoded_size: usize,
    encoded_buffer: *const u8,
    decoded_size: *mut usize,
    decoded_buffer: *mut u8,
) -> BrotliDecoderResult {
    unsafe {
        match decompress(raw_slice(encoded_buffer, encoded_size), *decoded_size) {
            Some(out) => {
                raw_slice_mut(decoded_buffer, out.len()).copy_from_slice(&out);
                *decoded_size = out.len();
                BROTLI_DECODER_RESULT_SUCCESS
            }
            None => BROTLI_DECODER_RESULT_ERROR,
        }
    }
}

pub fn BrotliDecoderCreateInstance_unsafe() -> *mut BrotliDecoderState {
    Box::into_raw(Box::default())
}

/// # Safety
///
/// Same contract as C's `BrotliDecoderDestroyInstance`.
pub unsafe fn BrotliDecoderDestroyInstance_unsafe(state: *mut BrotliDecoderState) {
    if !state.is_null() {
        drop(unsafe { Box::from_raw(state) });
    }
}

/// # Safety
///
/// Same contract as C's `BrotliDecoderDecompressStream`.
pub unsafe fn BrotliDecoderDecompressStream_unsafe(
    state: *mut BrotliDecoderState,
    available_in: *mut usize,
    next_in: *mut *const u8,
    available_out: *mut usize,
    next_out: *mut *mut u8,
    total_out: *mut usize,
) -> BrotliDecoderResult {
    unsafe {
        let input = raw_slice(*next_in, *available_in);
        let output = if next_out.is_null() {
            &mut []
        } else {
            raw_slice_mut(*next_out, *available_out)
        };
        let mut input_offset = 0;
        let mut output_offset = 0;
        let mut total = 0;
        let result = BrotliDecompressStream(
            &mut *available_in,
            &mut input_offset,
            input,
            &mut *available_out,
            &mut output_offset,
            output,
            &mut total,
            &mut (*state).state,
        );
        *next_in = (*next_in).wrapping_add(input_offset);
        if !next_out.is_null() {
            *next_out = (*next_out).wrapping_add(output_offset);
        }
        if !total_out.is_null() {
            *total_out = total;
        }
        decoder_result(result)
    }
}

/// # Safety
///
/// Same contract as C's `BrotliDecoderTakeOutput`: the returned buffer is owned
/// by the state, and valid until the next call on it.
pub unsafe fn BrotliDecoderTakeOutput_unsafe(
    state: *mut BrotliDecoderState,
    size: *mut usize,
) -> *const u8 {
    unsafe {
        brotli_decompressor::BrotliDecoderTakeOutput(&mut (*state).state, &mut *size).as_ptr()
    }
}

/// # Safety
///
/// Same contract as C's `BrotliDecoderGetErrorCode`.
pub unsafe fn BrotliDecoderGetErrorCode_unsafe(
    state: *const BrotliDecoderState,
) -> BrotliDecoderErrorCode {
    unsafe { (*state).state.error_code as BrotliDecoderErrorCode }
}
