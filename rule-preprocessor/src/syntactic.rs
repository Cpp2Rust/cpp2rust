// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use ra_ap_syntax::ast::{HasGenericParams, HasName, HasTypeBounds};
use ra_ap_syntax::{AstNode, SyntaxKind, ast, match_ast};
use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};

use crate::ir::{
    Access, BodyFragment, FileIr, FnIr, MethodCallInner, PlaceholderInner, RuleIr, RulesIR,
    TypeInfo, TypeIr,
};

/// Classify a type AST node as refcount pointer (`Ptr<T>`), unsafe pointer
/// (`*mut T`, `*const T`), or neither. References (`&T`, `&mut T`) are
/// classified based on their referent.
fn pointer_flags(ty: &ast::Type) -> (bool, bool) {
    let flags = match ty {
        ast::Type::PtrType(_) => (false, true),
        ast::Type::PathType(path) => {
            let is_ptr = path
                .path()
                .and_then(|p| p.segment())
                .and_then(|s| s.name_ref())
                .is_some_and(|name| name.text() == "Ptr");
            (is_ptr, false)
        }
        ast::Type::RefType(r) => r
            .ty()
            .map(|inner| pointer_flags(&inner))
            .unwrap_or_default(),
        _ => (false, false),
    };
    assert!(
        !(flags.0 && flags.1),
        "type cannot be both refcount and unsafe pointer"
    );
    flags
}

pub struct SyntacticAnalysis;

impl SyntacticAnalysis {
    pub fn run(crate_root: &Path) -> RulesIR {
        let rule_files = Self::collect_rule_files(crate_root);
        let mut all_ir = HashMap::new();
        let mut has_unknowns = false;

        for rule_file in &rule_files {
            let source = std::fs::read_to_string(rule_file).unwrap();
            let file_ir = Self::parse_rule_file(&source, rule_file);

            assert!(
                !file_ir.is_empty(),
                "Rule file {} produced no IR",
                rule_file.display()
            );

            has_unknowns |= file_ir.values().any(|r| match r {
                RuleIr::Fn(f) => f.has_unknowns(),
                RuleIr::Type(_) => false,
            });
            let canonical = rule_file
                .canonicalize()
                .unwrap_or_else(|_| rule_file.clone())
                .to_string_lossy()
                .to_string();
            all_ir.insert(canonical, file_ir);
        }

        RulesIR {
            all_ir,
            has_unknowns,
            crate_root: crate_root.to_path_buf(),
        }
    }

    fn collect_rule_files(dir: &Path) -> Vec<PathBuf> {
        let mut out = Vec::new();
        let Ok(entries) = std::fs::read_dir(dir) else {
            return out;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                out.extend(Self::collect_rule_files(&path));
            } else if let Some(name) = path.file_name().and_then(|s| s.to_str())
                && name.starts_with("tgt_")
                && name.ends_with(".rs")
            {
                out.push(path);
            }
        }
        out.sort();
        out
    }

    fn parse_rule_file(source: &str, path: &Path) -> FileIr {
        let parse = ra_ap_syntax::SourceFile::parse(source, ra_ap_syntax::Edition::Edition2024);
        let source_file = parse.tree();
        let mut file_ir = FileIr::new();

        for fn_item in source_file.syntax().descendants().filter_map(ast::Fn::cast) {
            let Some(name) = fn_item.name() else { continue };
            let fn_name = name.text().to_string();

            if fn_name == "types" {
                for (name, ir) in TypeIrBuilder::new(&fn_item).build() {
                    file_ir.insert(name, RuleIr::Type(ir));
                }
            } else if fn_name.starts_with('f') {
                file_ir.insert(
                    fn_name.clone(),
                    RuleIr::Fn(FnIrBuilder::new(&fn_item).build(path)),
                );
            }
        }

        file_ir
    }
}

struct FragmentCtx<'a> {
    builder: &'a FnIrBuilder<'a>,
    params: &'a [ParamInfo],
    generic_names: &'a [String],
    fragments: Vec<BodyFragment>,
    text_buf: String,
    pending_close_parens: usize,
}

