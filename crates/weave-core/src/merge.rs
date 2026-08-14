
use std::collections::{BTreeSet, HashMap, HashSet};
use std::sync::LazyLock;

use sem_core::model::entity::SemanticEntity;
use sem_core::parser::plugins::create_default_registry;
use sem_core::parser::registry::ParserRegistry;
use serde::Serialize;

pub(crate) static PARSER_REGISTRY: LazyLock<ParserRegistry> =
    LazyLock::new(create_default_registry);

pub const DECLINED_EXTENSIONS: &[&str] = &[
    ".hs",
    ".vue",
    ".erb",
    ".svelte",
    ".svelte.js",
    ".svelte.ts",
    ".svelte.test.js",
    ".svelte.test.ts",
    ".svelte.spec.js",
    ".svelte.spec.ts",
];

pub fn supported_merge_extensions() -> Vec<String> { panic!("STUB: not implemented") }

use crate::conflict::{classify_conflict, ConflictKind, EntityConflict, MarkerFormat, MergeStats};
use crate::host::{Host, LineMergeStyle};
use crate::region::FileRegion;
use crate::validate::SemanticWarning;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ResolutionStrategy {
    Unchanged,
    OursOnly,
    TheirsOnly,
    ContentEqual,
    DiffyMerged,
    DecoratorMerged,
    InnerMerged,
    
    StatementMerged,
    
    FootprintLicensed,
    ConflictBothModified,
    
    ConflictStatementScoped,
    ConflictModifyDelete,
    ConflictBothAdded,
    ConflictRenameRename,
    ConflictRenameModify,
    AddedOurs,
    AddedTheirs,
    Deleted,
    Renamed {
        from: String,
        to: String,
    },
}

impl ResolutionStrategy {
    
    pub fn guard(&self) -> Option<&'static str> { panic!("STUB: not implemented") }

    pub(crate) fn guard_or_ladder(&self) -> &'static str { panic!("STUB: not implemented") }
}

#[derive(Debug, Clone, Serialize)]
pub struct EntityAudit {
    pub name: String,
    #[serde(rename = "type")]
    pub entity_type: String,
    pub resolution: ResolutionStrategy,
}

#[derive(Debug)]
pub struct MergeResult {
    pub content: String,
    pub conflicts: Vec<EntityConflict>,
    pub warnings: Vec<SemanticWarning>,
    pub stats: MergeStats,
    pub audit: Vec<EntityAudit>,
}

impl MergeResult {
    
    pub fn is_clean(&self) -> bool { panic!("STUB: not implemented") }
}

pub fn entity_merge(base: &str, ours: &str, theirs: &str, file_path: &str) -> MergeResult { panic!("STUB: not implemented") }

pub fn entity_merge_fmt(
    base: &str,
    ours: &str,
    theirs: &str,
    file_path: &str,
    marker_format: &MarkerFormat,
    host: &Host,
) -> MergeResult { panic!("STUB: not implemented") }

pub fn entity_merge_with_registry(
    base: &str,
    ours: &str,
    theirs: &str,
    file_path: &str,
    registry: &ParserRegistry,
    marker_format: &MarkerFormat,
    host: &Host,
) -> MergeResult { panic!("STUB: not implemented") }

pub(crate) fn is_whitespace_only_diff(a: &str, b: &str) -> bool { panic!("STUB: not implemented") }

fn widest_gap(base: usize, ours: usize, theirs: usize) -> usize { panic!("STUB: not implemented") }

fn is_decorator_line(line: &str) -> bool { panic!("STUB: not implemented") }

fn split_decorators(content: &str) -> (Vec<&str>, &str) { panic!("STUB: not implemented") }

pub(crate) fn try_decorator_aware_merge(
    base: &str,
    ours: &str,
    theirs: &str,
    decorators_compose: bool,
) -> Option<String> { panic!("STUB: not implemented") }

pub(crate) fn diffy_merge(base: &str, ours: &str, theirs: &str) -> Option<String> { panic!("STUB: not implemented") }

pub(crate) fn merge_interstitials(
    base_regions: &[FileRegion],
    ours_regions: &[FileRegion],
    theirs_regions: &[FileRegion],
    marker_format: &MarkerFormat,
) -> (HashMap<String, String>, Vec<EntityConflict>) { panic!("STUB: not implemented") }

fn is_import_region(content: &str) -> bool { panic!("STUB: not implemented") }

fn is_import_line(line: &str) -> bool { panic!("STUB: not implemented") }

#[derive(Debug, Clone)]
struct ImportStatement {
    
    lines: Vec<String>,
    
    source: String,
    
    specifiers: Vec<String>,
    
    is_multiline: bool,
}

fn parse_single_line_specifiers(trimmed: &str) -> Vec<String> { panic!("STUB: not implemented") }

fn parse_import_statements(content: &str) -> (Vec<ImportStatement>, Vec<String>) { panic!("STUB: not implemented") }

fn order_preserving_import_union<'a>(ours: &[&'a str], theirs: &[&'a str]) -> (Vec<&'a str>, bool) { panic!("STUB: not implemented") }

fn merge_imports_commutatively(base: &str, ours: &str, theirs: &str) -> (String, bool) { panic!("STUB: not implemented") }

fn merge_imports_with_multiline(
    _base_raw: &str,
    ours_raw: &str,
    _theirs_raw: &str,
    base_imports: &[ImportStatement],
    ours_imports: &[ImportStatement],
    theirs_imports: &[ImportStatement],
) -> String { panic!("STUB: not implemented") }

fn import_source_prefix(line: &str) -> &str { panic!("STUB: not implemented") }

fn import_prefix_affinity(a: &str, b: &str) -> usize { panic!("STUB: not implemented") }

pub(crate) fn line_level_fallback(
    base: &str,
    ours: &str,
    theirs: &str,
    file_path: &str,
    host: &Host,
) -> MergeResult { panic!("STUB: not implemented") }

pub(crate) fn line_merge_file(
    base: &str,
    ours: &str,
    theirs: &str,
    mut stats: MergeStats,
    host: &Host,
) -> MergeResult { panic!("STUB: not implemented") }

fn diffy_fallback(base: &str, ours: &str, theirs: &str, mut stats: MergeStats) -> MergeResult { panic!("STUB: not implemented") }

pub(crate) fn has_excessive_duplicates(entities: &[SemanticEntity], threshold: usize) -> bool { panic!("STUB: not implemented") }

pub(crate) fn filter_nested_entities(mut entities: Vec<SemanticEntity>) -> Vec<SemanticEntity> { panic!("STUB: not implemented") }

pub(crate) fn get_child_entities<'a>(
    parent: &SemanticEntity,
    all_entities: &'a [SemanticEntity],
) -> Vec<&'a SemanticEntity> { panic!("STUB: not implemented") }

pub(crate) fn is_container_entity_type(entity_type: &str) -> bool { panic!("STUB: not implemented") }

#[derive(Clone, Copy)]
pub(crate) struct ScopeMarkers<'a> {
    pub fmt: &'a MarkerFormat,
    
    pub entity_type: &'a str,
    pub entity_name: &'a str,
    
    pub guard: &'static str,
}

impl<'a> ScopeMarkers<'a> {
    
    #[cfg(test)]
    pub(crate) fn bare(fmt: &'a MarkerFormat) -> Self {
        Self {
            fmt,
            entity_type: "",
            entity_name: "",
            guard: "statement_fold",
        }
    }

