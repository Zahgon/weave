
use std::collections::HashMap;

use crate::binding::{declared_name, footprint, footprints_disjoint, is_trivia_line, word_tokens};
use crate::container::{drops_unanimous_lines, InnerMergeResult};
use crate::merge::ScopeMarkers;
use crate::merge::{diffy_merge, scoped_conflict_marker};

macro_rules! part_claim {
    ($name:ident) => {
        #[derive(Debug, PartialEq, Eq)]
        pub(crate) struct $name(usize);
        impl $name {
            fn new(i: usize) -> Self {
                Self(i)
            }
            fn index(&self) -> usize {
                self.0
            }
        }
    };
}

part_claim!(BasePart);
part_claim!(OursPart);
part_claim!(TheirsPart);

#[derive(Debug, Clone)]
struct Part {
    
    key: String,
    
    lead: String,
    text: String,
}

#[derive(Debug)]
struct Partition {
    header: String,
    
    prelude: String,
    parts: Vec<Part>,
    tail: String,
    footer: String,
}

impl Partition {
    
    fn claims<T>(&self, mk: impl Fn(usize) -> T) -> Vec<T> { panic!("STUB: not implemented") }
}

fn join(lines: &[&str]) -> String { panic!("STUB: not implemented") }

fn statement_key(text: &str) -> String { panic!("STUB: not implemented") }

fn similarity(a: &str, b: &str) -> f64 { panic!("STUB: not implemented") }

fn jaccard<T: std::hash::Hash + Eq>(a: impl Iterator<Item = T>, b: impl Iterator<Item = T>) -> f64 { panic!("STUB: not implemented") }

fn words_jaccard(a: &str, b: &str) -> f64 { panic!("STUB: not implemented") }

fn bigram_jaccard(a: &str, b: &str) -> f64 { panic!("STUB: not implemented") }

fn depths(lines: &[&str]) -> Option<Vec<i32>> { panic!("STUB: not implemented") }

fn close_on_line(chars: &[char], start: usize, quote: char) -> Option<usize> { panic!("STUB: not implemented") }

fn is_continuation(trimmed: &str, prev: &str) -> bool { panic!("STUB: not implemented") }

fn statement_spans(
    lines: &[&str],
    body_start: usize,
    body_end: usize,
) -> Option<Vec<(usize, usize)>> { panic!("STUB: not implemented") }

fn attach_leading_trivia(lines: &[&str], start: usize, floor: usize) -> usize { panic!("STUB: not implemented") }

fn block_wrapper(lines: &[&str]) -> Option<(usize, usize)> { panic!("STUB: not implemented") }

fn partition(content: &str) -> Option<Partition> { panic!("STUB: not implemented") }

#[derive(Debug, Clone, PartialEq)]
enum Action {
    Unchanged,
    
    Edited(usize),
    Deleted,
}

#[derive(Debug)]
enum SideEdit<'a> {
    Unchanged,
    Edited(&'a Part),
    Deleted,
}

impl<'a> SideEdit<'a> {
    
    fn resolve(action: &Action, parts: &'a [Part], locate: impl Fn(usize) -> usize) -> Self { panic!("STUB: not implemented") }

    fn part(&self) -> Option<&'a Part> { panic!("STUB: not implemented") }
}

#[derive(Debug)]
struct Alignment {
    
    per_base: Vec<Action>,
    