impl<'a> FragmentCtx<'a> {
    fn new(
        builder: &'a FnIrBuilder<'a>,
        params: &'a [ParamInfo],
        generic_names: &'a [String],
    ) -> Self {
        Self {
            builder,
            params,
            generic_names,
            fragments: Vec::new(),
            text_buf: String::new(),
            pending_close_parens: 0,
        }
    }

    fn flush_text(&mut self) {
        if !self.text_buf.is_empty() {
            self.fragments.push(BodyFragment::Text {
                text: std::mem::take(&mut self.text_buf),
            });
        }
    }

    fn finish(mut self) -> Vec<BodyFragment> {
        self.flush_text();
        FnIrBuilder::trim_whitespaces(&mut self.fragments);
        self.fragments
    }

    fn visit(&mut self, child: ra_ap_syntax::SyntaxElement) {
        match child {
            ra_ap_syntax::NodeOrToken::Token(token) => self.visit_token(&token),
            ra_ap_syntax::NodeOrToken::Node(node) => {
                if let Some(call) = ast::MethodCallExpr::cast(node.clone()) {
                    self.emit_method_call(&call);
                    return;
                }
                for child in node.children_with_tokens() {
                    self.visit(child);
                }
            }
        }
    }

    fn visit_token(&mut self, token: &ra_ap_syntax::SyntaxToken) {
        if self.pending_close_parens > 0 && token.text() == ")" {
            self.pending_close_parens -= 1;
            return;
        }
        if token.kind() == SyntaxKind::IDENT {
            if let Some(param) = self.params.iter().find(|p| p.name == token.text()) {
                let mut access = self.builder.classify_access(token);
                if param.is_mut_ref && self.text_buf.ends_with('*') {
                    self.text_buf.pop();
                }
                if param.is_mut_ref && self.text_buf.ends_with("std::mem::take(&mut ") {
                    self.text_buf
                        .truncate(self.text_buf.len() - "std::mem::take(&mut ".len());
                    self.pending_close_parens += 1;
                    access = Access::Unknown;
                }
                self.flush_text();
                self.fragments.push(BodyFragment::Placeholder {
                    placeholder: PlaceholderInner {
                        arg: token.text()[1..].parse().unwrap_or(0),
                        access,
                    },
                });
                return;
            }
            let text = token.text().to_string();
            if self.generic_names.contains(&text) {
                self.flush_text();
                self.fragments.push(BodyFragment::Generic {
                    generic: text[1..].parse().unwrap_or(0),
                });
                return;
            }
        }
        self.text_buf.push_str(token.text());
    }

    fn emit_method_call(&mut self, call: &ast::MethodCallExpr) {
        assert!(
            call.receiver().is_some(),
            "MethodCallExpr does not have a receiver"
        );
        self.flush_text();

        let receiver = {
            let mut receiver_ctx = FragmentCtx::new(self.builder, self.params, self.generic_names);
            receiver_ctx.visit(ra_ap_syntax::NodeOrToken::Node(
                call.receiver().unwrap().syntax().clone(),
            ));
            receiver_ctx.finish()
        };

        let body = {
            let mut body_ctx = FragmentCtx::new(self.builder, self.params, self.generic_names);
            for child in call.syntax().children_with_tokens().skip(1) {
                body_ctx.visit(child);
            }
            body_ctx.finish()
        };

        self.fragments.push(BodyFragment::MethodCall {
            method_call: MethodCallInner { receiver, body },
        });
    }
}

struct ParamInfo {
    name: String,
    ty: String,
    is_refcount_pointer: bool,
    is_unsafe_pointer: bool,
    is_mut_ref: bool,
}

struct FnIrBuilder<'a> {
    fn_item: &'a ast::Fn,
}

impl<'a> FnIrBuilder<'a> {
    fn new(fn_item: &'a ast::Fn) -> Self {
        Self { fn_item }
    }

