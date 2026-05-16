// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::ir::RulesIR;
use crate::ir::{Access, FnIr, RuleIr};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct SemanticAnalysis;

impl SemanticAnalysis {
    pub fn run(ir: RulesIR) -> RulesIR {
        let args = build_rustc_args(&ir.crate_root);
        let mut resolver = MethodResolver { ir };

        if rustc_driver::catch_fatal_errors(|| {
            rustc_driver::run_compiler(&args, &mut resolver);
        })
        .is_err()
        {
            eprintln!("warning: rustc compilation had errors during semantic analysis");
        }

        resolver.assert_no_unknowns();
        resolver.ir
    }
}

fn build_rustc_args(crate_root: &Path) -> Vec<String> {
    let sysroot = get_sysroot();
    let lib_path = crate_root.join("src").join("lib.rs");
    let deps = find_deps_dir();

    let mut args = vec![
        "rustc".to_string(),
        lib_path.to_string_lossy().to_string(),
        "--crate-name".to_string(),
        "rules".to_string(),
        "--crate-type".to_string(),
        "lib".to_string(),
        "--edition".to_string(),
        "2024".to_string(),
        format!("--sysroot={}", sysroot.display()),
    ];

    args.push("-L".to_string());
    args.push(format!("dependency={}", deps.display()));

    for dep in &["libcc2rs", "libc", "brotli_sys"] {
        if let Some(rlib) = find_rlib(deps.as_path(), dep) {
            args.push("--extern".to_string());
            args.push(format!("{}={}", dep, rlib.display()));
        }
    }

    args
}

fn get_sysroot() -> PathBuf {
    let output = std::process::Command::new("rustc")
        .arg("--print=sysroot")
        .output()
        .expect("failed to run rustc --print=sysroot");
    PathBuf::from(String::from_utf8(output.stdout).unwrap().trim())
}

fn find_deps_dir() -> PathBuf {
    let target_dir = std::env::var("CARGO_TARGET_DIR").expect("CARGO_TARGET_DIR must be set");
    let profile = if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };
    PathBuf::from(target_dir).join(profile).join("deps")
}

fn find_rlib(deps_dir: &Path, crate_name: &str) -> Option<PathBuf> {
    let prefix = format!("lib{}-", crate_name);
    if let Ok(entries) = std::fs::read_dir(deps_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with(&prefix) && name.ends_with(".rlib") {
                return Some(entry.path());
            }
        }
    }
    None
}

struct FnDecl<'tcx> {
    source_file: String,
    name: String,
    body: &'tcx rustc_hir::Body<'tcx>,
}

impl<'tcx> FnDecl<'tcx> {
    fn param_names(&self) -> Vec<String> {
        self.body
            .params
            .iter()
            .map(|p| {
                if let rustc_hir::PatKind::Binding(_, _, ident, _) = p.pat.kind {
                    ident.name.as_str().to_string()
                } else {
                    panic!("Argument is not a simple binding");
                }
            })
            .collect()
    }

    fn resolve_unknowns(&self, tcx: rustc_middle::ty::TyCtxt<'tcx>, fn_ir: &mut FnIr) {
        let mut visitor = AstVisitor {
            tcx,
            param_names: self.param_names(),
            fn_ir,
            visited: HashMap::new(),
        };
        visitor.visit_expr(self.body.value, Access::Read);
    }
}

struct MethodResolver {
    ir: RulesIR,
}

impl MethodResolver {
    fn resolve_fn_decl<'tcx>(&mut self, tcx: rustc_middle::ty::TyCtxt<'tcx>, f: &FnDecl<'tcx>) {
        if let Some(file_ir) = self.ir.all_ir.get_mut(&f.source_file)
            && let Some(RuleIr::Fn(fn_ir)) = file_ir.get_mut(&f.name)
        {
            f.resolve_unknowns(tcx, fn_ir);
        }
    }

    fn assert_no_unknowns(&self) {
        for (source_file, file_ir) in &self.ir.all_ir {
            for (rule_name, rule) in file_ir {
                let RuleIr::Fn(fn_ir) = rule else { continue };
                assert!(
                    !fn_ir.has_unknowns(),
                    "unresolved access=\"unknown\" in {} ({})",
                    rule_name,
                    source_file
                );
            }
        }
    }
}

impl rustc_driver::Callbacks for MethodResolver {
    fn after_analysis(
        &mut self,
        _compiler: &rustc_interface::interface::Compiler,
        tcx: rustc_middle::ty::TyCtxt<'_>,
    ) -> rustc_driver::Compilation {
        for f in iter_fn_decls(tcx) {
            self.resolve_fn_decl(tcx, &f);
        }

        rustc_driver::Compilation::Stop
    }
}

