// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::{env, fs, path::PathBuf};

fn main() {
    let crate_root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let rules_dir = crate_root.join("../rules").canonicalize().unwrap();
    let rule_shims_rs = PathBuf::from(env::var("OUT_DIR").unwrap()).join("rule_shims.rs");

    // Collect all rules/<dir>/shim.rs files
    let mut rule_dirs = Vec::new();
    for e in fs::read_dir(&rules_dir).unwrap().flatten() {
        let p = e.path();
        let name = p.file_name().and_then(|s| s.to_str()).unwrap_or("");
        if p.is_dir() && !matches!(name, "target" | "src" | ".git") {
            rule_dirs.push(p);
        }
    }
    rule_dirs.sort();

    // Rebuild when a rule directory changes, so newly added shim.rs files are picked up
    for d in &rule_dirs {
        println!("cargo:rerun-if-changed={}", d.display());
    }
    println!("cargo:rerun-if-changed=build.rs");

    let mut buf = String::new();
    for d in rule_dirs {
        let shim = d.join("shim.rs");
        if !shim.is_file() {
            continue;
        }

        let module_name = format!(
            "rule_{}",
            d.file_name()
                .unwrap()
                .to_string_lossy()
                .replace(['.', '-'], "_")
        );

        buf.push_str(&format!(
            "#[path = r#\"{}\"#]\nmod {};\npub use {}::*;\n",
            shim.display(),
            module_name,
            module_name
        ));
    }

    fs::write(&rule_shims_rs, buf).unwrap();
}