    fn params(&self) -> Vec<ParamInfo> {
        let mut params = Vec::new();
        let Some(param_list) = self.fn_item.param_list() else {
            return params;
        };
        for param in param_list.params() {
            let Some(pat) = param.pat() else { continue };
            let Some(ty) = param.ty() else { continue };

            let ast::Pat::IdentPat(ident) = &pat else {
                panic!("param is not an IdentPat");
            };

            let (is_refcount_pointer, is_unsafe_pointer) = pointer_flags(&ty);
            params.push(ParamInfo {
                name: ident
                    .name()
                    .map(|n| n.text().to_string())
                    .unwrap_or_default(),
                ty: ty.syntax().text().to_string(),
                is_refcount_pointer,
                is_unsafe_pointer,
                is_mut_ref: matches!(&ty, ast::Type::RefType(r) if r.mut_token().is_some()),
            });
        }
        params
    }

    fn return_type(&self) -> Option<TypeInfo> {
        let ty = self.fn_item.ret_type()?.ty()?;
        let ty_str = ty.syntax().text().to_string();
        if ty_str == "()" {
            None
        } else {
            let (is_refcount_pointer, is_unsafe_pointer) = pointer_flags(&ty);
            Some(TypeInfo {
                ty: ty_str,
                is_refcount_pointer,
                is_unsafe_pointer,
            })
        }
    }

    fn generics(&self) -> BTreeMap<String, Vec<String>> {
        let mut generics = BTreeMap::new();

        let extract_bounds = |tbl: Option<ast::TypeBoundList>| -> Vec<String> {
            tbl.into_iter()
                .flat_map(|tbl| tbl.bounds())
                .map(|b| b.syntax().text().to_string())
                .filter(|s| s != "'static")
                .collect()
        };

        if let Some(gpl) = self.fn_item.generic_param_list() {
            for param in gpl.generic_params() {
                if let ast::GenericParam::TypeParam(tp) = param {
                    let name = tp.name().map(|n| n.text().to_string()).unwrap_or_default();
                    generics.insert(name, extract_bounds(tp.type_bound_list()));
                }
            }
        }

        if let Some(wc) = self.fn_item.where_clause() {
            for pred in wc.predicates() {
                let Some(ty) = pred.ty() else { continue };
                let name = ty.syntax().text().to_string();
                let bounds = extract_bounds(pred.type_bound_list());
                generics
                    .entry(name)
                    .and_modify(|existing| existing.extend(bounds.clone()))
                    .or_insert(bounds);
            }
        }

        generics
    }

    fn classify_rw_from_name_ref(&self, name_ref: &ast::NameRef) -> Access {
        name_ref
            .syntax()
            .ancestors()
            .find_map(|node| {
                match_ast! {
                    match node {
                        ast::BinExpr(expr) => {
                            if matches!(expr.op_kind(), Some(ast::BinaryOp::Assignment { .. }))
                                && let Some(lhs) = expr.lhs()
                                && lhs.syntax().text_range().end()
                                    == name_ref.syntax().text_range().end()
                            {
                                return Some(Access::Write);
                            }
                            Some(Access::Read)
                        },
                        ast::RefExpr(ref_expr) => {
                            if ref_expr.mut_token().is_some() {
                                Some(Access::Write)
                            } else {
                                Some(Access::Read)
                            }
                        },
                        ast::MethodCallExpr(call) => {
                            if let Some(receiver) = call.receiver() &&
                                !receiver
                                    .syntax()
                                    .text_range()
                                    .contains_range(name_ref.syntax().text_range())
                                {
                                    return None;
                                }
                            Some(Access::Unknown)
                        },
                        ast::ReturnExpr(ret) => {
                            if self.returns_mut_ref()
                                && ret.expr().is_some_and(|e|
                                    e.syntax().text_range() == name_ref.syntax().text_range())
                            {
                                Some(Access::Write)
                            } else {
                                Some(Access::Read)
                            }
                        },
                        ast::StmtList(sl) => {
                            if self.returns_mut_ref()
                                && sl.tail_expr().is_some_and(|tail|
                                    tail.syntax().text_range() == name_ref.syntax().text_range())
                            {
                                Some(Access::Write)
                            } else {
                                Some(Access::Read)
                            }
                        },
                        _ => None,
                    }
                }
            })
            .unwrap_or(Access::Read)
    }

    fn returns_mut_ref(&self) -> bool {
        self.fn_item
            .ret_type()
            .and_then(|rt| rt.ty())
            .is_some_and(|ty| matches!(ty, ast::Type::RefType(r) if r.mut_token().is_some()))
    }

