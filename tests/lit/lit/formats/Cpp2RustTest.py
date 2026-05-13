# Copyright (c) 2022-present INESC-ID.
# Distributed under the MIT license that can be found in the LICENSE file.

import lit.Test
import lit.util
from .base import TestFormat
import difflib
import os
import re
import shutil
import random


def read_rust_version():
    toolchain_path = os.path.join(
        os.path.dirname(__file__), "../../../../cmake/rust-toolchain.cmake"
    )
    with open(toolchain_path, "r") as f:
        for line in f:
            m = re.match(r'set\s*\(\s*RUST_VERSION\s+"([^"]+)', line)
            if m:
                return m.group(1)
    raise Exception("could not find rust version in " + toolchain_path)


def shared_target_dir():
    return os.path.abspath(
        os.path.join(os.path.dirname(__file__), "../../../../build/tmp/cargo-target")
    )


def cargo_env():
    return dict(os.environ, CARGO_TARGET_DIR=os.path.abspath(shared_target_dir()))


class Cpp2RustTest(TestFormat):
    def __init__(self):
        self.regex_xfail = re.compile(r"//\s*XFAIL:\s*(.*)")
        self.regex_panic = re.compile(r"//\s*panic\s*(?::\s*(.*))?$", re.MULTILINE)
        self.regex_nocompile = re.compile(
            r"//\s*no-compile\s*(?::\s*(.*))?$", re.MULTILINE
        )
        self.regex_translation_fail = re.compile(
            r"//\s*translation-fail\s*(?::\s*(.*))?$", re.MULTILINE
        )
        self.regex_nondet_result = re.compile(
            r"//\s*nondet-result\s*(?::\s*(.*))?$", re.MULTILINE
        )
        self.rust_version = read_rust_version()
        os.environ["RUSTFLAGS"] = "-Awarnings"

    def updateExpected(self, generated, expected_path):
        os.makedirs(os.path.dirname(expected_path), exist_ok=True)
        with open(expected_path, "w") as f:
            f.write(generated)

    def getExpectedFile(self, filepath, model, fname):
        return filepath + "/out/" + model + "/" + fname + ".rs"

    def getTestsForPath(self, testSuite, path_in_suite, litConfig, localConfig):
        source_path = testSuite.getSourcePath(path_in_suite)
        for model in ["refcount", "unsafe"]:
            yield lit.Test.Test(
                testSuite, path_in_suite + (model,), localConfig, source_path
            )

    def getTestsInDirectory(self, testSuite, path_in_suite, litConfig, localConfig):
        source_path = testSuite.getSourcePath(path_in_suite)
        for filename in os.listdir(source_path):
            if filename.endswith(".cpp") or filename.endswith(".c"):
                for t in self.getTestsForPath(
                    testSuite, path_in_suite + (filename,), litConfig, localConfig
                ):
                    yield t

    def execute(self, test, litConfig):
        replace_expected = os.environ.get("REPLACE_EXPECTED", False)
        skip_run = os.environ.get("SKIP_RUN", False)

        cc_input = test.getFilePath()
        fname = os.path.splitext(os.path.basename(cc_input))[0]
        filepath = os.path.dirname(cc_input)
        model = test.getSourcePath().split("/")[-1]

        with open(cc_input, "r") as f:
            text = f.read()

        def matches_model(match, model):
            if match is None:
                return False
            models = match.group(1)
            if models is None or models.strip() == "":
                return True
            return model in re.split(r"\s*,\s*", models.strip())

        should_fail = False
        fail_code = lit.Test.FAIL
        xfail = self.regex_xfail.search(text)
        if xfail:
            xfail = re.split(r"\s*,\s*", xfail.group(1))
            should_fail = model in xfail
            fail_code = lit.Test.XFAIL

        should_panic = matches_model(self.regex_panic.search(text), model)
        should_not_compile = matches_model(self.regex_nocompile.search(text), model)
        should_not_translate = matches_model(
            self.regex_translation_fail.search(text), model
        )
        is_nondet_result = matches_model(self.regex_nondet_result.search(text), model)

        tmp_dir = (
            "tmp/" + fname + "-" + model + "_" + format(random.getrandbits(64), "x")
        )
        rs_file = tmp_dir + "/src/main.rs"
        shutil.rmtree(tmp_dir, True)
        os.makedirs(tmp_dir + "/src")

        def fail(str, code=fail_code):
            shutil.rmtree(tmp_dir, True)
            return code, str

        cmd = ["./cpp2rust/cpp2rust", "-file", cc_input, "-model", model, "-o", rs_file]

        out, err, returncode = lit.util.executeCommand(cmd)

        if not os.path.exists(rs_file):
            return fail(
                "no out file (rc="
                + str(returncode)
                + ")\n"
                + "cmd: "
                + " ".join(cmd)
                + "\n"
                + "\nstderr: "
                + err
                + "\nstdout: "
                + out
            )

        with open(rs_file, "r") as f:
            generated = f.read()

        if returncode != 0:
            if should_not_translate:
                return lit.Test.XFAIL, ""
            return fail("cpp2rust failed\n" + err)

        if should_not_translate:
            return fail("expected translation-fail but cpp2rust succeeded")

        if not should_not_compile:
            expected_file = self.getExpectedFile(filepath, model, fname)
            if not os.path.exists(expected_file) and not replace_expected:
                return fail("no expected file")

            if replace_expected:
                self.updateExpected(generated, expected_file)

            with open(expected_file, "r") as f:
                expected = f.read()

            if generated != expected:
                diff = "".join(
                    difflib.unified_diff(
                        expected.splitlines(keepends=True),
                        generated.splitlines(keepends=True),
                        fromfile="expected",
                        tofile="generated",
                    )
                )
                return fail("different output\n" + diff)

        pkg_name = "test_" + re.sub(r"[^a-zA-Z0-9_]", "_", os.path.basename(tmp_dir))

        # Check if we can compile the rust file
        with open(tmp_dir + "/Cargo.toml", "w") as f:
            f.write(f"""
[package]
name = "{pkg_name}"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "{pkg_name}"
path = "src/main.rs"

[dependencies]
libc = "0.2.169"
libcc2rs = {{ path = "../../../libcc2rs" }}
""")

        cmd = ["cargo", "+" + self.rust_version, "build", "--release", "--quiet"]
        _, err, returncode = lit.util.executeCommand(cmd, tmp_dir, env=cargo_env())
        if should_not_compile:
            if returncode != 0:
                shutil.rmtree(tmp_dir, True)
                return lit.Test.XFAIL, ""
            return fail("expected no-compile but compiled successfully")
        if returncode != 0:
            return fail("cargo failed\n" + err)

        rust_bin = os.path.join(shared_target_dir(), "release", pkg_name)

        if not skip_run:
            if should_panic:
                _, err, returncode = lit.util.executeCommand(rust_bin)
                err = str(err)
                if (
                    not re.search(r"thread 'main' \(\d+\) panicked at", err)
                    or returncode != 101
                ):
                    return fail("expected panic\n" + err)
            elif is_nondet_result:
                lit.util.executeCommand(rust_bin)
            else:
                if cc_input.endswith(".c"):
                    cc = os.environ.get("CC", "clang")
                else:
                    cc = os.environ.get("CXX", "clang++")
                cmd = [cc, "-O3", "-o", tmp_dir + "/cpp", cc_input]
                _, _, code = lit.util.executeCommand(cmd)
                if code != 0:
                    return fail(cc + " failed")

                out_cpp, err_cpp, code_cpp = lit.util.executeCommand(tmp_dir + "/cpp")
                out_rs, err_rs, code_rs = lit.util.executeCommand(rust_bin)
                # Normalize pointer addresses (0x...) since they differ between C++ and Rust
                ptr_re = re.compile(r"0x[0-9a-fA-F]+")
                out_cpp_cmp = ptr_re.sub("0xPTR", out_cpp)
                out_rs_cmp = ptr_re.sub("0xPTR", out_rs)
                if (
                    out_cpp_cmp != out_rs_cmp
                    or code_cpp != code_rs
                    or err_rs != err_cpp
                ):
                    return fail(
                        "different output\n" + out_cpp + err_cpp + out_rs + err_rs
                    )

        if should_fail:
            return fail("did not fail as expected", lit.Test.FAIL)

        shutil.rmtree(tmp_dir, True)
        return lit.Test.PASS, ""
