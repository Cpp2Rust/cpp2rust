// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::ir::{Access, FnIr, Model, RuleIr, RulesIR};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct SemanticAnalysis;

impl SemanticAnalysis {
    pub fn run(ir: RulesIR, out_dir: &Path) -> RulesIR {
        let crate_root = write_crate_root(&ir, out_dir);
        let args = build_rustc_args(&crate_root);
        let mut resolver = MethodResolver { ir };

        if rustc_driver::catch_fatal_errors(|| {
            rustc_driver::compiler_entrypoint(&args, &mut resolver);
        })
        .is_err()
        {
            eprintln!("warning: rustc compilation had errors during semantic analysis");
        }

        resolver.assert_no_unknowns();
        resolver.ir
    }
}

fn write_crate_root(ir: &RulesIR, out_dir: &Path) -> PathBuf {
    let mut buf = String::from("#![allow(warnings)]\n");
    for (model, _) in ir {
        let filename = model.src_filename();
        let stem = filename.strip_suffix(".rs").unwrap();
        let source = ir.dir.join(filename);
        buf.push_str(&format!(
            "#[path = r#\"{}\"#]\npub mod rule_{stem};\n",
            source.display()
        ));
    }

    let path = out_dir.join("crate_root.rs");
    std::fs::write(&path, buf).unwrap();
    path
}

fn build_rustc_args(crate_root: &Path) -> Vec<String> {
    let sysroot = get_sysroot();

    let mut args = vec![
        "rustc".to_string(),
        crate_root.to_string_lossy().to_string(),
        "--crate-name".to_string(),
        "rules".to_string(),
        "--crate-type".to_string(),
        "lib".to_string(),
        "--edition".to_string(),
        "2024".to_string(),
        format!("--sysroot={}", sysroot.display()),
    ];

    let artifacts = read_cargo_artifacts();

    // Make transitive deps discoverable, as cargo does.
    let mut lib_dirs: Vec<&Path> = artifacts
        .values()
        .flatten()
        .filter_map(|lib| lib.parent())
        .collect();
    lib_dirs.sort();
    lib_dirs.dedup();
    for dir in lib_dirs {
        args.push("-L".to_string());
        args.push(format!("dependency={}", dir.display()));
    }

    for dep in &[
        "libcc2rs",
        "libcc2rs_macros",
        "libc",
        "nix",
        "jiff",
        "xattr",
    ] {
        let lib = match artifacts.get(*dep).map(Vec::as_slice) {
            Some([lib]) => lib,
            Some([]) | None => panic!("no artifact for crate `{dep}` in cargo's build output"),
            Some(libs) => panic!("multiple artifacts for crate `{dep}`: {libs:?}"),
        };
        args.push("--extern".to_string());
        args.push(format!("{}={}", dep, lib.display()));
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

/// Read the output of `cargo build --message-format=json` for this crate and
/// map each library crate name to the artifacts that build produced. The
/// target dir may also hold stale artifacts of the same crates (e.g., from an
/// older toolchain or from `cargo clippy`), so it cannot simply be scanned.
fn read_cargo_artifacts() -> HashMap<String, Vec<PathBuf>> {
    let path = std::env::var("RULE_PREPROCESSOR_ARTIFACTS")
        .expect("RULE_PREPROCESSOR_ARTIFACTS must be set");
    let contents =
        std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("failed to read {path}: {e}"));

    let mut artifacts: HashMap<String, Vec<PathBuf>> = HashMap::new();
    for line in contents.lines() {
        let Ok(msg) = serde_json::from_str::<serde_json::Value>(line) else {
            continue;
        };
        if msg["reason"] != "compiler-artifact" {
            continue;
        }
        let Some(name) = msg["target"]["name"].as_str() else {
            continue;
        };
        let Some(filenames) = msg["filenames"].as_array() else {
            continue;
        };
        let filenames: Vec<PathBuf> = filenames
            .iter()
            .filter_map(|f| f.as_str())
            .map(PathBuf::from)
            .collect();
        let find = |ext: &str| {
            filenames
                .iter()
                .find(|f| f.extension().is_some_and(|e| e == ext))
        };
        let Some(lib) = find("rmeta")
            .or_else(|| find("rlib"))
            .or_else(|| find(std::env::consts::DLL_EXTENSION))
        else {
            continue;
        };
        let libs = artifacts.entry(name.to_string()).or_default();
        if !libs.contains(lib) {
            libs.push(lib.clone());
        }
    }
    artifacts
}

struct FnDecl<'tcx> {
    source_file: String,
    name: String,
    def_id: rustc_span::def_id::DefId,
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
            .filter(|name| name != "init")
            .collect()
    }

    fn resolve_unknowns(&self, tcx: rustc_middle::ty::TyCtxt<'tcx>, fn_ir: &mut FnIr) {
        let mut visitor = AstVisitor {
            tcx,
            param_names: self.param_names(),
            fn_ir,
            visited: HashMap::new(),
        };
        if let rustc_hir::ExprKind::Block(block, _) = &self.body.value.kind
            && block.stmts.is_empty()
            && let Some(e) = block.expr
            && let Some(param) = visitor.expr_as_decl_ref(e)
        {
            visitor
                .fn_ir
                .resolve_next_param(&param, &mut visitor.visited, |p| {
                    if p.access == Access::Unknown {
                        p.access = Access::Borrow;
                    }
                });
            return;
        }
        visitor.visit_expr(self.body.value, Access::Borrow);
    }

    fn unsafe_uses(&self, tcx: rustc_middle::ty::TyCtxt<'tcx>) -> UnsafeUses {
        use rustc_hir::intravisit::Visitor;
        let mut finder = UnsafeFinder {
            tcx,
            typeck: tcx.typeck_body(self.body.id()),
            uses: UnsafeUses::default(),
        };
        finder.visit_expr(self.body.value);
        finder.uses
    }
}