    fn classify_access(&self, token: &ra_ap_syntax::SyntaxToken) -> Access {
        let Some(name_ref) = token.parent().and_then(ast::NameRef::cast) else {
            // Inside a macro invocation. Phase 2 will resolve this.
            return Access::Unknown;
        };
        self.classify_rw_from_name_ref(&name_ref)
    }

    fn trim_whitespaces(fragments: &mut Vec<BodyFragment>) {
        if let Some(first) = fragments.first_mut().and_then(|f| f.as_text_mut()) {
            *first = first.trim_start().to_string();
            if first.is_empty() {
                fragments.remove(0);
            }
        }
        if let Some(last) = fragments.last_mut().and_then(|f| f.as_text_mut()) {
            *last = last.trim_end().to_string();
            if last.is_empty() {
                fragments.pop();
            }
        }
    }

    fn body_fragments(&self, params: &[ParamInfo], generic_names: &[String]) -> Vec<BodyFragment> {
        let Some(body) = self.fn_item.body() else {
            return Vec::new();
        };
        let stmt_list = body.stmt_list().unwrap();

        let mut ctx = FragmentCtx::new(self, params, generic_names);
        for child in stmt_list.syntax().children_with_tokens() {
            if let ra_ap_syntax::NodeOrToken::Token(ref t) = child
                && (t.kind() == SyntaxKind::L_CURLY || t.kind() == SyntaxKind::R_CURLY)
                && t.parent().as_ref() == Some(stmt_list.syntax())
            {
                continue;
            }
            ctx.visit(child);
        }
        ctx.finish()
    }

    fn build(&self, path: &Path) -> FnIr {
        let fn_name = self
            .fn_item
            .name()
            .map(|n| n.text().to_string())
            .unwrap_or_default();
        let params = self.params();

        let params_map: BTreeMap<String, TypeInfo> = params
            .iter()
            .map(|p| {
                (
                    p.name.clone(),
                    TypeInfo {
                        ty: p.ty.clone(),
                        is_refcount_pointer: p.is_refcount_pointer,
                        is_unsafe_pointer: p.is_unsafe_pointer,
                    },
                )
            })
            .collect();

        let generics = self.generics();
        let generic_names: Vec<String> = generics.keys().cloned().collect();

        let multi_statement = self.fn_item.body().and_then(|body| {
            body.stmt_list().and_then(|stmt_list| {
                let stmt_count = stmt_list.statements().count();
                if stmt_count > 1 || (stmt_count == 1 && stmt_list.tail_expr().is_some()) {
                    Some(true)
                } else {
                    None
                }
            })
        });

        let body = self.body_fragments(&params, &generic_names);

        let ir = FnIr {
            params: if params_map.is_empty() {
                None
            } else {
                Some(params_map)
            },
            return_type: self.return_type(),
            generics: if generics.is_empty() {
                None
            } else {
                Some(generics)
            },
            multi_statement,
            body,
        };
        ir.validate(&format!("{}:{}", path.display(), fn_name));
        ir
    }
}

struct TypeIrBuilder<'a> {
    fn_item: &'a ast::Fn,
}

impl<'a> TypeIrBuilder<'a> {
    fn new(fn_item: &'a ast::Fn) -> Self {
        Self { fn_item }
    }

    fn build(&self) -> Vec<(String, TypeIr)> {
        let Some(body) = self.fn_item.body() else {
            return Vec::new();
        };
        let mut results = Vec::new();
        for stmt in body.syntax().descendants().filter_map(ast::LetStmt::cast) {
            let Some(pat) = stmt.pat() else { continue };
            let pat_text = pat.syntax().text().to_string();
            if !pat_text.starts_with('t') {
                continue;
            }
            let Some(ty) = stmt.ty() else { continue };
            let Some(init) = stmt.initializer() else {
                continue;
            };

            let (is_refcount_pointer, is_unsafe_pointer) = pointer_flags(&ty);
            results.push((
                pat_text,
                TypeIr {
                    init: init.syntax().text().to_string(),
                    type_info: TypeInfo {
                        ty: ty.syntax().text().to_string(),
                        is_refcount_pointer,
                        is_unsafe_pointer,
                    },
                },
            ));
        }
        results
    }
}
