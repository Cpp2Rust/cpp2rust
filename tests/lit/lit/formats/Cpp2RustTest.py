# Copyright (c) 2022-present INESC-ID.
# Distributed under the MIT license that can be found in the LICENSE file.

import lit.Test
import lit.util
from .base import TestFormat
from dataclasses import dataclass
from pathlib import Path
from typing import NamedTuple, Optional
import difflib
import os
import re
import shutil
import tomli


MODELS = ("refcount", "unsafe")
PTR_RE = re.compile(r"0x[0-9a-fA-F]+")

RE_XFAIL = re.compile(r"//\s*XFAIL:\s*(.*)")
RE_PANIC = re.compile(r"//\s*panic\s*(?::\s*(.*))?$", re.MULTILINE)
RE_NOCOMPILE = re.compile(r"//\s*no-compile\s*(?::\s*(.*))?$", re.MULTILINE)
RE_TRANS_FAIL = re.compile(r"//\s*translation-fail\s*(?::\s*(.*))?$", re.MULTILINE)
RE_NONDET = re.compile(r"//\s*nondet-result\s*(?::\s*(.*))?$", re.MULTILINE)


@dataclass
class TestExpectations:
    should_panic: bool = False
    should_not_compile: bool = False
    should_not_translate: bool = False
    is_nondet_result: bool = False
    xfail: bool = False
    fail_code: int = lit.Test.FAIL

    @classmethod
    def parse(cls, text, model):
        def matches(match):
            if match is None:
                return False
            models = match.group(1)
            if models is None or models.strip() == "":
                return True
            return model in re.split(r"\s*,\s*", models.strip())
        os.path.join(os.path.dirname(__file__), "../../../../build/tmp/cargo-target")
    )


def cargo_env():
    return dict(os.environ, CARGO_TARGET_DIR=os.path.abspath(shared_target_dir()))

        xfail_m = RE_XFAIL.search(text)
        xfail = xfail_m is not None and model in re.split(r"\s*,\s*", xfail_m.group(1))
        return cls(
            should_panic=matches(RE_PANIC.search(text)),
            should_not_compile=matches(RE_NOCOMPILE.search(text)),
            should_not_translate=matches(RE_TRANS_FAIL.search(text)),
            is_nondet_result=matches(RE_NONDET.search(text)),
            xfail=xfail,
            fail_code=lit.Test.XFAIL if xfail else lit.Test.FAIL,
        )


class RunResult(NamedTuple):
    stdout: str
    stderr: str
    returncode: int


