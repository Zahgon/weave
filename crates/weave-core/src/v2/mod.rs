
pub mod bind;
pub mod classify;
pub mod match_phase;
pub mod plan;
pub mod render;
pub mod resolve;
pub mod types;

use std::borrow::Cow;
use std::collections::HashMap;

use sem_core::parser::registry::ParserRegistry;

pub(crate) use classify::classify;
pub(crate) use match_phase::match_phase;
pub(crate) use match_phase::{build_arena, RawEntity};
pub use types::*;

use crate::conflict::{EntityConflict, MarkerFormat, MergeStats};
use crate::host::Host;
use crate::merge::{EntityAudit, MergeResult};
use crate::region::{extract_regions, FileRegion};
use crate::validate::{SemanticWarning, WarningKind};

pub(crate) fn normalize_encoding(input: &str) -> Cow<'_, str> { panic!("STUB: not implemented") }

#[cfg(test)]
pub(crate) fn encoding_differs(base: &str, ours: &str, theirs: &str) -> bool {
    let raw_crlf = |s: &str| s.contains("\r\n");
    let bom = |s: &str| s.starts_with('\u{feff}');
    let crlf = [raw_crlf(base), raw_crlf(ours), raw_crlf(theirs)];
    let boms = [bom(base), bom(ours), bom(theirs)];
    crlf.iter().any(|x| *x) && !crlf.iter().all(|x| *x)
        || boms.iter().any(|x| *x) && !boms.iter().all(|x| *x)
}

pub(crate) fn read_version(
    content: &str,
    file_path: &str,
    registry: &ParserRegistry,
) -> Option<Vec<RawEntity>> { panic!("STUB: not implemented") }

pub(crate) fn analyze(
    base: &str,
    ours: &str,
    theirs: &str,
    file_path: &str,
    registry: &ParserRegistry,
) -> Option<Analysis> { panic!("STUB: not implemented") }

pub fn analyze_default(base: &str, ours: &str, theirs: &str, file_path: &str) -> Option<Analysis> { panic!("STUB: not implemented") }

#[derive(Debug)]
pub struct Analysis {
    pub matching: Matching,
    
    pub cells: Vec<Cell>,
}

impl Analysis {
    pub fn iter(&self) -> impl Iterator<Item = (&Triple, Cell)> {
        self.matching.triples.iter().zip(self.cells.iter().copied())
    }
    
    pub fn label(&self, triple: &Triple) -> String { panic!("STUB: not implemented") }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crlf_and_bom_normalize_to_one_shape() {
        assert_eq!(normalize_encoding("a\r\nb\r\n"), "a\nb\n");
        assert_eq!(normalize_encoding("\u{feff}a\nb\n"), "a\nb\n");
        assert_eq!(normalize_encoding("\u{feff}a\r\nb\r\n"), "a\nb\n");
        assert_eq!(normalize_encoding("a\nb\n"), "a\nb\n");
    }

    #[test]
    fn a_one_sided_crlf_conversion_is_reported_as_an_encoding_change() {
        assert!(encoding_differs("a\n", "a\r\n", "a\n"));
        assert!(!encoding_differs("a\n", "a\n", "a\n"));
        assert!(!encoding_differs("a\r\n", "a\r\n", "a\r\n"));
    }

    #[test]
    fn the_summary_cannot_disagree_with_the_decisions_it_summarises() {
        
        let cases: [(&str, &str, &str); 4] = [
            (
                "def a():\n    return 1\n",
                "def a():\n    return 2\n",
                "def a():\n    return 3\n",
            ),
            (
                "def load(i):\n    return i\n\n\ndef use():\n    return load(1)\n",
                "def fetch(i):\n    return i\n\n\ndef use():\n    return load(1)\n",
                "def load(i):\n    return i\n\n\ndef use():\n    return load(2)\n",
            ),
            (
                "def gone():\n    return 1\n",
                "",
                "def gone():\n    return 1\n\n\ndef caller():\n    return gone()\n",
            ),
            (
                "def keep():\n    return 1\n",
                "def keep():\n    return 1\n\n\ndef helper():\n    return 9\n",
                "def keep():\n    return 1\n\n\ndef other():\n    return helper()\n",
            ),
        ];
        for (base, ours, theirs) in cases {
            let Ok(r) = merge_file(
                base,
                ours,
                theirs,
                "m.py",
                &crate::merge::PARSER_REGISTRY,
                &MarkerFormat::default(),
                &Host::default(),
            ) else {
                continue;
            };
            let s = &r.stats;
            let counted = s.entities_unchanged
                + s.entities_ours_only
                + s.entities_theirs_only
                + s.entities_both_changed_merged
                + s.entities_added_ours
                + s.entities_added_theirs
                + s.entities_deleted
                + s.entities_conflicted;
            assert_eq!(
                counted,
                r.audit.len(),
                "every entity is counted exactly once\nbase: {base:?}\nstats: {s:?}"
            );
            assert_eq!(
                s.has_conflicts(),
                !r.is_clean(),
                "the counter and the typed conflicts disagree\nbase: {base:?}"
            );
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Unsupported {
    
    NoGrammar,
    
    NoEntities,
    
    BothCreated,
    
    AmbiguousIdentity,
}

pub(crate) fn merge_file(
    base: &str,
    ours: &str,
    theirs: &str,
    file_path: &str,
    registry: &ParserRegistry,
    marker_format: &MarkerFormat,
    host: &Host,
) -> Result<MergeResult, Unsupported> { panic!("STUB: not implemented") }

fn summarize(resolved: &[resolve::Resolved]) -> MergeStats { panic!("STUB: not implemented") }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Encoding {
    crlf: bool,
    bom: bool,
}

impl Encoding {
    fn of(base: &str, ours: &str, theirs: &str) -> Self { panic!("STUB: not implemented") }
    fn apply(self, content: String) -> String { panic!("STUB: not implemented") }
}

fn raws(
    entities: &[sem_core::model::entity::SemanticEntity],
    regions: &[FileRegion],
) -> Vec<RawEntity> { panic!("STUB: not implemented") }
