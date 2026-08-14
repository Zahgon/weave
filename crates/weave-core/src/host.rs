
use std::io::Write;
use std::process::Command;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineMergeStyle {
    
    Plain,
    
    Labelled,
}

#[derive(Debug, Clone)]
pub struct LineMerged {
    
    pub content: String,
    
    pub clean: bool,
}

pub type LineMerge = fn(LineMergeStyle, &str, &str, &str) -> Option<LineMerged>;

#[derive(Debug, Clone, Copy)]
pub struct Host {
    
    pub max_duplicates: usize,

    pub line_merge: Option<LineMerge>,
}

impl Default for Host {
    
    fn default() -> Self { panic!("STUB: not implemented") }
}

pub fn git_line_merge(
    style: LineMergeStyle,
    base: &str,
    ours: &str,
    theirs: &str,
) -> Option<LineMerged> { panic!("STUB: not implemented") }

struct Scratch {
    base: tempfile::NamedTempFile,
    ours: tempfile::NamedTempFile,
    theirs: tempfile::NamedTempFile,
}

fn scratch_triple(base: &str, ours: &str, theirs: &str) -> Option<Scratch> { panic!("STUB: not implemented") }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_host_grants_nothing() {
        let host = Host::default();
        assert!(host.line_merge.is_none());
        assert_eq!(host.max_duplicates, 10);
    }

    #[test]
    fn a_granted_line_merge_answers_and_says_whether_it_was_clean() {
        let base = "a\nb\nc\nd\ne\nf\ng\n";
        let ours = "A\nb\nc\nd\ne\nf\ng\n";
        let theirs = "a\nb\nc\nd\ne\nf\nG\n";
        let Some(merged) = git_line_merge(LineMergeStyle::Labelled, base, ours, theirs) else {
            return; 
        };
        assert!(merged.clean);
        assert_eq!(merged.content, "A\nb\nc\nd\ne\nf\nG\n");

        let both = "X\nb\nc\nd\ne\nf\ng\n";
        let conflicted = git_line_merge(LineMergeStyle::Labelled, base, ours, both)
            .expect("git answered once already");
        assert!(!conflicted.clean);
        assert!(conflicted.content.contains("<<<<<<< ours"));
        
        assert!(git_line_merge(LineMergeStyle::Plain, base, ours, both).is_none());
    }
}