/// What a rule body has that needs an unsafe context.
#[derive(Default)]
struct UnsafeUses {
    block: bool,
    op: bool,
}

struct UnsafeFinder<'tcx> {
    tcx: rustc_middle::ty::TyCtxt<'tcx>,
    typeck: &'tcx rustc_middle::ty::TypeckResults<'tcx>,
    uses: UnsafeUses,
}

impl<'tcx> UnsafeFinder<'tcx> {
    fn is_unsafe_op(&self, expr: &'tcx rustc_hir::Expr<'tcx>) -> bool {
        use rustc_hir::def::{DefKind, Res};
        use rustc_middle::ty::TyKind;

        match &expr.kind {
            rustc_hir::ExprKind::Call(callee, _) => {
                let ty = self.typeck.expr_ty_adjusted(callee);
                matches!(ty.kind(), TyKind::FnDef(..) | TyKind::FnPtr(..))
                    && ty.fn_sig(self.tcx).safety().is_unsafe()
            }
            rustc_hir::ExprKind::MethodCall(..) => self
                .typeck
                .type_dependent_def_id(expr.hir_id)
                .is_some_and(|def_id| self.tcx.fn_sig(def_id).skip_binder().safety().is_unsafe()),
            rustc_hir::ExprKind::Unary(rustc_hir::UnOp::Deref, inner) => {
                self.typeck.expr_ty_adjusted(inner).is_raw_ptr()
            }
            rustc_hir::ExprKind::Field(base, _) => self
                .typeck
                .expr_ty_adjusted(base)
                .ty_adt_def()
                .is_some_and(|adt| adt.is_union()),
            rustc_hir::ExprKind::Path(qpath) => match self.typeck.qpath_res(qpath, expr.hir_id) {
                Res::Def(DefKind::Static { mutability, .. }, def_id) => {
                    mutability.is_mut() || self.tcx.is_foreign_item(def_id)
                }
                _ => false,
            },
            rustc_hir::ExprKind::InlineAsm(_) => true,
            _ => false,
        }
    }
}

