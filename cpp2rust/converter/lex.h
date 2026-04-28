#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

namespace cpp2rust {
namespace token {
inline constexpr char kOpenCurlyBracket[] = "{\n";
inline constexpr char kCloseCurlyBracket[] = "\n}\n";
inline constexpr char kOpenBracket[] = "[";
inline constexpr char kCloseBracket[] = "]";
inline constexpr char kSemiColon[] = ";\n";
inline constexpr char kComma[] = ",";
inline constexpr char kColon[] = ":";
inline constexpr char kDoubleColon[] = "::";
inline constexpr char kAssign[] = "=";
inline constexpr char kOpenParen[] = "(";
inline constexpr char kCloseParen[] = ")";
inline constexpr char kDot[] = ".";
inline constexpr char kNot[] = "!";
inline constexpr char kDiff[] = "!=";
inline constexpr char kZero[] = "0";
inline constexpr char kOne[] = "1";
inline constexpr char kRef[] = "&";
inline constexpr char kStar[] = "*";
inline constexpr char kArrow[] = "->";
inline constexpr char kHash[] = "#";
inline constexpr char kMinus[] = "-";
inline constexpr char kDiv[] = "/";
inline constexpr char kLt[] = "<";
inline constexpr char kGt[] = ">";
inline constexpr char kEmpty[] = "";
inline constexpr char kNewLine[] = "\n";
} // namespace token

namespace keyword {
inline constexpr const char *kAs = "as";
inline constexpr const char *kBreak = "break";
inline constexpr const char *kContinue = "continue";
inline constexpr const char *kConst = "const";
inline constexpr const char *kElse = "else";
inline constexpr const char *kFalse = "false";
inline constexpr const char *kFn = "fn";
inline constexpr const char *kIf = "if";
inline constexpr const char *kImpl = "impl";
inline constexpr const char *kLet = "let";
inline constexpr const char *kLoop = "loop";
inline constexpr const char *kPub = "pub";
inline constexpr const char *kModule = "mod";
inline constexpr const char *kReturn = "return";
inline constexpr const char *kSelfValue = "self";
inline constexpr const char *kStatic = "static";
inline constexpr const char *kStruct = "struct";
inline constexpr const char *kTrue = "true";
inline constexpr const char *kWhile = "while";
inline constexpr const char *kFor = "for";
inline constexpr const char *kIn = "in";
inline constexpr const char *kTrait = "trait";
inline constexpr const char *kDyn = "dyn";
} // namespace keyword
} // namespace cpp2rust