    pub(crate) fn inside(
        fmt: &'a MarkerFormat,
        entity_type: &'a str,
        entity_name: &'a str,
        guard: &'static str,
    ) -> Self { panic!("STUB: not implemented") }

    fn context(&self) -> String { panic!("STUB: not implemented") }
}

pub(crate) fn scoped_conflict_marker(
    name: &str,
    base: Option<&str>,
    ours: Option<&str>,
    theirs: Option<&str>,
    ours_deleted: bool,
    theirs_deleted: bool,
    scope: &ScopeMarkers<'_>,
) -> String { panic!("STUB: not implemented") }

fn is_python_style_container(lines: &[&str]) -> bool { panic!("STUB: not implemented") }

pub(crate) fn is_container_close_line(trimmed: &str) -> bool { panic!("STUB: not implemented") }

pub(crate) fn extract_container_wrapper(content: &str) -> Option<(&str, &str)> { panic!("STUB: not implemented") }

pub(crate) fn extract_member_name(line: &str) -> String { panic!("STUB: not implemented") }

pub(crate) fn derive_name_from_struct_literal(content: &str) -> Option<String> { panic!("STUB: not implemented") }

pub fn is_binary(content: &str) -> bool { panic!("STUB: not implemented") }

pub(crate) fn has_conflict_markers(content: &str) -> bool { panic!("STUB: not implemented") }

fn skip_expansion(file_path: &str) -> bool { panic!("STUB: not implemented") }

const EXPANSION_MARK: u8 = 0x01;

fn expansion_safe(base: &str, ours: &str, theirs: &str) -> bool { panic!("STUB: not implemented") }

fn expand_separators(content: &str) -> String { panic!("STUB: not implemented") }

fn collapse_separators(merged: &str) -> String { panic!("STUB: not implemented") }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binding::replace_at_word_boundaries;

    #[test]
    fn test_replace_at_word_boundaries() {
        
        assert_eq!(
            replace_at_word_boundaries("fn get() {}", "get", "__E__"),
            "fn __E__() {}"
        );
        
        assert_eq!(
            replace_at_word_boundaries("fn getAll() {}", "get", "__E__"),
            "fn getAll() {}"
        );
        assert_eq!(
            replace_at_word_boundaries("fn _get() {}", "get", "__E__"),
            "fn _get() {}"
        );
        
        assert_eq!(
            replace_at_word_boundaries("pub enum Source { Source }", "Source", "__E__"),
            "pub enum __E__ { __E__ }"
        );
        
        assert_eq!(
            replace_at_word_boundaries("SourceManager isSource", "Source", "__E__"),
            "SourceManager isSource"
        );
        
        assert_eq!(
            replace_at_word_boundaries("❌ get ✅", "get", "__E__"),
            "❌ __E__ ✅"
        );
        assert_eq!(
            replace_at_word_boundaries("fn 名前() { get }", "get", "__E__"),
            "fn 名前() { __E__ }"
        );
        
        assert_eq!(
            replace_at_word_boundaries("🎉🚀✨", "get", "__E__"),
            "🎉🚀✨"
        );
    }

    #[test]
    fn test_fast_path_identical() {
        let content = "hello world";
        let result = entity_merge(content, content, content, "test.ts");
        assert!(result.is_clean());
        assert_eq!(result.content, content);
    }

    #[test]
    fn test_fast_path_only_ours_changed() {
        let base = "hello";
        let ours = "hello world";
        let result = entity_merge(base, ours, base, "test.ts");
        assert!(result.is_clean());
        assert_eq!(result.content, ours);
    }

    #[test]
    fn test_fast_path_only_theirs_changed() {
        let base = "hello";
        let theirs = "hello world";
        let result = entity_merge(base, base, theirs, "test.ts");
        assert!(result.is_clean());
        assert_eq!(result.content, theirs);
    }

    #[test]
    fn test_different_functions_no_conflict() {
        
        let base = r#"export function existing() {
    return 1;
}
"#;
        let ours = r#"export function existing() {
    return 1;
}

export function agentA() {
    return "added by agent A";
}
"#;
        let theirs = r#"export function existing() {
    return 1;
}

