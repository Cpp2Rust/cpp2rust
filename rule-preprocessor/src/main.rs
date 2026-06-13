// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_infer;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_span;
extern crate rustc_trait_selection;

mod ir;
mod semantic;
mod syntactic;

use semantic::SemanticAnalysis;
use syntactic::SyntacticAnalysis;

fn main() {
    let out_dir = std::env::args()
        .nth(1)
        .expect("usage: rule-preprocessor <out-dir>");
    SemanticAnalysis::run(SyntacticAnalysis::run(
        &std::fs::canonicalize("../rules").unwrap(),
    ))
    .write_ir(&std::path::PathBuf::from(out_dir));
}