fn iter_fn_decls<'tcx>(tcx: rustc_middle::ty::TyCtxt<'tcx>) -> Vec<FnDecl<'tcx>> {
    let mut result = Vec::new();
    for decl_id in tcx.hir_crate_items(()).free_items() {
        let rustc_hir::OwnerNode::Item(decl) = tcx.hir_owner_node(decl_id.owner_id) else {
            continue;
        };
        let rustc_hir::ItemKind::Fn {
            ident,
            body: body_id,
            ..
        } = decl.kind
        else {
            continue;
        };
        let Some(source_file) = decl_source_file(tcx, decl) else {
            panic!("No source file associated with decl");
        };
        result.push(FnDecl {
            source_file,
            name: ident.name.as_str().to_string(),
            body: tcx.hir_body(body_id),
        });
    }
    result
}

fn decl_source_file(
    tcx: rustc_middle::ty::TyCtxt<'_>,
    decl: &rustc_hir::Item<'_>,
) -> Option<String> {
    let filename = tcx.sess.source_map().span_to_filename(decl.span);
    let file_path = match &filename {
        rustc_span::FileName::Real(real) => real.local_path().map(|p| p.to_path_buf()),
        _ => None,
    }?;
    Some(
        file_path
            .canonicalize()
            .unwrap_or(file_path)
            .to_string_lossy()
            .to_string(),
    )
}

struct AstVisitor<'a, 'tcx> {
    tcx: rustc_middle::ty::TyCtxt<'tcx>,
    param_names: Vec<String>,
    fn_ir: &'a mut FnIr,
    visited: HashMap<String, usize>,
}

impl<'a, 'tcx> AstVisitor<'a, 'tcx> {
    fn visit_expr_as_index_base(&mut self, expr: &'tcx rustc_hir::Expr<'tcx>, context: Access) {
        if let Some(param) = self.expr_as_decl_ref(expr) {
            self.fn_ir
                .resolve_next_param(&param, &mut self.visited, |p| {
                    if p.access == Access::Unknown {
                        p.access = context;
                    }
                    p.needs_autoref = true;
                });
            return;
        }
        self.visit_expr(expr, context);
    }

    fn visit_expr(&mut self, expr: &'tcx rustc_hir::Expr<'tcx>, context: Access) {
        // Reached an argument used inside the rule body
        if let Some(param) = self.expr_as_decl_ref(expr) {
            self.fn_ir
                .resolve_next_param(&param, &mut self.visited, |p| {
                    if p.access == Access::Unknown {
                        p.access = context;
                    }
                });
            return;
        }

        match &expr.kind {
            rustc_hir::ExprKind::MethodCall(_seg, receiver, args, _span) => {
                let param_access = self.resolve_callee_param_access(expr);
                self.visit_expr(
                    receiver,
                    param_access.first().copied().unwrap_or(Access::Unknown),
                );
                for (i, arg) in args.iter().enumerate() {
                    let access = param_access.get(i + 1).copied().unwrap_or(Access::Read);
                    self.visit_expr(arg, access);
                }
            }
            rustc_hir::ExprKind::Call(callee, args) => {
                if self.is_std_mem_take(expr) && args.len() == 1 {
                    self.visit_expr(&args[0], Access::Move);
                } else {
                    self.visit_expr(callee, context);
                    let param_access = self.resolve_callee_param_access(expr);
                    for (i, arg) in args.iter().enumerate() {
                        let access = param_access.get(i).copied().unwrap_or(Access::Read);
                        self.visit_expr(arg, access);
                    }
                }
            }

            rustc_hir::ExprKind::Assign(lhs, rhs, _)
            | rustc_hir::ExprKind::AssignOp(_, lhs, rhs) => {
                self.visit_expr(lhs, Access::Write);
                self.visit_expr(rhs, Access::Read);
            }

            rustc_hir::ExprKind::AddrOf(_, rustc_hir::Mutability::Mut, inner) => {
                self.visit_expr(
                    inner,
                    if context == Access::Move {
                        Access::Move
                    } else {
                        Access::Write
                    },
                );
            }
            rustc_hir::ExprKind::AddrOf(_, rustc_hir::Mutability::Not, inner) => {
                self.visit_expr(inner, Access::Read);
            }

            rustc_hir::ExprKind::Block(block, _) | rustc_hir::ExprKind::Loop(block, _, _, _) => {
                for stmt in block.stmts {
                    match &stmt.kind {
                        rustc_hir::StmtKind::Let(local) => {
                            if let Some(init) = local.init {
                                self.visit_expr(init, Access::Read);
                            }
                        }
                        rustc_hir::StmtKind::Expr(e) | rustc_hir::StmtKind::Semi(e) => {
                            self.visit_expr(e, Access::Read);
                        }
                        _ => {}
                    }
                }
                if let Some(e) = block.expr {
                    self.visit_expr(e, context);
                }
            }

            rustc_hir::ExprKind::If(cond, then_branch, else_branch) => {
                self.visit_expr(cond, Access::Read);
                self.visit_expr(then_branch, context);
                if let Some(e) = else_branch {
                    self.visit_expr(e, context);
                }
            }
            rustc_hir::ExprKind::Match(scrutinee, arms, _) => {
                self.visit_expr(scrutinee, Access::Read);
                for arm in arms.iter() {
                    self.visit_expr(arm.body, context);
                }
            }

            rustc_hir::ExprKind::Unary(_, e)
            | rustc_hir::ExprKind::Cast(e, _)
            | rustc_hir::ExprKind::Field(e, _)
            | rustc_hir::ExprKind::DropTemps(e)
            | rustc_hir::ExprKind::Repeat(e, _) => {
                self.visit_expr(e, context);
            }
            rustc_hir::ExprKind::Index(base, idx, _) => {
                self.visit_expr_as_index_base(base, context);
                self.visit_expr(idx, context);
            }
            rustc_hir::ExprKind::Binary(_, a, b) => {
                self.visit_expr(a, context);
                self.visit_expr(b, context);
            }
            rustc_hir::ExprKind::Tup(exprs) | rustc_hir::ExprKind::Array(exprs) => {
                for e in exprs.iter() {
                    self.visit_expr(e, context);
                }
            }
            rustc_hir::ExprKind::Closure(closure) => {
                self.visit_expr(self.tcx.hir_body(closure.body).value, context);
            }

            rustc_hir::ExprKind::Struct(_, fields, tail) => {
                for f in fields.iter() {
                    self.visit_expr(f.expr, context);
                }
                if let rustc_hir::StructTailExpr::Base(b) = tail {
                    self.visit_expr(b, context);
                }
            }

            rustc_hir::ExprKind::Lit(_)
            | rustc_hir::ExprKind::Path(_)
            | rustc_hir::ExprKind::Ret(None)
            | rustc_hir::ExprKind::Break(_, _)
            | rustc_hir::ExprKind::Continue(_) => {}

            rustc_hir::ExprKind::Ret(Some(e)) => {
                self.visit_expr(e, Access::Read);
            }

            other => {
                let span_str = self
                    .tcx
                    .sess
                    .source_map()
                    .span_to_diagnostic_string(expr.span);
                panic!("visit_expr: unhandled {other:?} at {span_str}");
            }
        }
    }