    additions: Vec<(usize, usize)>,
}

const MAX_LCS: usize = 250_000;

fn lcs_pairs(a: &[String], b: &[String]) -> Option<Vec<(usize, usize)>> { panic!("STUB: not implemented") }

const EDIT_SIMILARITY: f64 = 0.4;

fn best_matching(a: &[String], b: &[String]) -> Vec<(usize, usize)> { panic!("STUB: not implemented") }

fn align(base: &Partition, side: &Partition) -> Option<Alignment> { panic!("STUB: not implemented") }

fn merge_trivia(
    name: &str,
    b: &str,
    o: &str,
    t: &str,
    fmt: &ScopeMarkers<'_>,
    has_conflict: &mut bool,
) -> String { panic!("STUB: not implemented") }

fn same_gap_composition_licensed(
    mine: &[Part],
    yours: &[Part],
    license: License,
) -> Option<LicensedGap> { panic!("STUB: not implemented") }

fn independent_declarations(mine: &[Part], yours: &[Part]) -> Option<LicensedGap> { panic!("STUB: not implemented") }

fn formatting_tie(a: &str, b: &str) -> Option<String> { panic!("STUB: not implemented") }

fn key_of(parts: &[Part]) -> String { panic!("STUB: not implemented") }

fn label(text: &str) -> String { panic!("STUB: not implemented") }

pub(crate) fn statement_merge(
    base: &str,
    ours: &str,
    theirs: &str,
    fmt: &ScopeMarkers<'_>,
) -> Option<InnerMergeResult> { panic!("STUB: not implemented") }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum License {
    Refused,
    Footprint,
}

#[derive(Debug, Clone)]
pub(crate) struct LicensedGap {
    pub ours_binds: Vec<String>,
    pub theirs_binds: Vec<String>,
}

pub(crate) fn statement_merge_with(
    base: &str,
    ours: &str,
    theirs: &str,
    fmt: &ScopeMarkers<'_>,
    license: License,
    evidence: &mut Vec<LicensedGap>,
) -> Option<InnerMergeResult> { panic!("STUB: not implemented") }

pub(crate) fn statement_merge_licensed(
    base: &str,
    ours: &str,
    theirs: &str,
    fmt: &ScopeMarkers<'_>,
) -> Option<(InnerMergeResult, Vec<LicensedGap>)> { panic!("STUB: not implemented") }

const MAX_DEPTH: u8 = 3;

fn statement_merge_inner(
    base: &str,
    ours: &str,
    theirs: &str,
    fmt: &ScopeMarkers<'_>,
    depth: u8,
    license: License,
    evidence: &mut Vec<LicensedGap>,
    
    composed: &mut Vec<String>,
) -> Option<InnerMergeResult> { panic!("STUB: not implemented") }

fn fabricates_lines(base: &str, ours: &str, theirs: &str, out: &str, licensed: &[String]) -> bool { panic!("STUB: not implemented") }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::conflict::MarkerFormat;

    const SAMPLES: &[&str] = &[
        
        "def f(x):\n    total = 0\n    for i in x:\n        total += i\n\n    # done\n    return total\n",
        
        "function f(x: number[]): number {\n  let total = 0;\n  for (const i of x) {\n    total += i;\n  }\n\n  // done\n  return total;\n}\n",
        
        "fn f(x: &[i32]) -> i32 {\n    let mut total = 0;\n    for i in x {\n        total += i;\n    }\n\n    total\n}\n",
        
        "public int f(int[] x) {\n    int total = 0;\n    for (int i : x) {\n        total += i;\n    }\n    if (total > 0) {\n        total--;\n    } else {\n        total++;\n    }\n    return total;\n}\n",
        
        "func f(x []int) int {\n\ttotal := 0\n\tfor _, i := range x {\n\t\ttotal += i\n\t}\n\treturn total\n}\n",
        
        "fn g() {\n    foo(a,\nb);\n    bar();\n}\n",
        
        "fn h<'a>(s: &'a str) -> &'a str {\n    let t = s.trim();\n    t\n}\n",
    ];

    #[test]
    fn a_partition_tiles_the_body_it_came_from() {
        for content in SAMPLES {
            let p = partition(content).unwrap_or_else(|| panic!("partitions: {content:?}"));
            let mut rebuilt = p.header.clone();
            rebuilt.push_str(&p.prelude);
            for part in &p.parts {
                rebuilt.push_str(&part.lead);
                rebuilt.push_str(&part.text);
            }
            rebuilt.push_str(&p.tail);
            rebuilt.push_str(&p.footer);
            assert_eq!(rebuilt, *content, "partition lost or duplicated text");
        }
    }

    #[test]
    fn a_continuation_line_does_not_start_a_statement() {
        let p = partition("fn g() {\n    foo(a,\nb);\n    bar();\n}\n").unwrap();
        assert_eq!(p.parts.len(), 2, "{:?}", p.parts);
        assert_eq!(p.parts[0].text, "    foo(a,\nb);\n");
    }

    #[test]
    fn an_else_arm_stays_with_its_if() {
        let p = partition(SAMPLES[3]).unwrap();
        let keys: Vec<&str> = p.parts.iter().map(|x| x.key.as_str()).collect();
        assert_eq!(keys.len(), 4, "{keys:?}");
        assert!(keys[2].starts_with("if (total > 0) {"), "{keys:?}");
        assert!(keys[2].contains("} else {"), "{keys:?}");
    }

    #[test]
    fn disjoint_statement_edits_merge() {
        let base = "fn f() {\n    let a = 1;\n    let b = 2;\n    let c = 3;\n}\n";
        let ours = "fn f() {\n    let a = 11;\n    let b = 2;\n    let c = 3;\n}\n";
        let theirs = "fn f() {\n    let a = 1;\n    let b = 2;\n    let c = 33;\n}\n";
        let r = statement_merge(
            base,
            ours,
            theirs,
            &ScopeMarkers::bare(&MarkerFormat::default()),
        )
        .expect("merges");
        assert!(!r.has_conflicts);
        assert_eq!(
            r.content,
            "fn f() {\n    let a = 11;\n    let b = 2;\n    let c = 33;\n}\n"
        );
    }

