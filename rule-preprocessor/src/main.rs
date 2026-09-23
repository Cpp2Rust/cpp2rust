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
    let mut args = std::env::args().skip(1);
    let out_dir = args
        .next()
        .expect("usage: rule-preprocessor <out-dir> <rule-dir>");
    let in_dir = args
        .next()
        .expect("usage: rule-preprocessor <out-dir> <rule-dir>");

    let out = std::path::PathBuf::from(out_dir);
    SemanticAnalysis::run(
        SyntacticAnalysis::run(&std::fs::canonicalize(&in_dir).unwrap()),
        &out,
    )
    .write_ir(&out);
}