@dataclass
class TestContext:
    cc_input: Path
    is_multi: bool
    fname: str
    filepath: Path
    model: str
    tmp_dir: Path
    rs_file: Path
    expectations: TestExpectations
    replace_expected: bool = False
    skip_run: bool = False
    build_dir: Optional[Path] = None
    generated: Optional[str] = None
    pkg_name: Optional[str] = None
    cpp_bin: Optional[Path] = None
    rust_bin: Optional[Path] = None
    cpp_result: Optional[RunResult] = None
    rust_result: Optional[RunResult] = None

    @classmethod
    def setup(cls, test):
        cc_input = Path(test.getFilePath())
        is_multi = is_multi_file_test(cc_input)
        model = test.getSourcePath().split("/")[-1]
        fname = cc_input.name if is_multi else cc_input.stem
        tmp_dir = Path("tmp") / f"{fname}-{model}"

        shutil.rmtree(tmp_dir, ignore_errors=True)
        (tmp_dir / "src").mkdir(parents=True)

        return cls(
            cc_input=cc_input,
            is_multi=is_multi,
            fname=fname,
            filepath=cc_input if is_multi else cc_input.parent,
            model=model,
            tmp_dir=tmp_dir,
            rs_file=tmp_dir / "src" / "main.rs",
            expectations=TestExpectations.parse(load_source_text(cc_input), model),
            replace_expected=bool(os.environ.get("REPLACE_EXPECTED", False)),
            skip_run=bool(os.environ.get("SKIP_RUN", False)),
        )

    def translate(self):
        exp = self.expectations
        if self.is_multi:
            build_dir, err = setup_build_dir(self.tmp_dir, self.cc_input)
            if err is not None:
                return (exp.fail_code, err)
            self.build_dir = build_dir

        cmd = cpp2rust_command(self.cc_input, self.build_dir, self.model, self.rs_file)
        out, err, returncode = lit.util.executeCommand(cmd)

        if not self.rs_file.exists():
            return (
                exp.fail_code,
                "no out file (rc="
                + str(returncode)
                + ")\n"
                + "cmd: "
                + " ".join(cmd)
                + "\n"
                + "\nstderr: "
                + err
                + "\nstdout: "
                + out,
            )

        if returncode != 0:
            if exp.should_not_translate:
                return (lit.Test.XFAIL, "")
            return (exp.fail_code, "cpp2rust failed\n" + err)

        if exp.should_not_translate:
            return (exp.fail_code, "expected translation-fail but cpp2rust succeeded")

        self.generated = self.rs_file.read_text()
        return None

    def check_expected(self):
        exp = self.expectations
        # We don't care if no-compile tests have a corresponding generated file.
        if exp.should_not_compile:
            return None

        expected_file = get_expected_file(self.filepath, self.model, self.fname)
        if not expected_file.exists() and not self.replace_expected:
            return (exp.fail_code, "no expected file")

        if self.replace_expected:
            update_expected(self.generated, expected_file)

        expected = expected_file.read_text()

        if self.generated != expected:
            diff = "".join(
                difflib.unified_diff(
                    expected.splitlines(keepends=True),
                    self.generated.splitlines(keepends=True),
                    fromfile="expected",
                    tofile="generated",
                )
            )
            return (exp.fail_code, "different output\n" + diff)
        return None

    def build_cpp(self):
        exp = self.expectations
        if self.build_dir is not None:
            cmd = ["cmake", "--build", str(self.build_dir)]
            _, _, rc = lit.util.executeCommand(cmd)
            if rc != 0:
                return (exp.fail_code, "cmake build failed")
            self.cpp_bin = self.build_dir / "app"
            return None

        cc = (
            os.environ.get("CC", "clang")
            if self.cc_input.suffix == ".c"
            else os.environ.get("CXX", "clang++")
        )
        self.cpp_bin = self.tmp_dir / "cpp"
        cmd = [cc, "-O3", "-o", str(self.cpp_bin), str(self.cc_input)]
        _, _, rc = lit.util.executeCommand(cmd)
        if rc != 0:
            return (exp.fail_code, cc + " failed")
        return None

    def build_rust(self):
        exp = self.expectations
        rust_version = read_rust_version()
        self.pkg_name = "test_" + re.sub(r"[^a-zA-Z0-9_]", "_", self.tmp_dir.name)

        (self.tmp_dir / "rust-toolchain.toml").write_text(
            f'[toolchain]\nchannel = "{rust_version}"\n'
        )
        (self.tmp_dir / "Cargo.toml").write_text(f"""
[package]
name = "{self.pkg_name}"
version = "0.1.0"
edition = "2021"
rust-version = "{rust_version}"

[[bin]]
name = "{self.pkg_name}"
path = "src/main.rs"

[dependencies]
libc = "0.2.169"
libcc2rs = {{ path = "../../../libcc2rs" }}
""")

        cmd = ["cargo", "build", "--release", "--quiet"]
        _, err, returncode = lit.util.executeCommand(
            cmd, str(self.tmp_dir), env=cargo_env()
        )
        if exp.should_not_compile:
            if returncode != 0:
                return (lit.Test.XFAIL, "")
            return (exp.fail_code, "expected no-compile but compiled successfully")
        if returncode != 0:
            return (exp.fail_code, "cargo failed\n" + err)

        self.rust_bin = shared_target_dir() / "release" / self.pkg_name
        return None

    def run_cpp(self):
        if self.skip_run:
            return None
        self.cpp_result = RunResult(*lit.util.executeCommand(str(self.cpp_bin)))
        return None

    def run_rust(self):
        exp = self.expectations
        if self.skip_run:
            return None
        self.rust_result = RunResult(*lit.util.executeCommand(str(self.rust_bin)))

        if exp.should_panic:
            err = str(self.rust_result.stderr)
            if (
                not re.search(r"thread 'main' \(\d+\) panicked at", err)
                or self.rust_result.returncode != 101
            ):
                return (exp.fail_code, "expected panic\n" + err)
            return self.success_result()

        if exp.is_nondet_result:
            return self.success_result()
        return None

    def compare(self):
        exp = self.expectations
        if self.skip_run:
            return None

        cpp = self.cpp_result
        rs = self.rust_result
        out_cpp_cmp = PTR_RE.sub("0xPTR", cpp.stdout)
        out_rs_cmp = PTR_RE.sub("0xPTR", rs.stdout)
        if (
            out_cpp_cmp != out_rs_cmp
            or cpp.returncode != rs.returncode
            or cpp.stderr != rs.stderr
        ):
            return (
                exp.fail_code,
                "different output\n" + cpp.stdout + cpp.stderr + rs.stdout + rs.stderr,
            )
        return None

    def success_result(self):
        if self.expectations.xfail:
            return (lit.Test.FAIL, "did not fail as expected")
        return (lit.Test.PASS, "")

    def finalize(self, result):
        shutil.rmtree(self.tmp_dir, ignore_errors=True)
        return result


