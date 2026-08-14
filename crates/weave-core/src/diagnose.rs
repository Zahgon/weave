
use std::collections::BTreeSet;

use crate::binding::word_tokens;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SideAction {
    
    Untouched,
    
    Added,
    
    Removed,
    
    Modified,
}

impl SideAction {
    
    pub fn wire(&self) -> &'static str { panic!("STUB: not implemented") }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SideEdit {
    pub action: SideAction,
    
    pub removed: Vec<String>,
    
    pub added: Vec<String>,
    
    pub tokens_added: Vec<String>,
    
    pub tokens_removed: Vec<String>,
}

impl SideEdit {
    fn untouched() -> Self { panic!("STUB: not implemented") }

    fn of(removed: Vec<String>, added: Vec<String>) -> Self { panic!("STUB: not implemented") }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hunk {
    
    pub base_start: usize,
    pub base_end: usize,
    pub ours: SideEdit,
    pub theirs: SideEdit,
    
    pub collision: Vec<String>,
}

struct Edit {
    start: usize,
    len: usize,
    removed: Vec<String>,
    added: Vec<String>,
}

fn edits(base: &str, side: &str) -> Vec<Edit> { panic!("STUB: not implemented") }

fn touches(a: &Edit, b: &Edit) -> bool { panic!("STUB: not implemented") }

pub fn diagnose(base: &str, ours: &str, theirs: &str) -> Vec<Hunk> { panic!("STUB: not implemented") }

#[cfg(test)]
mod tests {
    use super::*;

    const BASE: &str = "fn f() {\n    a();\n    b();\n    c();\n}\n";

    #[test]
    fn two_sides_rewriting_the_same_line_collide_in_one_hunk() {
        let ours = "fn f() {\n    a();\n    b_ours();\n    c();\n}\n";
        let theirs = "fn f() {\n    a();\n    b_theirs();\n    c();\n}\n";
        let h = diagnose(BASE, ours, theirs);
        assert_eq!(h.len(), 1, "{h:#?}");
        assert_eq!((h[0].base_start, h[0].base_end), (3, 3));
        assert_eq!(h[0].ours.action, SideAction::Modified);
        assert_eq!(h[0].theirs.action, SideAction::Modified);
        assert_eq!(h[0].collision, vec!["    b();".to_string()]);
        assert_eq!(h[0].ours.tokens_added, vec!["b_ours".to_string()]);
        assert_eq!(h[0].theirs.tokens_added, vec!["b_theirs".to_string()]);
        assert_eq!(h[0].ours.tokens_removed, vec!["b".to_string()]);
    }

    #[test]
    fn edits_far_apart_are_two_hunks_and_neither_collides() {
        let ours = "fn f() {\n    a_ours();\n    b();\n    c();\n}\n";
        let theirs = "fn f() {\n    a();\n    b();\n    c_theirs();\n}\n";
        let h = diagnose(BASE, ours, theirs);
        assert_eq!(h.len(), 2, "{h:#?}");
        assert_eq!(h[0].ours.action, SideAction::Modified);
        assert_eq!(h[0].theirs.action, SideAction::Untouched);
        assert!(h[0].collision.is_empty());
        assert_eq!(h[1].ours.action, SideAction::Untouched);
        assert_eq!(h[1].theirs.action, SideAction::Modified);
        assert!(h[1].collision.is_empty());
    }

    #[test]
    fn a_deletion_and_an_insertion_are_named_as_what_they_are() {
        let ours = "fn f() {\n    a();\n    c();\n}\n"; 
        let theirs = "fn f() {\n    a();\n    b();\n    new();\n    c();\n}\n"; 
        let h = diagnose(BASE, ours, theirs);
        assert_eq!(h.len(), 1, "adjacent edits are one hunk: {h:#?}");
        assert_eq!(h[0].ours.action, SideAction::Removed);
        assert_eq!(h[0].theirs.action, SideAction::Added);
        assert!(h[0].collision.is_empty(), "theirs deleted nothing");
        assert_eq!(h[0].theirs.tokens_added, vec!["new".to_string()]);
    }

    #[test]
    fn an_unedited_body_has_no_hunks() {
        assert!(diagnose(BASE, BASE, BASE).is_empty());
    }

    #[test]
    fn hunks_come_out_in_base_order() {
        let ours = "fn f() {\n    a_o();\n    b();\n    c_o();\n}\n";
        let theirs = "fn f() {\n    a_t();\n    b();\n    c_t();\n}\n";
        let h = diagnose(BASE, ours, theirs);
        assert!(
            h.windows(2).all(|w| w[0].base_start <= w[1].base_start),
            "{h:#?}"
        );
        assert_eq!(h, diagnose(BASE, ours, theirs));
    }
}
