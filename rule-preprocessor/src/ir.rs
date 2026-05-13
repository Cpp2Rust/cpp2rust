// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};

fn validate_consecutive_keys<'a>(
    keys: impl Iterator<Item = &'a String>,
    prefix: char,
    start: usize,
    label: &str,
) {
    let mut indices: Vec<usize> = Vec::new();
    for key in keys {
        assert!(
            key.len() >= 2
                && key.starts_with(prefix)
                && key[1..].chars().all(|c| c.is_ascii_digit()),
            "{label}: invalid name '{key}', expected {prefix}{start}, {prefix}{}, {prefix}{}, ...",
            start + 1,
            start + 2
        );
        indices.push(key[1..].parse().unwrap());
    }
    indices.sort();
    for (i, &idx) in indices.iter().enumerate() {
        assert!(
            idx == i + start,
            "{label}: not consecutive. Got: {:?}",
            indices
                .iter()
                .map(|j| format!("{prefix}{j}"))
                .collect::<Vec<_>>()
        );
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeInfo {
    #[serde(rename = "type")]
    pub ty: String,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_refcount_pointer: bool,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_unsafe_pointer: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FnIr {
    pub body: Vec<BodyFragment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generics: Option<BTreeMap<String, Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_statement: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<BTreeMap<String, TypeInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_type: Option<TypeInfo>,
}

impl FnIr {
    /// Find the next unvisited placeholder for `param`, mark it visited,
    /// and if it was "unknown", patch it with the given access.
    /// Searches inside MethodCall bodies recursively.
    pub fn resolve_next_param(
        &mut self,
        param: &str,
        access: Access,
        visited: &mut HashMap<String, usize>,
    ) {
        let n = visited.entry(param.to_string()).or_insert(0);
        let nth = std::mem::replace(n, *n + 1);
        resolve_nth_unknown(&mut self.body, param, access, nth);
    }

    pub fn has_unknowns(&self) -> bool {
        fn check(body: &[BodyFragment]) -> bool {
            body.iter().any(|f| match f {
                BodyFragment::Placeholder { placeholder } => placeholder.access == Access::Unknown,
                BodyFragment::MethodCall { method_call } => {
                    check(&method_call.receiver) || check(&method_call.body)
                }
                _ => false,
            })
        }
        check(&self.body)
    }

    pub fn validate(&self, name: &str) {
        validate_consecutive_keys(
            self.params.as_ref().map(|p| p.keys()).into_iter().flatten(),
            'a',
            0,
            &format!("Rule {name} params"),
        );
        validate_consecutive_keys(
            self.generics
                .as_ref()
                .map(|g| g.keys())
                .into_iter()
                .flatten(),
            'T',
            1,
            &format!("Rule {name} generics"),
        );
        assert!(!self.body.is_empty(), "Rule {name}: body must not be empty");
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeIr {
    pub init: String,
    #[serde(flatten)]
    pub type_info: TypeInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BodyFragment {
    Text { text: String },
    Placeholder { placeholder: PlaceholderInner },
    Generic { generic: i32 },
    MethodCall { method_call: MethodCallInner },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Access {
    Read,
    Write,
    Move,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaceholderInner {
    pub arg: i32,
    pub access: Access,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodCallInner {
    pub receiver: Vec<BodyFragment>,
    pub body: Vec<BodyFragment>,
}

// For convenience: match on fragment kind
impl BodyFragment {
    pub fn as_text_mut(&mut self) -> Option<&mut String> {
        match self {
            BodyFragment::Text { text } => Some(text),
            _ => None,
        }
    }
}

/// Resolve the nth Unknown placeholder for `param` in a body fragment list.
fn resolve_nth_unknown(body: &mut [BodyFragment], param: &str, access: Access, nth: usize) {
    let mut count = 0;
    fn resolve(
        body: &mut [BodyFragment],
        param: &str,
        access: Access,
        nth: usize,
        count: &mut usize,
    ) -> bool {
        for frag in body {
            match frag {
                BodyFragment::Placeholder { placeholder }
                    if placeholder.arg == param[1..].parse().unwrap_or(0) =>
                {
                    if *count == nth {
                        if placeholder.access == Access::Unknown {
                            placeholder.access = access;
                        }
                        return true;
                    }
                    *count += 1;
                }
                BodyFragment::MethodCall { method_call } => {
                    if resolve(&mut method_call.receiver, param, access, nth, count) {
                        return true;
                    }
                    if resolve(&mut method_call.body, param, access, nth, count) {
                        return true;
                    }
                }
                _ => {}
            }
        }
        false
    }
    resolve(body, param, access, nth, &mut count);
}

// A rule file's IR: mix of function rules (f1, f2, ...) and type rules (t1, t2, ...)
// Both serialize to the same JSON object, but FnIr has "body" while TypeIr has "type"+"init".
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RuleIr {
    Fn(FnIr),
    Type(TypeIr),
}

/// Per-file IR: rule name -> FnIr or TypeIr
pub type FileIr = BTreeMap<String, RuleIr>;

/// All IR for all rule files.
pub struct RulesIR {
    pub all_ir: HashMap<String, FileIr>,
    pub has_unknowns: bool,
    pub crate_root: PathBuf,
}

impl RulesIR {
    pub fn write_ir(&self) {
        let crate_root = self.crate_root.canonicalize().unwrap();
        for (rule_path, file_ir) in &self.all_ir {
            let rule_path = Path::new(rule_path);
            let file_name = rule_path.file_name().unwrap().to_str().unwrap();
            let json_name = file_name.replace("tgt_", "ir_").replace(".rs", ".json");
            let json_path = rule_path.parent().unwrap().join(json_name);

            let json = serde_json::to_string_pretty(file_ir).unwrap();
            std::fs::write(&json_path, format!("{json}\n")).unwrap();

            let json_rel = json_path.strip_prefix(&crate_root).unwrap_or(&json_path);
            println!("{}", json_rel.display());
        }
    }
}