class Cpp2RustTest(TestFormat):
    def __init__(self):
        os.environ["RUSTFLAGS"] = "-Awarnings"

    def execute(self, test, litConfig):
        ctx = TestContext.setup(test)
        result = (
            ctx.translate()
            or ctx.check_expected()
            or ctx.build_cpp()
            or ctx.build_rust()
            or ctx.run_cpp()
            or ctx.run_rust()
            or ctx.compare()
            or ctx.success_result()
        )
        return ctx.finalize(result)

    def getTestsForPath(self, testSuite, path_in_suite, litConfig, localConfig):
        source_path = testSuite.getSourcePath(path_in_suite)
        for model in MODELS:
            yield lit.Test.Test(
                testSuite, path_in_suite + (model,), localConfig, source_path
            )

    def getTestsInDirectory(self, testSuite, path_in_suite, litConfig, localConfig):
        source_path = Path(testSuite.getSourcePath(path_in_suite))
        if is_multi_file_test(source_path):
            for t in self.getTestsForPath(
                testSuite, path_in_suite, litConfig, localConfig
            ):
                yield t
            return
        for entry in source_path.iterdir():
            if entry.is_file() and entry.suffix in (".cpp", ".c"):
                for t in self.getTestsForPath(
                    testSuite, path_in_suite + (entry.name,), litConfig, localConfig
                ):
                    yield t


def read_rust_version():
    toolchain_path = Path(__file__).parent / "../../../../libcc2rs/rust-toolchain.toml"
    return tomli.loads(toolchain_path.read_text())["toolchain"]["channel"]


def shared_target_dir():
    return (Path(__file__).parent / "../../../../build/tmp/cargo-target").resolve()


def cargo_env():
    return dict(os.environ, CARGO_TARGET_DIR=str(shared_target_dir()))


def is_multi_file_test(p):
    return p.is_dir() and (p / "CMakeLists.txt").exists()


def load_source_text(cc_input):
    if cc_input.is_dir():
        expectations_path = cc_input / "test.expectations"
        if not expectations_path.exists():
            return ""
        return "// " + expectations_path.read_text()
    return cc_input.read_text()


def setup_build_dir(tmp_dir, cc_input):
    build_dir = (tmp_dir / "cmake-build").resolve()
    build_dir.mkdir(parents=True, exist_ok=True)
    cmd = [
        "cmake",
        "-S",
        str(cc_input),
        "-B",
        str(build_dir),
        "-DCMAKE_EXPORT_COMPILE_COMMANDS=ON",
    ]
    _, err, rc = lit.util.executeCommand(cmd)
    if rc != 0:
        return None, "cmake configure failed\n" + err
    return build_dir, None


def cpp2rust_command(cc_input, build_dir, model, rs_file):
    if build_dir is not None:
        return [
            "./cpp2rust/cpp2rust",
            "-dir",
            str(build_dir),
            "-model",
            model,
            "-o",
            str(rs_file),
        ]
    return [
        "./cpp2rust/cpp2rust",
        "-file",
        str(cc_input),
        "-model",
        model,
        "-o",
        str(rs_file),
    ]


def get_expected_file(filepath, model, fname):
    return filepath / "out" / model / f"{fname}.rs"


def update_expected(generated, expected_path):
    expected_path.parent.mkdir(parents=True, exist_ok=True)
    expected_path.write_text(generated)