export function agentB() {
    return "added by agent B";
}
"#;
        let result = entity_merge(base, ours, theirs, "test.ts");
        assert!(
            result.is_clean(),
            "Should auto-resolve: different functions added. Conflicts: {:?}",
            result.conflicts
        );
        assert!(
            result.content.contains("agentA"),
            "Should contain agentA function"
        );
        assert!(
            result.content.contains("agentB"),
            "Should contain agentB function"
        );
    }

    #[test]
    fn test_same_function_modified_by_both_conflict() {
        let base = r#"export function shared() {
    return "original";
}
"#;
        let ours = r#"export function shared() {
    return "modified by ours";
}
"#;
        let theirs = r#"export function shared() {
    return "modified by theirs";
}
"#;
        let result = entity_merge(base, ours, theirs, "test.ts");
        
        assert!(
            !result.is_clean(),
            "Should conflict when both modify same function differently"
        );
        assert_eq!(result.conflicts.len(), 1);
        assert_eq!(result.conflicts[0].entity_name, "shared");
    }

    #[test]
    fn test_fallback_for_unknown_filetype() {
        
        let base = "line 1\nline 2\nline 3\nline 4\nline 5\n";
        let ours = "line 1 modified\nline 2\nline 3\nline 4\nline 5\n";
        let theirs = "line 1\nline 2\nline 3\nline 4\nline 5 modified\n";
        let result = entity_merge(base, ours, theirs, "test.xyz");
        assert!(
            result.is_clean(),
            "Non-adjacent changes should merge cleanly. Conflicts: {:?}",
            result.conflicts,
        );
    }

    #[test]
    fn test_line_level_fallback() {
        
        let base = "a\nb\nc\nd\ne\n";
        let ours = "A\nb\nc\nd\ne\n";
        let theirs = "a\nb\nc\nd\nE\n";
        let result = line_level_fallback(base, ours, theirs, "test.rs", &Host::default());
        assert!(result.is_clean());
        assert!(result.stats.used_fallback);
        assert_eq!(result.content, "A\nb\nc\nd\nE\n");
    }

    #[test]
    fn test_line_level_fallback_conflict() {
        
        let base = "a\nb\nc\n";
        let ours = "X\nb\nc\n";
        let theirs = "Y\nb\nc\n";
        let result = line_level_fallback(base, ours, theirs, "test.rs", &Host::default());
        assert!(!result.is_clean());
        assert!(result.stats.used_fallback);
    }

    #[test]
    fn test_expand_separators() {
        let code = "function foo() { return 1; }";
        let expanded = expand_separators(code);
        
        let seen: Vec<&str> = expanded.lines().map(str::trim_end).collect();
        assert!(
            seen.iter().any(|l| l.trim_end_matches('\u{1}') == "{"),
            "opening brace should stand alone: {expanded:?}"
        );
        assert!(
            seen.iter().any(|l| l.trim_end_matches('\u{1}') == ";"),
            "semicolon should stand alone: {expanded:?}"
        );
        assert!(
            seen.iter().any(|l| l.trim_end_matches('\u{1}') == "}"),
            "closing brace should stand alone: {expanded:?}"
        );
    }

    #[test]
    fn test_expand_separators_preserves_strings() {
        let code = r#"let x = "hello { world };";"#;
        let expanded = expand_separators(code);
        
        assert!(
            expanded.contains("\"hello { world };\""),
            "Separators in strings should be preserved: {}",
            expanded
        );
    }

    #[test]
    fn separator_expansion_is_invertible() {
        for code in [
            "function foo() { return 1; }",
            "use crate::*;\nuse std::fs;\n",
            "buildscript {\n    repositories {\n        jcenter()\n    }\n}\n\nrepositories {\n}\n",
            "class A {\n\tint x = 1;\n\n\tvoid f() {\n\t\tg();\n\t}\n}\n",
            r#"let x = "hello { world };";"#,
            "no separators here at all\n",
            "",
            "trailing blank lines\n\n\n",
            "\n\nleading blank lines\nx = 1;\n",
        ] {
            assert_eq!(
                collapse_separators(&expand_separators(code)),
                code,
                "collapse ∘ expand must be the identity on {code:?}"
            );
        }
    }

    #[test]
    fn reordered_use_statements_keep_their_lines() {
        let base = "use crate::*;\nuse std::fs;\n";
        let ours = "use std::fs;\nuse crate::*;\n";
        let theirs = "use crate::*;\nuse std::fs;\nuse itertools::Itertools;\n";
        let out = line_level_fallback(base, ours, theirs, "x.rs", &Host::default());
        assert!(out.is_clean(), "should still resolve: {:?}", out.content);
        for line in ["use std::fs;", "use crate::*;", "use itertools::Itertools;"] {
            assert!(
                out.content.lines().any(|l| l.trim_end() == line),
                "{line:?} must survive as a line, got {:?}",
                out.content
            );
        }
    }

    #[test]
    fn test_is_import_region() {
        assert!(is_import_region(
            "import foo from 'foo';\nimport bar from 'bar';\n"
        ));
        assert!(is_import_region("use std::io;\nuse std::fs;\n"));
        assert!(!is_import_region("let x = 1;\nlet y = 2;\n"));
        
        assert!(!is_import_region(
            "import foo from 'foo';\nlet x = 1;\nlet y = 2;\n"
        ));
        
        assert!(!is_import_region(""));
    }

    #[test]
    fn test_is_import_line() {
        
        assert!(is_import_line("import foo from 'foo';"));
        assert!(is_import_line("import { bar } from 'bar';"));
        assert!(is_import_line("from typing import List"));
        
        assert!(is_import_line("use std::io::Read;"));
        
        assert!(is_import_line("#include <stdio.h>"));
        
        assert!(is_import_line("const fs = require('fs');"));
        
        assert!(!is_import_line("let x = 1;"));
        assert!(!is_import_line("function foo() {}"));
    }

    #[test]
    fn test_commutative_import_merge_both_add_different() {
        
        let base = "import a from 'a';\nimport b from 'b';\n";
        let ours = "import a from 'a';\nimport b from 'b';\nimport c from 'c';\n";
        let theirs = "import a from 'a';\nimport b from 'b';\nimport d from 'd';\n";
        let (result, _order_conflict) = merge_imports_commutatively(base, ours, theirs);
        assert!(result.contains("import a from 'a';"));
        assert!(result.contains("import b from 'b';"));
        assert!(result.contains("import c from 'c';"));
        assert!(result.contains("import d from 'd';"));
    }

    #[test]
    fn test_commutative_import_merge_one_removes() {
        
        let base = "import a from 'a';\nimport b from 'b';\nimport c from 'c';\n";
        let ours = "import a from 'a';\nimport c from 'c';\n";
        let theirs = "import a from 'a';\nimport b from 'b';\nimport c from 'c';\n";
        let (result, _order_conflict) = merge_imports_commutatively(base, ours, theirs);
        assert!(result.contains("import a from 'a';"));
        assert!(
            !result.contains("import b from 'b';"),
            "Removed import should stay removed"
        );
        assert!(result.contains("import c from 'c';"));
    }

    #[test]
    fn test_commutative_import_merge_both_add_same() {
        
        let base = "import a from 'a';\n";
        let ours = "import a from 'a';\nimport b from 'b';\n";
        let theirs = "import a from 'a';\nimport b from 'b';\n";
        let (result, _order_conflict) = merge_imports_commutatively(base, ours, theirs);
        let count = result.matches("import b from 'b';").count();
        assert_eq!(count, 1, "Duplicate import should be deduplicated");
    }

    #[test]
    fn test_commutative_import_merge_preserves_file_directive() {
        
        let base = "// @ts-nocheck\nimport { a } from \"./a\";\n";
        let ours = "// @ts-nocheck\nimport { a } from \"./a\";\nimport { b } from \"./b\";\n";
        let theirs = "// @ts-nocheck\nimport { a } from \"./a\";\nimport { c } from \"./c\";\n";
        let (result, _order_conflict) = merge_imports_commutatively(base, ours, theirs);
        assert!(
            result.contains("// @ts-nocheck"),
            "Directive should be preserved. Got: {:?}",
            result
        );
        let nocheck_pos = result.find("// @ts-nocheck").unwrap();
        let first_import_pos = result.find("import").unwrap();
        assert!(
            nocheck_pos < first_import_pos,
            "// @ts-nocheck must stay before imports. Got: {:?}",
            result
        );
        assert!(result.contains("import { b }"), "ours import missing");
        assert!(result.contains("import { c }"), "theirs import missing");
    }

    #[test]
    fn test_commutative_import_merge_preserves_shebang() {
        
        let base = "#!/usr/bin/env node\nimport { a } from \"./a\";\n";
        let ours = "#!/usr/bin/env node\nimport { a } from \"./a\";\nimport { b } from \"./b\";\n";
        let theirs =
            "#!/usr/bin/env node\nimport { a } from \"./a\";\nimport { c } from \"./c\";\n";
        let (result, _order_conflict) = merge_imports_commutatively(base, ours, theirs);
        assert!(
            result.starts_with("#!/usr/bin/env node\n"),
            "Shebang must be first line. Got: {:?}",
            result
        );
    }

    #[test]
    fn test_entity_merge_ts_nocheck_stays_above_imports() {
        
        let base = "// @ts-nocheck\nimport { a } from \"./a\";\n\nexport function f() {}\n";
        let ours =
            "// @ts-nocheck\nimport { a } from \"./a\";\nimport { b } from \"./b\";\n\nexport function f() {}\n";
        let theirs =
            "// @ts-nocheck\nimport { a } from \"./a\";\nimport { c } from \"./c\";\n\nexport function f() {}\n";
        let result = entity_merge(base, ours, theirs, "src/example.ts");
        assert!(
            result.is_clean(),
            "Should merge cleanly. Conflicts: {:?}",
            result.conflicts,
        );
        let nocheck_pos = result.content.find("// @ts-nocheck");
        let first_import_pos = result.content.find("import");
        assert!(
            nocheck_pos.is_some(),
            "// @ts-nocheck must be present. Got:\n{}",
            result.content
        );
        assert!(
            nocheck_pos.unwrap() < first_import_pos.unwrap(),
            "// @ts-nocheck must be before imports. Got:\n{}",
            result.content
        );
    }

    #[test]
    fn test_import_preserved_when_both_edit_declaration() {
        
        let base = "type T = {\n  a: string;\n  b: string;\n};\n";
        let ours = "type T = {\n  a: string;\n  b: string;\n  c: string;\n};\n";
        let theirs = "import { G } from './g';\n\ntype T = {\n  a: G;\n  b: string;\n};\n";
        let result = entity_merge(base, ours, theirs, "test.ts");
        assert!(
            result.content.contains("import { G }"),
            "Import from theirs must be preserved. Got:\n{}",
            result.content
        );
        assert!(
            result.content.contains("c: string"),
            "Field added by ours must be present. Got:\n{}",
            result.content
        );
    }

    #[test]
    fn test_import_preserved_one_sided_entity_change() {
        
        let base = "type T = {\n  a: string;\n};\n";
        let ours = base;
        let theirs = "import { G } from './g';\n\ntype T = {\n  a: G;\n};\n";
        let result = entity_merge(base, ours, theirs, "test.ts");
        assert!(
            result.content.contains("import { G }"),
            "Import must be preserved in one-sided case. Got:\n{}",
            result.content
        );
    }

    #[test]
    fn test_inner_entity_merge_different_methods() {
        
        let base = r#"export class Calculator {
    add(a: number, b: number): number {
        return a + b;
    }

    subtract(a: number, b: number): number {
        return a - b;
    }
}
"#;
        let ours = r#"export class Calculator {
    add(a: number, b: number): number {
        // Added logging
        console.log("adding", a, b);
        return a + b;
    }

    subtract(a: number, b: number): number {
        return a - b;
    }
}
"#;
        let theirs = r#"export class Calculator {
    add(a: number, b: number): number {
        return a + b;
    }

    subtract(a: number, b: number): number {
        // Added validation
        if (b > a) throw new Error("negative");
        return a - b;
    }
}
"#;
        let result = entity_merge(base, ours, theirs, "test.ts");
        assert!(
            result.is_clean(),
            "Different methods modified should auto-merge via inner entity merge. Conflicts: {:?}",
            result.conflicts,
        );
        assert!(
            result.content.contains("console.log"),
            "Should contain ours changes"
        );
        assert!(
            result.content.contains("negative"),
            "Should contain theirs changes"
        );
    }

    #[test]
    fn test_inner_entity_merge_both_add_different_methods() {
        
        let base = r#"export class Calculator {
    add(a: number, b: number): number {
        return a + b;
    }
}
"#;
        let ours = r#"export class Calculator {
    add(a: number, b: number): number {
        return a + b;
    }

    multiply(a: number, b: number): number {
        return a * b;
    }
}
"#;
        let theirs = r#"export class Calculator {
    add(a: number, b: number): number {
        return a + b;
    }

    divide(a: number, b: number): number {
        return a / b;
    }
}
"#;
        let result = entity_merge(base, ours, theirs, "test.ts");
        assert!(
            result.is_clean(),
            "Both adding different methods should auto-merge. Conflicts: {:?}",
            result.conflicts,
        );
        assert!(
            result.content.contains("multiply"),
            "Should contain ours's new method"
        );
        assert!(
            result.content.contains("divide"),
            "Should contain theirs's new method"
        );
    }

    #[test]
    fn test_inner_entity_merge_same_method_modified_still_conflicts() {
        
        let base = r#"export class Calculator {
    add(a: number, b: number): number {
        return a + b;
    }

    subtract(a: number, b: number): number {
        return a - b;
    }
}
"#;
        let ours = r#"export class Calculator {
    add(a: number, b: number): number {
        return a + b + 1;
    }

    subtract(a: number, b: number): number {
        return a - b;
    }
}
"#;
        let theirs = r#"export class Calculator {
    add(a: number, b: number): number {
        return a + b + 2;
    }

    subtract(a: number, b: number): number {
        return a - b;
    }
}
"#;
        let result = entity_merge(base, ours, theirs, "test.ts");
        assert!(
            !result.is_clean(),
            "Both modifying same method differently should still conflict"
        );
    }

    #[test]
    fn test_extract_member_name() {
        assert_eq!(extract_member_name("add(a, b) {"), "add");
        assert_eq!(extract_member_name("fn add(&self, a: i32) -> i32 {"), "add");
        assert_eq!(extract_member_name("def add(self, a, b):"), "add");
        assert_eq!(
            extract_member_name("public static getValue(): number {"),
            "getValue"
        );
        assert_eq!(extract_member_name("async fetchData() {"), "fetchData");
    }

    #[test]
    fn test_commutative_import_merge_rust_use() {
        let base = "use std::io;\nuse std::fs;\n";
        let ours = "use std::io;\nuse std::fs;\nuse std::path::Path;\n";
        let theirs = "use std::io;\nuse std::fs;\nuse std::collections::HashMap;\n";
        let (result, _order_conflict) = merge_imports_commutatively(base, ours, theirs);
        assert!(result.contains("use std::path::Path;"));
        assert!(result.contains("use std::collections::HashMap;"));
        assert!(result.contains("use std::io;"));
        assert!(result.contains("use std::fs;"));
    }

    #[test]
    fn test_is_whitespace_only_diff_true() {
        
        assert!(is_whitespace_only_diff(
            "    return 1;\n    return 2;\n",
            "      return 1;\n      return 2;\n"
        ));
        
        assert!(is_whitespace_only_diff(
            "return 1;\nreturn 2;\n",
            "return 1;\n\nreturn 2;\n"
        ));
    }

    #[test]
    fn test_is_whitespace_only_diff_false() {
        
        assert!(!is_whitespace_only_diff(
            "    return 1;\n",
            "    return 2;\n"
        ));
        
        assert!(!is_whitespace_only_diff(
            "return 1;\n",
            "return 1;\nconsole.log('x');\n"
        ));
    }

    #[test]
    fn test_ts_interface_both_add_different_fields() {
        let base = "interface Config {\n    name: string;\n}\n";
        let ours = "interface Config {\n    name: string;\n    age: number;\n}\n";
        let theirs = "interface Config {\n    name: string;\n    email: string;\n}\n";
        let result = entity_merge(base, ours, theirs, "test.ts");
        eprintln!(
            "TS interface: clean={}, conflicts={:?}",
            result.is_clean(),
            result.conflicts
        );
        eprintln!("Content: {:?}", result.content);
        assert!(
            result.is_clean(),
            "Both adding different fields to TS interface should merge. Conflicts: {:?}",
            result.conflicts,
        );
        assert!(result.content.contains("age"));
        assert!(result.content.contains("email"));
    }

    #[test]
    fn test_rust_enum_both_add_different_variants() {
        let base = "enum Color {\n    Red,\n    Blue,\n}\n";
        let ours = "enum Color {\n    Red,\n    Blue,\n    Green,\n}\n";
        let theirs = "enum Color {\n    Red,\n    Blue,\n    Yellow,\n}\n";
        let result = entity_merge(base, ours, theirs, "test.rs");
        eprintln!(
            "Rust enum: clean={}, conflicts={:?}",
            result.is_clean(),
            result.conflicts
        );
        eprintln!("Content: {:?}", result.content);
        assert!(
            result.is_clean(),
            "Both adding different enum variants should merge. Conflicts: {:?}",
            result.conflicts,
        );
        assert!(result.content.contains("Green"));
        assert!(result.content.contains("Yellow"));
    }

    #[test]
    fn test_python_both_add_different_decorators() {
        
        let base = "def foo():\n    return 1\n\ndef bar():\n    return 2\n";
        let ours = "@cache\ndef foo():\n    return 1\n\ndef bar():\n    return 2\n";
        let theirs = "@deprecated\ndef foo():\n    return 1\n\ndef bar():\n    return 2\n";
        let result = entity_merge(base, ours, theirs, "test.py");
        assert!(
            !result.is_clean(),
            "Both sides adding different decorators must conflict (order is semantic)",
        );
        assert!(result.content.contains("@cache"));
        assert!(result.content.contains("@deprecated"));
    }

    #[test]
    fn test_decorator_plus_body_change() {
        
        let base = "def foo():\n    return 1\n";
        let ours = "@cache\ndef foo():\n    return 1\n";
        let theirs = "def foo():\n    return 42\n";
        let result = entity_merge(base, ours, theirs, "test.py");
        assert!(
            result.is_clean(),
            "Decorator + body change should merge. Conflicts: {:?}",
            result.conflicts,
        );
        assert!(result.content.contains("@cache"));
        assert!(result.content.contains("return 42"));
    }

    #[test]
    fn test_ts_class_decorator_merge() {
        
        let base = "class Foo {\n    bar() {\n        return 1;\n    }\n}\n";
        let ours = "class Foo {\n    @Injectable()\n    bar() {\n        return 1;\n    }\n}\n";
        let theirs = "class Foo {\n    @Deprecated()\n    bar() {\n        return 1;\n    }\n}\n";
        let result = entity_merge(base, ours, theirs, "test.ts");
        
        assert!(
            !result.is_clean(),
            "Both sides adding different decorators must conflict (order is semantic)",
        );
        assert!(result.content.contains("@Injectable()"));
        assert!(result.content.contains("@Deprecated()"));
    }

    #[test]
    fn test_non_adjacent_intra_function_changes() {
        let base = r#"export function process(data: any) {
    const validated = validate(data);
    const transformed = transform(validated);
    const saved = save(transformed);
    return saved;
}
"#;
        let ours = r#"export function process(data: any) {
    const validated = validate(data);
    const transformed = transform(validated);
    const saved = save(transformed);
    console.log("saved", saved);
    return saved;
}
"#;
        let theirs = r#"export function process(data: any) {
    console.log("input", data);
    const validated = validate(data);
    const transformed = transform(validated);
    const saved = save(transformed);
    return saved;
}
"#;
        let result = entity_merge(base, ours, theirs, "test.ts");
        assert!(
            result.is_clean(),
            "Non-adjacent changes within same function should merge via diffy. Conflicts: {:?}",
            result.conflicts,
        );
        assert!(result.content.contains("console.log(\"saved\""));
        assert!(result.content.contains("console.log(\"input\""));
    }

    #[test]
    fn test_method_reordering_with_modification() {
        
        let base = r#"class Service {
    getUser(id: string) {
        return db.find(id);
    }

    createUser(data: any) {
        return db.create(data);
    }

    deleteUser(id: string) {
        return db.delete(id);
    }
}
"#;
        
        let ours = r#"class Service {
    getUser(id: string) {
        return db.find(id);
    }

    deleteUser(id: string) {
        return db.delete(id);
    }

    createUser(data: any) {
        return db.create(data);
    }
}
"#;
        
        let theirs = r#"class Service {
    getUser(id: string) {
        console.log("fetching", id);
        return db.find(id);
    }

    createUser(data: any) {
        return db.create(data);
    }

    deleteUser(id: string) {
        return db.delete(id);
    }
}
"#;
        let result = entity_merge(base, ours, theirs, "test.ts");
        eprintln!(
            "Method reorder: clean={}, conflicts={:?}",
            result.is_clean(),
            result.conflicts
        );
        eprintln!("Content:\n{}", result.content);
        assert!(
            result.is_clean(),
            "Method reordering + modification should merge. Conflicts: {:?}",
            result.conflicts,
        );
        assert!(
            result.content.contains("console.log(\"fetching\""),
            "Should contain theirs modification"
        );
        assert!(
            result.content.contains("deleteUser"),
            "Should have deleteUser"
        );
        assert!(
            result.content.contains("createUser"),
            "Should have createUser"
        );
    }

    #[test]
    fn test_doc_comment_plus_body_change() {
        
        let base = r#"export function calculate(a: number, b: number): number {
    return a + b;
}
"#;
        let ours = r#"/**
 * Calculate the sum of two numbers.
 * @param a - First number
 * @param b - Second number
 */
