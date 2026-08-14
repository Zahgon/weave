
use std::collections::{BTreeSet, HashSet};

pub(crate) fn is_ident_char(b: u8) -> bool { panic!("STUB: not implemented") }

const MODIFIERS: [&str; 11] = [
    "export ",
    "public ",
    "private ",
    "protected ",
    "static ",
    "pub ",
    "async ",
    "default ",
    "abstract ",
    "final ",
    "override ",
];

const DEFINERS: [&str; 11] = [
    "def ",
    "function ",
    "fn ",
    "class ",
    "func ",
    "interface ",
    "struct ",
    "trait ",
    "impl ",
    "type ",
    "enum ",
];

pub(crate) fn is_definition_line(line: &str, name: &str) -> bool { panic!("STUB: not implemented") }

pub fn has_definition(content: &str, name: &str) -> bool { panic!("STUB: not implemented") }

pub fn has_call_reference(content: &str, name: &str) -> bool { panic!("STUB: not implemented") }

pub(crate) fn called_names(content: &str) -> HashSet<&str> { panic!("STUB: not implemented") }

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Footprint {
    
    pub defines: BTreeSet<String>,
    
    pub uses: BTreeSet<String>,
}

pub(crate) fn declared_name(text: &str) -> Option<String> { panic!("STUB: not implemented") }

pub(crate) fn defined_name(line: &str) -> Option<String> { panic!("STUB: not implemented") }

pub(crate) fn is_trivia_line(line: &str) -> bool { panic!("STUB: not implemented") }

pub(crate) fn word_tokens(s: &str) -> Vec<&str> { panic!("STUB: not implemented") }

pub(crate) fn footprint(text: &str) -> Option<Footprint> { panic!("STUB: not implemented") }

pub(crate) fn footprints_disjoint(a: &Footprint, b: &Footprint) -> bool { panic!("STUB: not implemented") }

pub fn replace_at_word_boundaries(content: &str, needle: &str, replacement: &str) -> String { panic!("STUB: not implemented") }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_definition_line_is_not_a_call_site() {
        assert!(is_definition_line("def fetch_user(id):", "fetch_user"));
        assert!(is_definition_line(
            "    pub async fn fetch_user(id: u32) {",
            "fetch_user"
        ));
        assert!(!has_call_reference(
            "def fetch_user(id):\n    pass\n",
            "fetch_user"
        ));
    }

    #[test]
    fn a_call_needs_a_word_boundary_and_an_open_paren() {
        assert!(has_call_reference("x = fetch_user(1)\n", "fetch_user"));
        
        assert!(!has_call_reference("x = fetch_user_id(1)\n", "fetch_user"));
        
        assert!(!has_call_reference("x = db.fetch_user(1)\n", "fetch_user"));
        
        assert!(!has_call_reference("x = 'fetch_user'\n", "fetch_user"));
    }

    #[test]
    fn definers_survive_modifier_prefixes() {
        assert!(is_definition_line("export default class Foo {", "Foo"));
        assert!(!is_definition_line("export default class FooBar {", "Foo"));
    }

    #[test]
    fn a_dollar_prefixed_identifier_is_its_own_name() {
        assert!(!has_call_reference("x = $fetch(1)\n", "fetch"));
        assert!(has_call_reference("x = $fetch(1)\n", "$fetch"));
        assert_eq!(
            replace_at_word_boundaries("$fetch(1); fetch(2)", "fetch", "grab"),
            "$fetch(1); grab(2)"
        );
    }

    #[test]
    fn a_footprint_exists_only_where_the_effect_is_visible() {
        
        let fp = footprint("timeout = settings.timeout\n").expect("a declaration has one");
        assert!(fp.defines.contains("timeout"));
        assert!(fp.uses.contains("settings"));
        assert!(!fp.uses.contains("timeout"), "a name is not its own use");

        let fp = footprint("def scale(v):\n    return v * factor\n").expect("a def has one");
        assert!(fp.defines.contains("scale"));
        assert!(fp.uses.contains("factor"));

        for opaque in [
            "flush()\n",                                
            "return total\n",                           
            "if ready:\n    go()\n",                    
            "total += 1\n",                             
            "self.cache[key] = v\n",                    
            "@register\ndef scale(v):\n    return v\n", 
            "",                                         
        ] {
            assert!(
                footprint(opaque).is_none(),
                "{opaque:?} should have no read/write set"
            );
        }
    }

    #[test]
    fn disjoint_composition_is_symmetric_and_fails_on_either_edge() {
        let a = footprint("def scale(v):\n    return v * 2\n").unwrap();
        let b = footprint("def offset(v):\n    return v + 3\n").unwrap();
        assert!(footprints_disjoint(&a, &b));
        assert!(footprints_disjoint(&b, &a));

        let uses_a = footprint("def offset(v):\n    return scale(v) + 3\n").unwrap();
        assert!(!footprints_disjoint(&a, &uses_a));
        assert!(!footprints_disjoint(&uses_a, &a));

        let rival = footprint("def scale(v):\n    return v + 3\n").unwrap();
        assert!(!footprints_disjoint(&a, &rival));
    }

    #[test]
    fn the_call_index_and_the_call_predicate_agree() {
        let text = "x = alpha(1)\ny = db.beta(2)\nz = 'gamma('\ndef delta():\n    delta_helper()\n";
        let indexed = called_names(text);
        for name in ["alpha", "beta", "gamma", "delta", "delta_helper"] {
            assert_eq!(
                indexed.contains(name),
                has_call_reference(text, name),
                "index and predicate disagree about {name}"
            );
        }
    }
}
