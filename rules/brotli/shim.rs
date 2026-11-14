// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#![allow(non_snake_case)]

use crate::{ByteRepr, Ptr};
use brotli::enc::backward_references::BrotliEncoderMode;
use brotli::enc::{BrotliCompress, BrotliEncoderParams, StandardAlloc};
use brotli_decompressor::{BrotliDecompress, BrotliDecompressStream, BrotliResult};

const BROTLI_DECODER_RESULT_ERROR: u32 = 0;
const BROTLI_DECODER_RESULT_SUCCESS: u32 = 1;
const BROTLI_DECODER_RESULT_NEEDS_MORE_INPUT: u32 = 2;
const BROTLI_DECODER_RESULT_NEEDS_MORE_OUTPUT: u32 = 3;

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

pub fn BrotliEncoderCompress(
    quality: i32,
    lgwin: i32,
    mode: u32,
    input_size: usize,
    input_buffer: Ptr<u8>,
    encoded_size: Ptr<usize>,
    encoded_buffer: Ptr<u8>,
) -> i32 {
    let params = BrotliEncoderParams {
        quality,
        lgwin,
        mode: match mode {
            1 => BrotliEncoderMode::BROTLI_MODE_TEXT,
            2 => BrotliEncoderMode::BROTLI_MODE_FONT,
            _ => BrotliEncoderMode::BROTLI_MODE_GENERIC,
        },
        size_hint: input_size,
        large_window: lgwin > 24,
        ..Default::default()
    };
    let input = input_buffer.with_slice(input_size, |s| s.to_vec());
    let mut out = Vec::new();
    match BrotliCompress(&mut input.as_slice(), &mut out, &params) {
        Ok(_) if out.len() <= encoded_size.read() => {
            encoded_buffer.with_slice_mut(out.len(), |d| d.copy_from_slice(&out));
            encoded_size.write(out.len());
            1
        }
        _ => {
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
) -> u32 {
    let input = encoded_buffer.with_slice(encoded_size, |s| s.to_vec());
    let mut out = Vec::new();
    match BrotliDecompress(&mut input.as_slice(), &mut out) {
        Ok(()) if out.len() <= decoded_size.read() => {
            decoded_buffer.with_slice_mut(out.len(), |d| d.copy_from_slice(&out));
            decoded_size.write(out.len());
            BROTLI_DECODER_RESULT_SUCCESS
        }
        _ => BROTLI_DECODER_RESULT_ERROR,
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
) -> u32 {
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
    match result {
        BrotliResult::ResultSuccess => BROTLI_DECODER_RESULT_SUCCESS,
        BrotliResult::NeedsMoreInput => BROTLI_DECODER_RESULT_NEEDS_MORE_INPUT,
        BrotliResult::NeedsMoreOutput => BROTLI_DECODER_RESULT_NEEDS_MORE_OUTPUT,
        BrotliResult::ResultFailure => BROTLI_DECODER_RESULT_ERROR,
    }
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