export function calculate(a: number, b: number): number {
    return a + b;
}
"#;
        let theirs = r#"export function calculate(a: number, b: number): number {
    const result = a + b;
    console.log("result:", result);
    return result;
}
"#;
        let result = entity_merge(base, ours, theirs, "test.ts");
        eprintln!(
            "Doc comment + body: clean={}, conflicts={:?}",
            result.is_clean(),
            result.conflicts
        );
        eprintln!("Content:\n{}", result.content);
        
    }

    #[test]
    fn test_both_add_different_guard_clauses() {
        
        let base = r#"export function processOrder(order: Order): Result {
    const total = calculateTotal(order);
    return { success: true, total };
}
"#;
        let ours = r#"export function processOrder(order: Order): Result {
    if (!order) throw new Error("Order required");
    const total = calculateTotal(order);
    return { success: true, total };
}
"#;
        let theirs = r#"export function processOrder(order: Order): Result {
    if (order.items.length === 0) throw new Error("Empty order");
    const total = calculateTotal(order);
    return { success: true, total };
}
"#;
        let result = entity_merge(base, ours, theirs, "test.ts");
        eprintln!(
            "Guard clauses: clean={}, conflicts={:?}",
            result.is_clean(),
            result.conflicts
        );
        eprintln!("Content:\n{}", result.content);
        
    }

    #[test]
    fn test_both_modify_different_enum_variants() {
        
        let base = r#"enum Status {
    Active = "active",
    Inactive = "inactive",
    Pending = "pending",
}
"#;
        let ours = r#"enum Status {
    Active = "active",
    Inactive = "disabled",
    Pending = "pending",
}
"#;
        let theirs = r#"enum Status {
    Active = "active",
    Inactive = "inactive",
    Pending = "pending",
    Deleted = "deleted",
}
"#;
        let result = entity_merge(base, ours, theirs, "test.ts");
        eprintln!(
            "Enum modify+add: clean={}, conflicts={:?}",
            result.is_clean(),
            result.conflicts
        );
        eprintln!("Content:\n{}", result.content);
        assert!(
            result.is_clean(),
            "Modify variant + add new variant should merge. Conflicts: {:?}",
            result.conflicts,
        );
        assert!(
            result.content.contains("\"disabled\""),
            "Should have modified Inactive"
        );
        assert!(
            result.content.contains("Deleted"),
            "Should have new Deleted variant"
        );
    }

    #[test]
    fn test_config_object_field_additions() {
        
        let base = r#"export const config = {
    timeout: 5000,
    retries: 3,
};
"#;
        let ours = r#"export const config = {
    timeout: 5000,
    retries: 3,
    maxConnections: 10,
};
"#;
        let theirs = r#"export const config = {
    timeout: 5000,
    retries: 3,
    logLevel: "info",
};
"#;
        let result = entity_merge(base, ours, theirs, "test.ts");
        eprintln!(
            "Config fields: clean={}, conflicts={:?}",
            result.is_clean(),
            result.conflicts
        );
        eprintln!("Content:\n{}", result.content);
        
    }

    #[test]
    fn test_call_wrapped_object_scopes_conflict_per_key() {
        
        let base =
            "export const flags = configure({\n  a: 1,\n  b: 2,\n  c: 3,\n  d: 4,\n  e: 5,\n});\n";
        let ours =
            "export const flags = configure({\n  a: 10,\n  b: 2,\n  c: 3,\n  d: 4,\n  e: 50,\n});\n";
        let theirs =
            "export const flags = configure({\n  a: 11,\n  b: 2,\n  c: 3,\n  d: 4,\n  e: 51,\n});\n";
        let result = entity_merge(base, ours, theirs, "test.ts");

        let hunks = result.content.matches("<<<<<<<").count();
        assert_eq!(
            hunks, 2,
            "expected per-key conflicts on `a` and `e`, got {hunks}:\n{}",
            result.content
        );
        
        for key in ["  b: 2,", "  c: 3,", "  d: 4,"] {
            assert!(
                result.content.contains(key),
                "untouched key {key:?} should survive cleanly:\n{}",
                result.content
            );
        }
    }

    #[test]
    fn test_rust_impl_block_both_add_methods() {
        
        let base = r#"impl Calculator {
    fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}