impl<'tcx> rustc_hir::intravisit::Visitor<'tcx> for UnsafeFinder<'tcx> {
    type NestedFilter = rustc_middle::hir::nested_filter::OnlyBodies;

    fn maybe_tcx(&mut self) -> rustc_middle::ty::TyCtxt<'tcx> {
        self.tcx
    }

    fn visit_block(&mut self, block: &'tcx rustc_hir::Block<'tcx>) {
        let rustc_hir::BlockCheckMode::UnsafeBlock(source) = block.rules else {
            rustc_hir::intravisit::walk_block(self, block);
            return;
        };
        // Like the `unsafe_code` lint, trust the unsafe blocks of the compiler
        // and of the macros that allow internal unsafe, e.g., `panic!`.
        if source == rustc_hir::UnsafeSource::UserProvided && !block.span.allows_unsafe() {
            // The unsafe block of a macro is an unsafe operation of the rule.
            if block.span.from_expansion() {
                self.uses.op = true;
            } else {
                self.uses.block = true;
            }
        }
    }

    fn visit_expr(&mut self, expr: &'tcx rustc_hir::Expr<'tcx>) {
        self.uses.op |= self.is_unsafe_op(expr);
        rustc_hir::intravisit::walk_expr(self, expr);
    }
}

struct MethodResolver {
    ir: RulesIR,
}

impl MethodResolver {
    fn resolve_rule<'tcx>(&mut self, tcx: rustc_middle::ty::TyCtxt<'tcx>, f: &FnDecl<'tcx>) {
        // The refcount model falls back to the unsafe rules it does not have.
        let used_by_refcount = f.source_file.ends_with(Model::Refcount.src_filename())
            || !self
                .ir
                .refcount_ir
                .as_ref()
                .is_some_and(|ir| ir.contains_key(&f.name));
        let Some(file_ir) = self.ir.get_mut(&f.source_file) else {
            return;
        };
        match file_ir.get_mut(&f.name) {
            Some(RuleIr::Fn(fn_ir)) => f.resolve_unknowns(tcx, fn_ir),
            Some(RuleIr::Type(type_ir)) => {
                let ret_ty = tcx.fn_sig(f.def_id).skip_binder().output().skip_binder();
                type_ir.type_info.derives = type_derives(tcx, tcx.param_env(f.def_id), ret_ty);
            }
            None => return,
        }