    fn expr_as_decl_ref(&self, expr: &rustc_hir::Expr<'_>) -> Option<String> {
        if let rustc_hir::ExprKind::Path(rustc_hir::QPath::Resolved(_, path)) = &expr.kind
            && let Some(seg) = path.segments.last()
        {
            let name = seg.ident.name.as_str().to_string();
            if self.param_names.contains(&name) {
                return Some(name);
            }
        }
        None
    }

    fn resolve_callee_param_access(&self, call_expr: &rustc_hir::Expr<'tcx>) -> Vec<Access> {
        let type_check_results = self.tcx.typeck(call_expr.hir_id.owner);

        let param_types: Option<Vec<rustc_middle::ty::Ty<'tcx>>> =
            if let Some(def_id) = type_check_results.type_dependent_def_id(call_expr.hir_id) {
                let sig = self.tcx.fn_sig(def_id).skip_binder();
                Some(sig.inputs().skip_binder().to_vec())
            } else if let rustc_hir::ExprKind::Call(callee, _) = &call_expr.kind {
                let callee_ty = type_check_results.expr_ty(callee);
                match callee_ty.kind() {
                    rustc_middle::ty::TyKind::FnDef(def_id, _) => {
                        let sig = self.tcx.fn_sig(*def_id).skip_binder();
                        Some(sig.inputs().skip_binder().to_vec())
                    }
                    rustc_middle::ty::TyKind::FnPtr(sig_tys, _) => {
                        let sig = sig_tys.skip_binder();
                        Some(sig.inputs().to_vec())
                    }
                    _ => panic!("Unhandled callee type"),
                }
            } else {
                panic!("Unhandled call expression");
            };

        match param_types {
            Some(types) => types.iter().map(|ty| Self::access_for_type(ty)).collect(),
            None => Vec::new(),
        }
    }

    fn is_std_mem_take(&self, expr: &rustc_hir::Expr<'tcx>) -> bool {
        if let rustc_hir::ExprKind::Call(callee, _) = &expr.kind
            && let rustc_middle::ty::TyKind::FnDef(def_id, _) =
                self.tcx.typeck(expr.hir_id.owner).expr_ty(callee).kind()
        {
            return self.tcx.def_path_str(*def_id) == "std::mem::take";
        }
        false
    }

    fn access_for_type(ty: &rustc_middle::ty::Ty<'_>) -> Access {
        match ty.kind() {
            rustc_middle::ty::TyKind::Ref(_, _, rustc_middle::ty::Mutability::Mut) => Access::Write,
            rustc_middle::ty::TyKind::Ref(_, _, rustc_middle::ty::Mutability::Not) => Access::Read,
            rustc_middle::ty::TyKind::RawPtr(_, rustc_middle::ty::Mutability::Mut) => Access::Write,
            rustc_middle::ty::TyKind::RawPtr(_, rustc_middle::ty::Mutability::Not) => Access::Read,
            _ => Access::Read,
        }
    }
}