"#;
        let ours = r#"impl Calculator {
    fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }

    fn multiply(&self, a: i32, b: i32) -> i32 {
        a * b
    }
}
"#;
        let theirs = r#"impl Calculator {
    fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }

    fn divide(&self, a: i32, b: i32) -> i32 {
        a / b
    }
}
"#;
        let result = entity_merge(base, ours, theirs, "test.rs");
        eprintln!(
            "Rust impl: clean={}, conflicts={:?}",
            result.is_clean(),
            result.conflicts
        );
        eprintln!("Content:\n{}", result.content);
        assert!(
            result.is_clean(),
            "Both adding methods to Rust impl should merge. Conflicts: {:?}",
            result.conflicts,
        );
        assert!(result.content.contains("multiply"), "Should have multiply");
        assert!(result.content.contains("divide"), "Should have divide");
    }

    #[test]
    fn test_rust_impl_same_trait_different_types() {
        
        let base = r#"struct Foo;
struct Bar;

impl Stream for Foo {
    type Item = i32;
    fn poll_next(&self) -> Option<i32> {
        Some(1)
    }
}

impl Stream for Bar {
    type Item = String;
    fn poll_next(&self) -> Option<String> {
        Some("hello".into())
    }
}

fn other() {}
"#;
        let ours = r#"struct Foo;
struct Bar;

impl Stream for Foo {
    type Item = i32;
    fn poll_next(&self) -> Option<i32> {
        let x = compute();
        Some(x + 1)
    }
}

impl Stream for Bar {
    type Item = String;
    fn poll_next(&self) -> Option<String> {
        Some("hello".into())
    }
}

fn other() {}
"#;
        let theirs = r#"struct Foo;
struct Bar;

impl Stream for Foo {
    type Item = i32;
    fn poll_next(&self) -> Option<i32> {
        Some(1)
    }
}

impl Stream for Bar {
    type Item = String;
    fn poll_next(&self) -> Option<String> {
        let s = format!("hello {}", name);
        Some(s)
    }
}

fn other() {}
"#;
        let result = entity_merge(base, ours, theirs, "test.rs");
        assert!(
            result.is_clean(),
            "Same trait, different types should not conflict. Conflicts: {:?}",
            result.conflicts,
        );
        assert!(
            result.content.contains("impl Stream for Foo"),
            "Should have Foo impl"
        );
        assert!(
            result.content.contains("impl Stream for Bar"),
            "Should have Bar impl"
        );
        assert!(
            result.content.contains("compute()"),
            "Should have ours' Foo change"
        );
        assert!(
            result.content.contains("format!"),
            "Should have theirs' Bar change"
        );
    }

    #[test]
    fn test_rust_doc_comment_plus_body_change() {
        
        let base = r#"fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}
"#;
        let ours = r#"/// Adds two numbers together.
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}
"#;
        let theirs = r#"fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b - 1
}
"#;
        let result = entity_merge(base, ours, theirs, "test.rs");
        assert!(
            result.is_clean(),
            "Rust doc comment + body change should merge. Conflicts: {:?}",
            result.conflicts,
        );
        assert!(
            result.content.contains("/// Adds two numbers"),
            "Should have ours doc comment"
        );
        assert!(
            result.content.contains("a - b - 1"),
            "Should have theirs body change"
        );
    }

    #[test]
    fn test_both_add_different_doc_comments() {
        
        let base = r#"fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}
"#;
        let ours = r#"/// Adds two numbers.
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}
"#;
        let theirs = r#"fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Subtracts b from a.
