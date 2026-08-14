
use serde::Serialize;

use crate::conflict::{ConflictKind, EntityConflict, MarkerFormat};
use crate::diagnose::{Hunk, SideAction, SideEdit};
use crate::merge::entity_merge_fmt;

const HUNK_LIMIT: usize = 12;

const SAMPLE: usize = 3;

const WIDTH: usize = 100;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Explanation {
    pub file: String,
    
    pub clean: bool,
    pub conflicts: Vec<ConflictExplanation>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ConflictExplanation {
    pub entity: String,
    pub entity_type: String,
    pub kind: &'static str,
    
    pub refused_by: &'static str,
    pub complexity: String,
    
    pub contested_hunks: usize,
    
    pub hunks: Vec<HunkExplanation>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repair: Option<Repair>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(tag = "repair", rename_all = "snake_case")]
pub enum Repair {
    
    Rename { from: String, to: String },
    
    DecideExistence { kept_by: &'static str },
    
    Reconcile { name: String },
}

impl std::fmt::Display for Repair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { panic!("STUB: not implemented") }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct HunkExplanation {
    
    pub base_start: usize,
    pub base_end: usize,
    pub ours: SideSummary,
    pub theirs: SideSummary,
    
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub collision: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct SideSummary {
    pub action: &'static str,
    pub removed_lines: usize,
    pub added_lines: usize,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<String>,
}

impl SideSummary {
    fn of(e: &SideEdit) -> Self { panic!("STUB: not implemented") }
}

fn clip(line: &str) -> String { panic!("STUB: not implemented") }

fn contested(h: &Hunk) -> bool { panic!("STUB: not implemented") }

fn repair_of(c: &EntityConflict) -> Option<Repair> { panic!("STUB: not implemented") }

pub fn explain(
    base: &str,
    ours: &str,
    theirs: &str,
    file_path: &str,
    host: &crate::host::Host,
) -> Explanation { panic!("STUB: not implemented") }

impl Explanation {
    
    pub fn render(&self) -> String { panic!("STUB: not implemented") }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASE: &str = "def a():\n    x = 1\n    return x\n\ndef b():\n    return 2\n";

    #[test]
    fn only_hunks_both_sides_wrote_in_are_counted() {
        
        let ours = "def a():\n    x = 11\n    return x\n\ndef b():\n    return 22\n";
        let theirs = "def a():\n    x = 33\n    return x\n\ndef b():\n    return 2\n";
        let e = explain(BASE, ours, theirs, "m.py", &crate::host::Host::default());
        assert!(!e.clean);
        assert_eq!(e.conflicts.len(), 1, "{e:#?}");
        let c = &e.conflicts[0];
        assert_eq!(c.entity, "a");
        assert_eq!(c.contested_hunks, 1, "{c:#?}");
        assert_eq!(c.hunks[0].collision, vec!["    x = 1".to_string()]);
    }

    #[test]
    fn a_clean_merge_explains_itself_in_a_sentence_and_never_an_empty_list() {
        let ours = "def a():\n    x = 11\n    return x\n\ndef b():\n    return 2\n";
        let theirs = "def a():\n    x = 1\n    return x\n\ndef b():\n    return 22\n";
        let e = explain(BASE, ours, theirs, "m.py", &crate::host::Host::default());
        assert!(e.clean, "{e:#?}");
        let r = e.render();
        assert!(r.contains("CLEAN"), "{r}");
        assert!(!r.trim().is_empty());
    }

    #[test]
    fn the_guard_named_here_is_the_guard_the_marker_prints() {
        let ours = "def a():\n    x = 11\n    return x\n\ndef b():\n    return 2\n";
        let theirs = "def a():\n    x = 33\n    return x\n\ndef b():\n    return 2\n";
        let e = explain(BASE, ours, theirs, "m.py", &crate::host::Host::default());
        let merged = entity_merge_fmt(
            BASE,
            ours,
            theirs,
            "m.py",
            &MarkerFormat::default(),
            &crate::host::Host::default(),
        );
        let guard = e.conflicts[0].refused_by;
        assert!(
            merged.content.contains(&format!("refused_by: {guard}")),
            "explain says {guard}, artifact says:\n{}",
            merged.content
        );
    }

    #[test]
    fn a_rename_carries_a_derivable_repair() {
        let base = "def fetch():\n    return 1\n";
        let ours = "def get():\n    return 1\n";
        let theirs = "def fetch():\n    return 99\n";
        let e = explain(base, ours, theirs, "m.py", &crate::host::Host::default());
        let repair = e.conflicts.iter().find_map(|c| c.repair.clone());
        assert!(
            matches!(&repair, Some(Repair::Rename { from, to }) if from == "fetch" && to == "get"),
            "{e:#?}"
        );
    }
}