    #[test]
    fn a_move_is_not_a_conflict_with_an_edit_elsewhere() {
        let base = "fn f() {\n    a();\n    b();\n    c();\n}\n";
        
        let ours = "fn f() {\n    b();\n    c();\n    a();\n}\n";
        let theirs = "fn f() {\n    a();\n    b(1);\n    c();\n}\n";
        let r = statement_merge(
            base,
            ours,
            theirs,
            &ScopeMarkers::bare(&MarkerFormat::default()),
        )
        .expect("merges");
        assert!(!r.has_conflicts, "{}", r.content);
        assert_eq!(r.content, "fn f() {\n    b(1);\n    c();\n    a();\n}\n");
    }

    #[test]
    fn two_insertions_into_one_gap_are_a_conflict() {
        let base = "fn f() {\n    a();\n    z();\n}\n";
        let ours = "fn f() {\n    a();\n    m();\n    z();\n}\n";
        let theirs = "fn f() {\n    a();\n    n();\n    z();\n}\n";
        let r = statement_merge(
            base,
            ours,
            theirs,
            &ScopeMarkers::bare(&MarkerFormat::default()),
        )
        .expect("merges");
        assert!(r.has_conflicts, "{}", r.content);
    }

    #[test]
    fn insertions_into_different_gaps_both_land() {
        let base = "fn f() {\n    a();\n    z();\n    q();\n}\n";
        let ours = "fn f() {\n    a();\n    m();\n    z();\n    q();\n}\n";
        let theirs = "fn f() {\n    a();\n    z();\n    n();\n    q();\n}\n";
        let r = statement_merge(
            base,
            ours,
            theirs,
            &ScopeMarkers::bare(&MarkerFormat::default()),
        )
        .expect("merges");
        assert!(!r.has_conflicts, "{}", r.content);
        assert_eq!(
            r.content,
            "fn f() {\n    a();\n    m();\n    z();\n    n();\n    q();\n}\n"
        );
    }

    #[test]
    fn delete_versus_edit_of_one_statement_conflicts_on_that_statement() {
        let base = "fn f() {\n    a();\n    b();\n    c();\n}\n";
        let ours = "fn f() {\n    a();\n    c();\n}\n";
        let theirs = "fn f() {\n    a();\n    b(2);\n    c();\n}\n";
        let r = statement_merge(
            base,
            ours,
            theirs,
            &ScopeMarkers::bare(&MarkerFormat::default()),
        )
        .expect("merges");
        assert!(r.has_conflicts);
        assert!(r.content.contains("a();"), "{}", r.content);
        assert!(r.content.contains("c();"), "{}", r.content);
    }

    #[test]
    fn an_unterminated_literal_is_refused_but_a_fragment_is_not() {
        assert!(depths(&["fn f() {", "    /* open"]).is_none());
        assert!(
            depths(&["    a();", "}"]).is_some(),
            "a fragment still scans"
        );
        let d = depths(&["fn f() {", "    a();"]).expect("a fragment still scans");
        assert_eq!(d, vec![0, 1], "depth is reported, not judged");
    }

    #[test]
    fn statements_are_found_at_the_fragments_own_level() {
        let p = partition("    if x:\n        a()\n        b()\n").expect("partitions");
        let keys: Vec<&str> = p.parts.iter().map(|x| x.key.as_str()).collect();
        assert_eq!(keys, vec!["a()\n", "b()\n"], "{keys:?}");
    }

    #[test]
    fn the_statement_merge_is_a_function_of_its_inputs() {
        let base = "fn f() {\n    a();\n    b();\n    c();\n    d();\n}\n";
        let ours = "fn f() {\n    a2();\n    b();\n    c();\n    x();\n    d();\n}\n";
        let theirs = "fn f() {\n    a();\n    b();\n    c2();\n    d();\n}\n";
        let first = statement_merge(
            base,
            ours,
            theirs,
            &ScopeMarkers::bare(&MarkerFormat::default()),
        )
        .map(|r| (r.content, r.has_conflicts));
        for _ in 0..24 {
            let again = statement_merge(
                base,
                ours,
                theirs,
                &ScopeMarkers::bare(&MarkerFormat::default()),
            )
            .map(|r| (r.content, r.has_conflicts));
            assert_eq!(first, again, "statement merge is not a function");
        }
    }
}