fn subtract(a: i32, b: i32) -> i32 {
    a - b
}
"#;
        let result = entity_merge(base, ours, theirs, "test.rs");
        assert!(
            result.is_clean(),
            "Both adding doc comments to different functions should merge. Conflicts: {:?}",
            result.conflicts,
        );
        assert!(
            result.content.contains("/// Adds two numbers"),
            "Should have add's doc comment"
        );
        assert!(
            result.content.contains("/// Subtracts b from a"),
            "Should have subtract's doc comment"
        );
    }

    #[test]
    fn test_go_import_block_both_add_different() {
        
        let base = "package main\n\nimport (\n\t\"fmt\"\n\t\"os\"\n)\n\nfunc main() {\n\tfmt.Println(\"hello\")\n}\n";
        let ours = "package main\n\nimport (\n\t\"fmt\"\n\t\"os\"\n\t\"strings\"\n)\n\nfunc main() {\n\tfmt.Println(\"hello\")\n}\n";
        let theirs = "package main\n\nimport (\n\t\"fmt\"\n\t\"os\"\n\t\"io\"\n)\n\nfunc main() {\n\tfmt.Println(\"hello\")\n}\n";
        let result = entity_merge(base, ours, theirs, "main.go");
        eprintln!(
            "Go import block: clean={}, conflicts={:?}",
            result.is_clean(),
            result.conflicts
        );
        eprintln!("Content:\n{}", result.content);
        
    }

    #[test]
    fn test_python_class_both_add_methods() {
        
        let base = "class Calculator:\n    def add(self, a, b):\n        return a + b\n";
        let ours = "class Calculator:\n    def add(self, a, b):\n        return a + b\n\n    def multiply(self, a, b):\n        return a * b\n";
        let theirs = "class Calculator:\n    def add(self, a, b):\n        return a + b\n\n    def divide(self, a, b):\n        return a / b\n";
        let result = entity_merge(base, ours, theirs, "test.py");
        eprintln!(
            "Python class: clean={}, conflicts={:?}",
            result.is_clean(),
            result.conflicts
        );
        eprintln!("Content:\n{}", result.content);
        assert!(
            result.is_clean(),
            "Both adding methods to Python class should merge. Conflicts: {:?}",
            result.conflicts,
        );
        assert!(result.content.contains("multiply"), "Should have multiply");
        assert!(result.content.contains("divide"), "Should have divide");
    }

    #[test]
    fn test_interstitial_conflict_not_silently_embedded() {
        
        let base = r#"export { alpha } from "./alpha";

// Section: data utilities
// TODO: add more exports here

export { beta } from "./beta";
"#;
        let ours = r#"export { alpha } from "./alpha";

// Section: data utilities (sorting)
// Sorting helpers for list views

export { beta } from "./beta";
"#;
        let theirs = r#"export { alpha } from "./alpha";

// Section: data utilities (filtering)
// Filtering helpers for search views

export { beta } from "./beta";
"#;
        let result = entity_merge(base, ours, theirs, "index.ts");

        let has_markers = result.content.contains("<<<<<<<") || result.content.contains(">>>>>>>");
        if has_markers {
            assert!(
                !result.is_clean(),
                "BUG: is_clean()=true but merged content has conflict markers!\n\
                 stats: {}\nconflicts: {:?}\ncontent:\n{}",
                result.stats,
                result.conflicts,
                result.content
            );
            assert!(
                result.stats.entities_conflicted > 0,
                "entities_conflicted should be > 0 when markers are present"
            );
        }

        if result.is_clean() {
            assert!(
                !has_markers,
                "Clean merge should not contain conflict markers!\ncontent:\n{}",
                result.content
            );
        }
    }

    #[test]
    fn test_pre_conflicted_input_not_treated_as_clean() {
        
        let base = "";
        let theirs = "";
        let ours = r#"/**
 * MIT License
 */

<<<<<<<< HEAD:src/lib/exports/index.ts
export { renderDocToBuffer } from "./doc-exporter";
export type { ExportOptions, ExportMetadata, RenderContext } from "./types";
========
export * from "./editor";
export * from "./types";
>>>>>>>> feature:packages/core/src/editor/index.ts
"#;
        let result = entity_merge(base, ours, theirs, "index.ts");

        assert!(
            !result.is_clean(),
            "Pre-conflicted input must not be reported as clean!\n\
             stats: {}\nconflicts: {:?}",
            result.stats,
            result.conflicts,
        );
        assert!(result.stats.entities_conflicted > 0);
        assert!(!result.conflicts.is_empty());
    }

    #[test]
    fn test_multi_line_signature_classified_as_syntax() {
        
        let base = "function process(\n    a: number,\n    b: string\n) {\n    return a;\n}\n";
        let ours = "function process(\n    a: number,\n    b: string,\n    c: boolean\n) {\n    return a;\n}\n";
        let theirs = "function process(\n    a: number,\n    b: number\n) {\n    return a;\n}\n";
        let complexity = crate::conflict::classify_conflict(Some(base), Some(ours), Some(theirs));
        assert_eq!(
            complexity,
            crate::conflict::ConflictComplexity::Syntax,
            "Multi-line signature change should be classified as Syntax, got {:?}",
            complexity
        );
    }

    #[test]
    fn test_grouped_import_merge_preserves_groups() {
        let base = "import os\nimport sys\n\nfrom collections import OrderedDict\nfrom typing import List\n";
        let ours = "import os\nimport sys\nimport json\n\nfrom collections import OrderedDict\nfrom typing import List\n";
        let theirs = "import os\nimport sys\n\nfrom collections import OrderedDict\nfrom collections import defaultdict\nfrom typing import List\n";
        let (result, _order_conflict) = merge_imports_commutatively(base, ours, theirs);
        
        let lines: Vec<&str> = result.lines().collect();
        let json_idx = lines.iter().position(|l| l.contains("json"));
        let blank_idx = lines.iter().position(|l| l.trim().is_empty());
        let defaultdict_idx = lines.iter().position(|l| l.contains("defaultdict"));
        assert!(json_idx.is_some(), "json import should be present");
        assert!(
            blank_idx.is_some(),
            "blank line separator should be present"
        );
        assert!(
            defaultdict_idx.is_some(),
            "defaultdict import should be present"
        );
        
        assert!(
            json_idx.unwrap() < blank_idx.unwrap(),
            "json should be in first group"
        );
        assert!(
            defaultdict_idx.unwrap() > blank_idx.unwrap(),
            "defaultdict should be in second group"
        );
    }

    #[test]
    fn test_configurable_duplicate_threshold() {
        
        let entities: Vec<SemanticEntity> = (0..15)
            .map(|i| SemanticEntity {
                id: format!("test::function::test_{}", i),
                file_path: "test.ts".to_string(),
                entity_type: "function".to_string(),
                name: "test".to_string(),
                parent_id: None,
                content: format!("function test() {{ return {}; }}", i),
                content_hash: format!("hash_{}", i),
                structural_hash: None,
                start_line: i * 3 + 1,
                end_line: i * 3 + 3,
                start_byte: None,
                end_byte: None,
                metadata: None,
            })
            .collect();
        
        assert!(has_excessive_duplicates(
            &entities,
            Host::default().max_duplicates
        ));
        
        assert!(!has_excessive_duplicates(&entities, 20));
    }

    #[test]
    fn test_ts_multiline_import_consolidation() {
        
        let base = "\
import type { Foo } from \"./foo\"
import {
     type a,
     type b,
     type c,
} from \"./foo\"

export function bar() {
    return 1;
}
";
        let ours = base;
        let theirs = "\
import {
     type Foo,
     type a,
     type b,
     type c,
} from \"./foo\"

