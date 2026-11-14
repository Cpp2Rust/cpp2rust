// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

// The subset of the brotli C API (<brotli/decode.h> and <brotli/encode.h>)
// that the rules use.

#include <stddef.h>
#include <stdint.h>

#define BROTLI_BOOL int
#define BROTLI_ARRAY_PARAM(name)

typedef void *(*brotli_alloc_func)(void *opaque, size_t size);
typedef void (*brotli_free_func)(void *opaque, void *address);

typedef struct BrotliDecoderStateStruct BrotliDecoderState;

typedef enum {
  BROTLI_DECODER_RESULT_ERROR = 0,
  BROTLI_DECODER_RESULT_SUCCESS = 1,
  BROTLI_DECODER_RESULT_NEEDS_MORE_INPUT = 2,
  BROTLI_DECODER_RESULT_NEEDS_MORE_OUTPUT = 3
} BrotliDecoderResult;

typedef enum {
  BROTLI_DECODER_NO_ERROR = 0,
  BROTLI_DECODER_ERROR_FORMAT_EXUBERANT_NIBBLE = -1,
  BROTLI_DECODER_ERROR_FORMAT_EXUBERANT_META_NIBBLE = -3,
  BROTLI_DECODER_ERROR_FORMAT_SIMPLE_HUFFMAN_ALPHABET = -4,
  BROTLI_DECODER_ERROR_FORMAT_SIMPLE_HUFFMAN_SAME = -5,
  BROTLI_DECODER_ERROR_FORMAT_CL_SPACE = -6,
  BROTLI_DECODER_ERROR_FORMAT_HUFFMAN_SPACE = -7,
  BROTLI_DECODER_ERROR_FORMAT_CONTEXT_MAP_REPEAT = -8,
  BROTLI_DECODER_ERROR_FORMAT_BLOCK_LENGTH_1 = -9,
  BROTLI_DECODER_ERROR_FORMAT_BLOCK_LENGTH_2 = -10,
  BROTLI_DECODER_ERROR_FORMAT_TRANSFORM = -11,
  BROTLI_DECODER_ERROR_FORMAT_DICTIONARY = -12,
  BROTLI_DECODER_ERROR_FORMAT_WINDOW_BITS = -13,
  BROTLI_DECODER_ERROR_FORMAT_PADDING_1 = -14,
  BROTLI_DECODER_ERROR_FORMAT_PADDING_2 = -15,
  BROTLI_DECODER_ERROR_INVALID_ARGUMENTS = -20,
  BROTLI_DECODER_ERROR_ALLOC_CONTEXT_MODES = -21,
  BROTLI_DECODER_ERROR_ALLOC_TREE_GROUPS = -22,
  BROTLI_DECODER_ERROR_ALLOC_CONTEXT_MAP = -25,
  BROTLI_DECODER_ERROR_ALLOC_RING_BUFFER_1 = -26,
  BROTLI_DECODER_ERROR_ALLOC_RING_BUFFER_2 = -27,
  BROTLI_DECODER_ERROR_ALLOC_BLOCK_TYPE_TREES = -30
} BrotliDecoderErrorCode;

typedef enum BrotliEncoderMode {
  BROTLI_MODE_GENERIC = 0,
  BROTLI_MODE_TEXT = 1,
  BROTLI_MODE_FONT = 2
} BrotliEncoderMode;

BrotliDecoderState *BrotliDecoderCreateInstance(brotli_alloc_func alloc_func,
                                                brotli_free_func free_func,
                                                void *opaque);
void BrotliDecoderDestroyInstance(BrotliDecoderState *state);
BrotliDecoderResult BrotliDecoderDecompress(size_t encoded_size,
                                            const uint8_t *encoded_buffer,
                                            size_t *decoded_size,
                                            uint8_t *decoded_buffer);
BrotliDecoderResult
BrotliDecoderDecompressStream(BrotliDecoderState *state, size_t *available_in,
                              const uint8_t **next_in, size_t *available_out,
                              uint8_t **next_out, size_t *total_out);
const uint8_t *BrotliDecoderTakeOutput(BrotliDecoderState *state, size_t *size);
BrotliDecoderErrorCode
BrotliDecoderGetErrorCode(const BrotliDecoderState *state);
uint32_t BrotliDecoderVersion(void);
BROTLI_BOOL BrotliEncoderCompress(int quality, int lgwin,
                                  BrotliEncoderMode mode, size_t input_size,
                                  const uint8_t *input_buffer,
                                  size_t *encoded_size,
                                  uint8_t *encoded_buffer);