        // A rule is either unsafe as a whole or does nothing unsafe.
        let uses = f.unsafe_uses(tcx);
        assert!(
            !uses.block,
            "{} ({}): rules must not have unsafe blocks; mark the rule unsafe instead",
            f.name, f.source_file
        );
        if tcx.fn_sig(f.def_id).skip_binder().safety().is_unsafe() {
            assert!(
                !used_by_refcount,
                "{} ({}): the refcount model uses this rule, so it must not be unsafe",
                f.name, f.source_file
            );
            assert!(
                uses.op,
                "{} ({}): the rule does no unsafe operation, so it must not be unsafe",
                f.name, f.source_file
            );
        } else {
            assert!(
                !uses.op,
                "{} ({}): the rule does an unsafe operation, so it must be unsafe",
                f.name, f.source_file
            );
        }
    }

    fn assert_no_unknowns(&self) {
        for (model, file_ir) in &self.ir {
            for (rule_name, rule) in file_ir {
                let RuleIr::Fn(fn_ir) = rule else { continue };
                assert!(
                    !fn_ir.has_unknowns(),
                    "unresolved access=\"unknown\" in {} ({})",
                    rule_name,
                    self.ir.dir.join(model.src_filename()).display()
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
            self.resolve_rule(tcx, &f);
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
            def_id: decl_id.owner_id.to_def_id(),
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

fn is_copy<'tcx>(
    tcx: rustc_middle::ty::TyCtxt<'tcx>,
    param_env: rustc_middle::ty::ParamEnv<'tcx>,
    ty: rustc_middle::ty::Ty<'tcx>,
) -> bool {
    use rustc_infer::infer::TyCtxtInferExt;
    use rustc_trait_selection::infer::InferCtxtExt;

    let Some(copy_trait) = tcx.lang_items().copy_trait() else {
        return false;
    };
    let infcx = tcx
        .infer_ctxt()
        .build(rustc_middle::ty::TypingMode::non_body_analysis());
    infcx
        .type_implements_trait(copy_trait, [ty], param_env)
        .must_apply_modulo_regions()
}

fn type_derives<'tcx>(
    tcx: rustc_middle::ty::TyCtxt<'tcx>,
    param_env: rustc_middle::ty::ParamEnv<'tcx>,
    ty: rustc_middle::ty::Ty<'tcx>,
) -> Vec<String> {
    use rustc_infer::infer::TyCtxtInferExt;
    use rustc_span::sym;
    use rustc_trait_selection::infer::InferCtxtExt;

    let lang = tcx.lang_items();
    let derivable = [
        lang.copy_trait(),
        lang.clone_trait(),
        tcx.get_diagnostic_item(sym::Debug),
        tcx.get_diagnostic_item(sym::Default),
        tcx.get_diagnostic_item(sym::PartialEq),
        tcx.get_diagnostic_item(sym::Eq),
        tcx.get_diagnostic_item(sym::PartialOrd),
        tcx.get_diagnostic_item(sym::Ord),
        tcx.get_diagnostic_item(sym::Hash),
    ];

    let infcx = tcx
        .infer_ctxt()
        .build(rustc_middle::ty::TypingMode::non_body_analysis());

    derivable
        .into_iter()
        .flatten()
        .filter(|&trait_def_id| {
            let args = vec![ty; tcx.generics_of(trait_def_id).count()];
            infcx
                .type_implements_trait(trait_def_id, args, param_env)
                .must_apply_modulo_regions()
        })
        .map(|trait_def_id| tcx.item_name(trait_def_id).to_string())
        .collect()
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
                    p.is_index_base = true;
                });
            return;
        }
        self.visit_expr(expr, context);
    }

    fn visit_expr(&mut self, expr: &'tcx rustc_hir::Expr<'tcx>, context: Access) {
        // Reached an argument used inside the rule body
        if let Some(param) = self.expr_as_decl_ref(expr) {
            let access = if context == Access::Borrow && self.is_moved(expr) {
                Access::Move
            } else {
                context
            };
            self.fn_ir
                .resolve_next_param(&param, &mut self.visited, |p| {
                    if p.access == Access::Unknown {
                        p.access = access;
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
                    let access = param_access.get(i + 1).copied().unwrap_or(Access::Borrow);
                    self.visit_expr(arg, access);
                }
            }
            rustc_hir::ExprKind::Call(callee, args) => {
                if self.is_std_mem_take(expr) && args.len() == 1 {
                    self.visit_expr(&args[0], Access::Take);
                } else {
                    self.visit_expr(callee, context);
                    let param_access = self.resolve_callee_param_access(expr);
                    for (i, arg) in args.iter().enumerate() {
                        let access = param_access.get(i).copied().unwrap_or(Access::Borrow);
                        self.visit_expr(arg, access);
                    }
                }
            }

            rustc_hir::ExprKind::Assign(lhs, rhs, _)
            | rustc_hir::ExprKind::AssignOp(_, lhs, rhs) => {
                self.visit_expr(lhs, Access::BorrowMut);
                self.visit_expr(rhs, Access::Borrow);
            }

            rustc_hir::ExprKind::AddrOf(_, rustc_hir::Mutability::Mut, inner) => {
                self.visit_expr(
                    inner,
                    if context == Access::Take {
                        Access::Take
                    } else {
                        Access::BorrowMut
                    },
                );
            }
            rustc_hir::ExprKind::AddrOf(_, rustc_hir::Mutability::Not, inner) => {
                if let Some(param) = self.expr_as_decl_ref(inner) {
                    self.fn_ir
                        .resolve_next_param(&param, &mut self.visited, |p| {
                            if p.access == Access::Unknown {
                                p.access = Access::Borrow;
                            }
                        });
                    return;
                }
                self.visit_expr(inner, Access::Borrow);
            }

            rustc_hir::ExprKind::Block(block, _) | rustc_hir::ExprKind::Loop(block, _, _, _) => {
                for stmt in block.stmts {
                    match &stmt.kind {
                        rustc_hir::StmtKind::Let(local) => {
                            if let Some(init) = local.init {
                                self.visit_expr(init, Access::Borrow);
                            }
                        }
                        rustc_hir::StmtKind::Expr(e) | rustc_hir::StmtKind::Semi(e) => {
                            self.visit_expr(e, Access::Borrow);
                        }
                        _ => {}
                    }
                }
                if let Some(e) = block.expr {
                    self.visit_expr(e, context);
                }
            }

            rustc_hir::ExprKind::If(cond, then_branch, else_branch) => {
                self.visit_expr(cond, Access::Borrow);
                self.visit_expr(then_branch, context);
                if let Some(e) = else_branch {
                    self.visit_expr(e, context);
                }
            }
            rustc_hir::ExprKind::Match(scrutinee, arms, _) => {
                self.visit_expr(scrutinee, Access::Borrow);
                for arm in arms.iter() {
                    if let Some(guard) = arm.guard {
                        self.visit_expr(guard, Access::Borrow);
                    }
                    self.visit_expr(arm.body, context);
                }
            }

            rustc_hir::ExprKind::Field(base, _) => {
                if let Some(param) = self.expr_as_decl_ref(base) {
                    self.fn_ir
                        .resolve_next_param(&param, &mut self.visited, |p| {
                            if p.access == Access::Unknown {
                                p.access = context;
                            }
                        });
                    return;
                }
                self.visit_expr(base, context);
            }
            rustc_hir::ExprKind::Unary(_, e)
            | rustc_hir::ExprKind::Cast(e, _)
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
            | rustc_hir::ExprKind::Break(_, None)
            | rustc_hir::ExprKind::Continue(_) => {}

            rustc_hir::ExprKind::Ret(Some(e)) | rustc_hir::ExprKind::Break(_, Some(e)) => {
                self.visit_expr(e, Access::Borrow);
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

    fn is_moved(&self, expr: &rustc_hir::Expr<'tcx>) -> bool {
        let results = self.tcx.typeck(expr.hir_id.owner);
        let borrowed = results
            .expr_adjustments(expr)
            .iter()
            .any(|adj| matches!(adj.kind, rustc_middle::ty::adjustment::Adjust::Borrow(_)));
        let param_env = self.tcx.param_env(expr.hir_id.owner.to_def_id());
        !borrowed && !is_copy(self.tcx, param_env, results.expr_ty(expr))
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
            rustc_middle::ty::TyKind::Ref(_, _, rustc_middle::ty::Mutability::Mut) => {
                Access::BorrowMut
            }
            rustc_middle::ty::TyKind::Ref(_, _, rustc_middle::ty::Mutability::Not) => {
                Access::Borrow
            }
            rustc_middle::ty::TyKind::RawPtr(_, rustc_middle::ty::Mutability::Mut) => {
                Access::BorrowMut
            }
            rustc_middle::ty::TyKind::RawPtr(_, rustc_middle::ty::Mutability::Not) => {
                Access::Borrow
            }
            _ => Access::Borrow,
        }
    }
}