export function bar() {
    return 1;
}
";
        let result = entity_merge(base, ours, theirs, "test.ts");
        eprintln!(
            "TS import consolidation: clean={}, conflicts={:?}",
            result.is_clean(),
            result.conflicts
        );
        eprintln!("Content:\n{}", result.content);
        
        assert!(
            result.content.contains("import {"),
            "import {{ must not be dropped"
        );
        assert!(
            result.content.contains("type Foo,"),
            "type Foo must be present"
        );
        assert!(
            result.content.contains("} from \"./foo\""),
            "closing must be present"
        );
        assert!(
            !result.content.contains("import type { Foo }"),
            "old separate import should be removed"
        );
    }

    #[test]
    fn test_ts_multiline_import_both_modify() {
        
        let base = "\
import type { Foo } from \"./foo\"
import {
     type a,
     type b,
     type c,
} from \"./foo\"

export function bar() {
    return 1;
}
";
        
        let ours = "\
import {
     type Foo,
     type a,
     type b,
     type c,
     type d,
} from \"./foo\"

export function bar() {
    return 1;
}
";
        
        let theirs = "\
import {
     type Foo,
     type a,
     type b,
     type c,
     type e,
} from \"./foo\"

export function bar() {
    return 1;
}
";
        let result = entity_merge(base, ours, theirs, "test.ts");
        eprintln!(
            "TS import both modify: clean={}, conflicts={:?}",
            result.is_clean(),
            result.conflicts
        );
        eprintln!("Content:\n{}", result.content);
        assert!(
            result.content.contains("import {"),
            "import {{ must not be dropped"
        );
        assert!(
            result.content.contains("type Foo,"),
            "type Foo must be present"
        );
        assert!(
            result.content.contains("type d,"),
            "ours addition must be present"
        );
        assert!(
            result.content.contains("type e,"),
            "theirs addition must be present"
        );
        assert!(
            result.content.contains("} from \"./foo\""),
            "closing must be present"
        );
    }

    #[test]
    fn test_ts_multiline_import_no_entities() {
        
        let base = "\
import type { Foo } from \"./foo\"
import {
     type a,
     type b,
     type c,
} from \"./foo\"
";
        let ours = base;
        let theirs = "\
import {
     type Foo,
     type a,
     type b,
     type c,
} from \"./foo\"
";
        let result = entity_merge(base, ours, theirs, "test.ts");
        eprintln!(
            "TS import no entities: clean={}, conflicts={:?}",
            result.is_clean(),
            result.conflicts
        );
        eprintln!("Content:\n{}", result.content);
        assert!(
            result.content.contains("import {"),
            "import {{ must not be dropped"
        );
        assert!(
            result.content.contains("type Foo,"),
            "type Foo must be present"
        );
    }

    #[test]
    fn test_ts_multiline_import_export_variable() {
        
        let base = "\
import type { Foo } from \"./foo\"
import {
     type a,
     type b,
     type c,
} from \"./foo\"

export const X = 1;

export function bar() {
    return 1;
}
";
        let ours = "\
import type { Foo } from \"./foo\"
import {
     type a,
     type b,
     type c,
     type d,
} from \"./foo\"

export const X = 1;

export function bar() {
    return 1;
}
";
        let theirs = "\
import {
     type Foo,
     type a,
     type b,
     type c,
} from \"./foo\"

