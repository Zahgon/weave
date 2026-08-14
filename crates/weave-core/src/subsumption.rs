
use std::ops::Range;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Superset {
    Ours,
    Theirs,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Edit {
    old: Range<usize>,
    ins: Vec<String>,
}

fn key_lines(text: &str) -> Vec<&str> { panic!("STUB: not implemented") }

fn edits(base: &str, side: &str) -> Vec<Edit> { panic!("STUB: not implemented") }

fn is_subsequence(small: &[String], big: &[String]) -> bool { panic!("STUB: not implemented") }

fn carries(inner: &Edit, outer: &Edit) -> bool { panic!("STUB: not implemented") }

pub(crate) fn subsuming_side(base: &str, ours: &str, theirs: &str) -> Option<Superset> { panic!("STUB: not implemented") }

#[cfg(test)]
mod tests {
    use super::*;

    fn ours_side(b: &str, o: &str, t: &str) -> Option<Superset> {
        subsuming_side(b, o, t)
    }

    #[test]
    fn theirs_rewrote_the_region_ours_touched_and_kept_ours_line() {
        
        let base = "fn f() {\n    work();\n}\n";
        let ours = "fn f() {\n    check();\n    work();\n}\n";
        let theirs = "fn f() {\n    check();\n    audit();\n    work();\n}\n";
        assert_eq!(ours_side(base, ours, theirs), Some(Superset::Theirs));
    }

    #[test]
    fn the_same_line_added_at_two_places_is_not_subsumption() {
        
        let base = "class C {\n  void a() {}\n\n  void b() {}\n}\n";
        let ours = "class C {\n  @Test\n  void a() {}\n\n  void b() {}\n}\n";
        let theirs = "class C {\n  void a() {}\n\n  @Test\n  void b() {}\n}\n";
        assert_eq!(ours_side(base, ours, theirs), None);
    }

    #[test]
    fn a_version_bump_is_not_subsumption() {
        let base = "<version>4.0.0-beta-1-SNAPSHOT</version>\n";
        let ours = "<version>4.0.0-beta-2-SNAPSHOT</version>\n";
        let theirs = "<version>4.0.0-beta-1</version>\n";
        assert_eq!(ours_side(base, ours, theirs), None);
    }

    #[test]
    fn a_rewrite_that_spans_ours_lines_without_keeping_them_is_not_subsumption() {
        let base = "a\nb\nc\n";
        let ours = "a\nB\nc\n";
        let theirs = "a\nX\nY\nc\n";
        assert_eq!(ours_side(base, ours, theirs), None);
    }

    #[test]
    fn identical_edits_are_left_to_the_pipeline() {
        let base = "a\nb\n";
        let ours = "a\nb\nc\n";
        let theirs = "a\nb\nc\n";
        assert_eq!(ours_side(base, ours, theirs), None);
    }

    #[test]
    fn ours_can_be_the_superset() {
        let base = "x\ny\nz\n";
        let ours = "x\np\nq\ny\nz\n";
        let theirs = "x\np\ny\nz\n";
        assert_eq!(ours_side(base, ours, theirs), Some(Superset::Ours));
    }

    #[test]
    fn a_blank_line_only_edit_never_fires_the_rule() {
        
        let base = "a\nb\n";
        let ours = "a\n\nb\n";
        let theirs = "a\nb\nc\n";
        assert_eq!(ours_side(base, ours, theirs), None);
    }

    #[test]
    fn deletions_subsume_too() {
        let base = "a\nb\nc\nd\n";
        let ours = "a\nc\nd\n"; 
        let theirs = "a\nd\n"; 
        assert_eq!(ours_side(base, ours, theirs), Some(Superset::Theirs));
    }

    #[test]
    fn a_deletion_against_an_edit_of_the_same_lines_stays_a_conflict() {
        
        let base = "a\nb\nc\n";
        let ours = "a\nc\n";
        let theirs = "a\nb2\nc\n";
        assert_eq!(ours_side(base, ours, theirs), None);
    }

    #[test]
    fn a_line_ending_conversion_blocks_the_rule() {
        let base = "a\nb\n";
        let ours = "a\r\nb\r\nc\r\n";
        let theirs = "a\nb\n";
        assert_eq!(ours_side(base, ours, theirs), None);
    }

    #[test]
    fn two_unrelated_edits_are_not_subsumption() {
        let base = "a\nb\nc\nd\n";
        let ours = "a\nB\nc\nd\n";
        let theirs = "a\nb\nc\nD\n";
        assert_eq!(ours_side(base, ours, theirs), None);
    }
}
