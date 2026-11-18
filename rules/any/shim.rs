// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::ByteRepr;

impl ByteRepr for std::rc::Rc<dyn std::any::Any> {}
