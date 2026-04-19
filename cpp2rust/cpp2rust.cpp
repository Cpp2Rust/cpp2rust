// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <algorithm>
#include <array>
#include <cstdlib>
#include <filesystem>
#include <fstream>
#include <vector>
#if defined(_WIN32)
#include <windows.h>
#elif defined(__linux__)
#include <limits.h>
#include <unistd.h>
#elif defined(__APPLE__)
#include <mach-o/dyld.h>
#endif

#include <llvm/Support/CommandLine.h>

#include "cpp2rust_lib.h"

// LLVM's BumpPtrAllocator intentionally poisons freed slab memory via
// __asan_poison_memory_region.  When ASan is enabled this can produce
// use-after-poison false positives inside clang/LLVM itself (e.g. during
// IdentifierTable initialisation).  Opting out of user-poisoning suppresses
// those LLVM-internal false positives without affecting real UAF detection.
#if defined(__has_feature) && __has_feature(address_sanitizer)
extern "C" const char *__asan_default_options() {
  return "allow_user_poisoning=0";
}
#endif

namespace fs = std::filesystem;

namespace {
llvm::cl::OptionCategory cpp2rust_cmdargs("Cpp2Rust options");

llvm::cl::opt<std::string> CcFile("file",
                                  llvm::cl::desc("Path to the C++ file"),
                                  llvm::cl::value_desc("file.cpp"),
                                  llvm::cl::cat(cpp2rust_cmdargs));

llvm::cl::opt<std::string>
    BuildDir("dir",
             llvm::cl::desc("Directory that contains compile_commands.json"),
             llvm::cl::value_desc("dir"), llvm::cl::cat(cpp2rust_cmdargs));

llvm::cl::opt<std::string> RsFile("o", llvm::cl::desc("Path to the Rust file"),
                                  llvm::cl::value_desc("output.rs"),
                                  llvm::cl::Required,
                                  llvm::cl::cat(cpp2rust_cmdargs));

llvm::cl::opt<std::string>
    Model("model",
          llvm::cl::desc(
              "Name of the translation model (unsafe, refcount [default])"),
          llvm::cl::value_desc("model"), llvm::cl::init("refcount"),
          llvm::cl::cat(cpp2rust_cmdargs));

llvm::cl::opt<std::string>
    RulesDir("rules",
             llvm::cl::desc("Directory where translation rules are located"),
             llvm::cl::value_desc("rules"), llvm::cl::cat(cpp2rust_cmdargs));

llvm::cl::list<std::string> CXXFlags("cxxflags",
                                     llvm::cl::desc("Additional CXXFLAGS"),
                                     llvm::cl::value_desc("cxxflags"),
                                     llvm::cl::ZeroOrMore,
                                     llvm::cl::cat(cpp2rust_cmdargs));

} // namespace

// Get the directory of the running executable
static fs::path GetExecutableDir() {
#if defined(_WIN32)
  char path[MAX_PATH];
  GetModuleFileNameA(NULL, path, MAX_PATH);
  return fs::path(path).parent_path();
#elif defined(__linux__)
  char path[PATH_MAX];
  ssize_t count = readlink("/proc/self/exe", path, PATH_MAX);
  return fs::path(std::string_view(path, std::max((ssize_t)0, count)))
      .parent_path();
#elif defined(__APPLE__)
  uint32_t size = 0;
  _NSGetExecutablePath(nullptr, &size); // get path length
  std::vector<char> buffer(size);
  _NSGetExecutablePath(buffer.data(), &size);
  return fs::path(buffer.data()).parent_path();
#endif
  return ".";
}

static bool ResolveRulesDir() {
  std::array<fs::path, 3> candidates = {fs::path("./rules"),
                                        fs::path("../rules"),
                                        GetExecutableDir() / "../../rules"};

  for (const auto &dir : candidates) {
    if (fs::exists(dir) && fs::is_directory(dir)) {
      RulesDir = fs::canonical(dir).string();
      llvm::outs() << "Using rules directory: " << RulesDir << '\n';
      return true;
    }
  }
  return false;
}

int main(int argc, char *argv[]) {
  llvm::cl::HideUnrelatedOptions(cpp2rust_cmdargs);
  llvm::cl::ParseCommandLineOptions(argc, argv);

  if (CcFile.empty() && BuildDir.empty()) {
    llvm::errs() << "ERROR: please provide either --file or --dir\n";
    return EXIT_FAILURE;
  }

  if (!CcFile.empty() && !BuildDir.empty()) {
    llvm::errs() << "ERROR: please provide only one of --file or --dir\n";
    return EXIT_FAILURE;
  }

  if (!BuildDir.empty() && !CXXFlags.empty()) {
    llvm::errs() << "ERROR: can't combine --dir with --cxxflags\n";
    return EXIT_FAILURE;
  }

  auto model = cpp2rust::Model::kRefCount;
  if (Model == "refcount") {
    // ok
  } else if (Model == "unsafe") {
    model = cpp2rust::Model::kUnsafe;
  } else {
    llvm::errs() << "ERROR: unknown model: " << Model << "\n";
    return EXIT_FAILURE;
  }

  std::string cc_code;
  if (!CcFile.empty()) {
    std::ifstream file(CcFile);
    if (!file) {
      llvm::errs() << "ERROR: failed to open " << CcFile << "\n";
      return EXIT_FAILURE;
    }
    cc_code = {std::istreambuf_iterator<char>(file),
               std::istreambuf_iterator<char>()};
    if (cc_code.empty()) {
      llvm::errs() << "ERROR: empty source file\n";
      return EXIT_FAILURE;
    }
  }

  std::vector<std::string_view> cxx_flags(CXXFlags.begin(), CXXFlags.end());

  if (RulesDir.empty() && !ResolveRulesDir()) {
    llvm::errs() << "ERROR: rules directory not found. "
                    "Please specify one with --rules\n";
    return EXIT_FAILURE;
  }

  auto rs_code =
      BuildDir.empty()
          ? cpp2rust::TranspileSrc(cc_code, model, cxx_flags, RulesDir)
          : cpp2rust::TranspileDir(BuildDir, model, RulesDir);

  if (rs_code.empty()) {
    llvm::errs() << "ERROR: empty output file\n";
    return EXIT_FAILURE;
  }

  std::ofstream file(RsFile);
  if (!file) {
    llvm::errs() << "ERROR: failed to open " << RsFile << "\n";
    return EXIT_FAILURE;
  }

  file << rs_code;
  file.close();

  // call rustfmt.
  std::string rustfmt_command = "rustfmt " + RsFile;
  if (std::system(rustfmt_command.c_str()) != 0) {
    llvm::errs() << "ERROR: failed to run rustfmt\n";
    return EXIT_FAILURE;
  }

  return EXIT_SUCCESS;
}
