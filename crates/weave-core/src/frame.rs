
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Resolution {
    
    Ours,
    
    Theirs,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Zone {
    Frame,
    Ours,
    Base,
    Theirs,
}

fn zones(content: &str) -> impl Iterator<Item = (Zone, &str)> {
    let mut zone = Zone::Frame;
    content.lines().filter_map(move |line| {
        if line.starts_with("<<<<<<<") {
            zone = Zone::Ours;
            return None;
        }
        if zone != Zone::Frame && line.starts_with("|||||||") {
            zone = Zone::Base;
            return None;
        }
        if zone != Zone::Frame && line.starts_with("=======") {
            zone = Zone::Theirs;
            return None;
        }
        if line.starts_with(">>>>>>>") {
            zone = Zone::Frame;
            return None;
        }
        if zone == Zone::Ours && crate::conflict::refusal_body(line).is_some() {
            return None;
        }
        if crate::conflict::is_teach_line(line) {
            return None;
        }
        Some((zone, line))
    })
}

pub fn marker_line_count(content: &str) -> usize { panic!("STUB: not implemented") }

pub fn resolve_conflicts(content: &str, take: Resolution) -> String { panic!("STUB: not implemented") }

pub fn frame(content: &str) -> Vec<&str> { panic!("STUB: not implemented") }

pub(crate) fn is_conflicted(content: &str) -> bool { panic!("STUB: not implemented") }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrameDuplicate {
    
    pub line: String,
    
    pub found: usize,
    
    pub allowed: usize,
}

fn is_structural(line: &str) -> bool { panic!("STUB: not implemented") }

fn line_counts(lines: impl IntoIterator<Item = impl AsRef<str>>) -> HashMap<String, usize> { panic!("STUB: not implemented") }

pub fn frame_duplicates(base: &str, ours: &str, theirs: &str, merged: &str) -> Vec<FrameDuplicate> { panic!("STUB: not implemented") }

#[cfg(test)]
mod tests {
    use super::*;

    const CONFLICTED: &str = "\
before()
<<<<<<< ours — function `f` (S, confidence: low)
// refused_by: statement_fold · collision: `body()`
ours_body()
=======
theirs_body()
>>>>>>> theirs — function `f`
after()
";

    #[test]
    fn the_two_resolutions_keep_the_frame_and_one_claim_each() {
        assert_eq!(
            resolve_conflicts(CONFLICTED, Resolution::Ours),
            "before()\nours_body()\nafter()\n"
        );
        assert_eq!(
            resolve_conflicts(CONFLICTED, Resolution::Theirs),
            "before()\ntheirs_body()\nafter()\n"
        );
    }

    #[test]
    fn the_refusal_line_is_marker_furniture_and_belongs_to_neither_side() {
        assert!(!resolve_conflicts(CONFLICTED, Resolution::Ours).contains("refused_by:"));
        assert!(!frame(CONFLICTED).iter().any(|l| l.contains("refused_by:")));
    }

    #[test]
    fn the_frame_is_what_sits_outside_every_box() {
        assert_eq!(frame(CONFLICTED), vec!["before()", "after()"]);
    }

    #[test]
    fn the_base_section_of_a_diff3_box_is_nobody_s_claim() {
        let diff3 = "<<<<<<< ours\na()\n||||||| base\nb()\n=======\nc()\n>>>>>>> theirs\n";
        assert_eq!(resolve_conflicts(diff3, Resolution::Ours), "a()\n");
        assert_eq!(resolve_conflicts(diff3, Resolution::Theirs), "c()\n");
        assert!(frame(diff3).is_empty());
    }

    #[test]
    fn a_frame_line_no_version_wrote_twice_is_reported_once() {
        let merged = "def alpha():\n    return 1\ndef alpha():\n    return 1\n";
        let one = "def alpha():\n    return 1\n";
        let dups = frame_duplicates(one, one, one, merged);
        assert_eq!(dups.len(), 2, "both body lines are duplicated: {dups:?}");
        assert_eq!(dups[1].line, "def alpha():");
        assert_eq!((dups[1].found, dups[1].allowed), (2, 1));
    }

    #[test]
    fn a_line_one_version_really_does_write_twice_is_not_a_duplicate() {
        let twice = "log_it()\nlog_it()\n";
        assert!(frame_duplicates("log_it()\n", twice, "log_it()\n", twice).is_empty());
    }

    #[test]
    fn two_sides_independently_adding_one_copy_may_both_be_kept() {
        
        let both = "import a\nimport a\n";
        assert!(frame_duplicates("", "import a\n", "import a\n", both).is_empty());
        
        assert_eq!(
            frame_duplicates(
                "",
                "import a\n",
                "import a\n",
                "import a\nimport a\nimport a\n"
            )
            .len(),
            1
        );
    }
}