export const X = 2;

export function bar() {
    return 1;
}
";
        let result = entity_merge(base, ours, theirs, "test.ts");
        eprintln!(
            "TS import + export var: clean={}, conflicts={:?}",
            result.is_clean(),
            result.conflicts
        );
        eprintln!("Content:\n{}", result.content);
        assert!(
            result.content.contains("import {"),
            "import {{ must not be dropped"
        );
    }

    #[test]
    fn test_ts_multiline_import_adjacent_to_entity() {
        
        let base = "\
import type { Foo } from \"./foo\"
import {
     type a,
     type b,
     type c,
} from \"./foo\"
export function bar() {
    return 1;
}
";
        let ours = base;
        let theirs = "\
import {
     type Foo,
     type a,
     type b,
     type c,
} from \"./foo\"
export function bar() {
    return 1;
}
";
        let result = entity_merge(base, ours, theirs, "test.ts");
        eprintln!(
            "TS import adjacent: clean={}, conflicts={:?}",
            result.is_clean(),
            result.conflicts
        );
        eprintln!("Content:\n{}", result.content);
        assert!(
            result.content.contains("import {"),
            "import {{ must not be dropped"
        );
        assert!(
            result.content.contains("type Foo,"),
            "type Foo must be present"
        );
    }

    #[test]
    fn test_ts_multiline_import_both_consolidate_differently() {
        
        let base = "\
import type { Foo } from \"./foo\"
import {
     type a,
     type b,
} from \"./foo\"

export function bar() {
    return 1;
}
";
        let ours = "\
import {
     type Foo,
     type a,
     type b,
     type c,
} from \"./foo\"

export function bar() {
    return 1;
}
";
        let theirs = "\
import {
     type Foo,
     type a,
     type b,
     type d,
} from \"./foo\"

export function bar() {
    return 1;
}
";
        let result = entity_merge(base, ours, theirs, "test.ts");
        eprintln!(
            "TS both consolidate: clean={}, conflicts={:?}",
            result.is_clean(),
            result.conflicts
        );
        eprintln!("Content:\n{}", result.content);
        assert!(
            result.content.contains("import {"),
            "import {{ must not be dropped"
        );
        assert!(
            result.content.contains("type Foo,"),
            "type Foo must be present"
        );
        assert!(
            result.content.contains("} from \"./foo\""),
            "closing must be present"
        );
    }

    #[test]
    fn test_ts_multiline_import_ours_adds_theirs_consolidates() {
        
        let base = "\
import type { Foo } from \"./foo\"
import {
     type a,
     type b,
     type c,
} from \"./foo\"

export function bar() {
    return 1;
}
";
        
        let ours = "\
import type { Foo } from \"./foo\"
import {
     type a,
     type b,
     type c,
     type d,
} from \"./foo\"

export function bar() {
    return 1;
}
";
        
        let theirs = "\
import {
     type Foo,
     type a,
     type b,
     type c,
} from \"./foo\"

export function bar() {
    return 1;
}
";
        let result = entity_merge(base, ours, theirs, "test.ts");
        eprintln!(
            "TS import ours-adds theirs-consolidates: clean={}, conflicts={:?}",
            result.is_clean(),
            result.conflicts
        );
        eprintln!("Content:\n{}", result.content);
        assert!(
            result.content.contains("import {"),
            "import {{ must not be dropped"
        );
        assert!(
            result.content.contains("type d,"),
            "ours addition must be present"
        );
        assert!(
            result.content.contains("} from \"./foo\""),
            "closing must be present"
        );
    }

    #[test]
    fn test_ts_multiline_import_multiple_sources_no_closing_leak() {
        
        let base = "\
import {
    type A,
} from \"./file1\"
import {
    type B,
} from \"./file2\"
import {
    type C,
} from \"./file3\"

export function main() { return 1; }
";
        
        let ours = "\
import {
    type A,
    type A2,
} from \"./file1\"
import {
    type B,
} from \"./file2\"
import {
    type C,
} from \"./file3\"
import {
    type D,
} from \"./file4\"

export function main() { return 1; }
";
        
        let theirs = "\
import {
    type A,
} from \"./file1\"
import {
    type B,
} from \"./file2\"
import {
    type C,
    type C2,
} from \"./file3\"

export function main() { return 1; }
";
        let result = entity_merge(base, ours, theirs, "test.ts");
        eprintln!("Multiple source imports: clean={}", result.is_clean());
        eprintln!("Content:\n{}", result.content);

        assert!(result.content.contains("type A,"), "A must be present");
        assert!(
            result.content.contains("type A2,"),
            "A2 (ours addition) must be present"
        );
        assert!(result.content.contains("type B,"), "B must be present");
        assert!(result.content.contains("type C,"), "C must be present");
        assert!(
            result.content.contains("type C2,"),
            "C2 (theirs addition) must be present"
        );
        assert!(
            result.content.contains("type D,"),
            "D (ours new import) must be present"
        );

        let open_count = result
            .content
            .lines()
            .filter(|l| l.trim().starts_with("import {"))
            .count();
        let close_count = result
            .content
            .lines()
            .filter(|l| {
                let t = l.trim();
                t.starts_with('}') && t.contains("from ")
            })
            .count();
        assert_eq!(
            open_count, close_count,
            "import {{ and }} from must be balanced: {} opens vs {} closes\n{}",
            open_count, close_count, result.content
        );
    }

    #[test]
    fn test_rename_plus_modify_auto_resolves() {
        
        let base = r#"export const cubeQueryExecutorTool = tool({
    name: "cubeQueryExecutorTool",
    description: "Execute a cube query",
    schema: z.object({ query: z.string() }),
    execute: async (input) => {
        return await runQuery(input.query);
    },
});
"#;
        
        let ours = r#"export const cubeQueryTool = tool({
    name: "cubeQueryTool",
    description: "Execute a cube query",
    schema: z.object({ query: z.string() }),
    execute: async (input) => {
        return await runQuery(input.query);
    },
});
"#;
        
        let theirs = r#"export const cubeQueryExecutorTool = tool({
    name: "cubeQueryExecutorTool",
    description: "Execute a cube query with unit inference",
    schema: z.object({ query: z.string(), unit: z.string().optional() }),
    execute: async (input) => {
        const unit = input.unit || inferUnit(input.query);
        return await runQuery(input.query, unit);
    },
});
"#;
        let result = entity_merge(base, ours, theirs, "cubeQueryTool.ts");
        
        assert_eq!(
            result.conflicts.len(),
            1,
            "Should have exactly one conflict"
        );
        assert!(
            matches!(
                result.conflicts[0].kind,
                ConflictKind::RenameModify {
                    renamed_in_ours: true,
                    ..
                }
            ),
            "Should be a RenameModify conflict, got: {:?}",
            result.conflicts[0].kind
        );
        
        assert!(
            result.content.contains("cubeQueryTool"),
            "Ours (renamed) should be in conflict markers"
        );
        assert!(
            result.content.contains("unit inference"),
            "Theirs (modified) should be in conflict markers"
        );
    }
}
