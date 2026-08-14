
use crate::binding::is_trivia_line;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct TokenClaim(usize);

impl TokenClaim {
    fn index(&self) -> usize { panic!("STUB: not implemented") }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    
    Word,
    
    Number,
    
    Str,
    
    Comment,
    
    Op,
}

#[derive(Debug, Clone, Copy)]
struct Tok {
    gap: usize,
    text: usize,
    end: usize,
    kind: Kind,
}

const OPERATORS: &[&str] = &[
    ">>>=", "<<=", ">>=", ">>>", "...", "&&=", "||=", "**=", "//=", "===", "!==", "??=", "==",
    "!=", "<=", ">=", "&&", "||", "->", "=>", "::", "++", "--", "+=", "-=", "*=", "/=", "%=", "&=",
    "|=", "^=", "<<", ">>", "**", "??", "?.", "..", "!!", ":=", "<-", "|>",
];

const MAX_TOKENS: usize = 256;

fn tokenize(src: &str) -> Option<Vec<Tok>> { panic!("STUB: not implemented") }

fn is_lifetime(b: &[u8], i: usize, rustish: bool) -> bool { panic!("STUB: not implemented") }

fn close_on_line(b: &[u8], start: usize, quote: u8) -> Option<usize> { panic!("STUB: not implemented") }

fn tok_text<'a>(src: &'a str, t: &Tok) -> &'a str { panic!("STUB: not implemented") }

fn key<'a>(src: &'a str, t: &Tok) -> (Kind, &'a str) { panic!("STUB: not implemented") }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Hunk {
    b0: usize,
    b1: usize,
    s0: usize,
    s1: usize,
}

const MAX_LCS: usize = 60_000;

fn lcs_pairs(a: &[(Kind, &str)], b: &[(Kind, &str)]) -> Option<Vec<(usize, usize)>> { panic!("STUB: not implemented") }

fn hunks(bk: &[(Kind, &str)], sk: &[(Kind, &str)]) -> Option<Vec<Hunk>> { panic!("STUB: not implemented") }

fn groups(src: &str, toks: &[Tok]) -> Vec<Option<usize>> { panic!("STUB: not implemented") }

fn separated(o: &Hunk, t: &Hunk) -> bool { panic!("STUB: not implemented") }

fn top_level_commas(src: &str, toks: &[Tok], lo: usize, hi: usize) -> usize { panic!("STUB: not implemented") }

fn changes_arity(base: &str, btoks: &[Tok], side: &str, stoks: &[Tok], h: &Hunk) -> bool { panic!("STUB: not implemented") }

fn touches_trivia(src: &str, toks: &[Tok], lo: usize, hi: usize) -> bool { panic!("STUB: not implemented") }

fn net_depth(src: &str, toks: &[Tok]) -> i32 { panic!("STUB: not implemented") }

fn gaps_faithful(base: &str, btoks: &[Tok], side: &str, stoks: &[Tok], hs: &[Hunk]) -> bool { panic!("STUB: not implemented") }

#[derive(Debug, Clone)]
pub(crate) struct Composition {
    pub text: String,
    
    pub composed_lines: Vec<String>,
}

pub(crate) fn expression_merge(base: &str, ours: &str, theirs: &str) -> Option<Composition> { panic!("STUB: not implemented") }

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLES: &[&str] = &[
        
        "    return (var.type != null ? var.type.cast(v, ctx, input) : v).iter(ctx);\n",
        "    private static final String VERSION = \"4.0.0-beta-1-SNAPSHOT\";\n",
        "    Map<String, List<Integer>> m = new HashMap<>(); // trailing\n",
        
        "    fn h<'a>(s: &'a str) -> &'a str { s.trim() }\n",
        "    let x = a >>= b; /* block */ let y = 0x1f_u8;\n",
        "    #[derive(Debug, Clone)]\n",
        
        "    s = \"\"\"triple\n    quoted\"\"\"  # comment\n",
        "    d = {'a': 1, 'b': 2}\n",
        
        "  const q = `tpl ${x}`, r = a?.b ?? c;\n",
        "  export default function f(a: number = 1e-3): void {}\n",
        
        "\ttotal := 0\n\tfor _, i := range x {\n\t\ttotal += i\n\t}\n",
        
        "    print(\"he said \\\"hi\\\"\", 'x', \"héllo wörld\")\n",
    ];

    #[test]
    fn a_tokenization_tiles_the_text_it_came_from() {
        for src in SAMPLES {
            let toks = tokenize(src).unwrap_or_else(|| panic!("tokenizes: {src:?}"));
            let mut rebuilt = String::new();
            for t in &toks {
                rebuilt.push_str(&src[t.gap..t.end]);
            }
            if let Some(last) = toks.last() {
                rebuilt.push_str(&src[last.end..]);
            }
            assert_eq!(rebuilt, *src, "tokenization lost or duplicated text");
        }
    }

    #[test]
    fn literals_and_comments_are_one_token_each() {
        let src = "x = \"a b c\"; // one comment\n";
        let toks = tokenize(src).unwrap();
        let strs: Vec<&str> = toks
            .iter()
            .filter(|t| t.kind == Kind::Str)
            .map(|t| tok_text(src, t))
            .collect();
        assert_eq!(strs, vec!["\"a b c\""]);
        let cs: Vec<&str> = toks
            .iter()
            .filter(|t| t.kind == Kind::Comment)
            .map(|t| tok_text(src, t))
            .collect();
        assert_eq!(cs, vec!["// one comment"]);
    }

    #[test]
    fn a_rust_lifetime_is_not_a_string() {
        let src = "fn h<'a>(s: &'a str) {}\n";
        let toks = tokenize(src).unwrap();
        assert!(
            toks.iter().all(|t| t.kind != Kind::Str),
            "a lifetime was read as a literal"
        );
    }

    #[test]
    fn an_unterminated_literal_is_refused() {
        assert!(tokenize("x = \"open\n").is_none());
        assert!(tokenize("x = 1; /* open\n").is_none());
    }

    #[test]
    fn disjoint_token_edits_to_one_statement_compose() {
        let b = "    return (var.type != null ? var.type.cast(v, ctx, input) : v).iter(ctx);\n";
        let o = "    return (var.type != null ? var.type.cast(v, ctx, input) : v).iter();\n";
        let t = "    return (ret != null ? ret.cast(v, ctx, input) : v).iter(ctx);\n";
        let c = expression_merge(b, o, t).expect("composes");
        assert_eq!(
            c.text,
            "    return (ret != null ? ret.cast(v, ctx, input) : v).iter();\n"
        );
    }

    #[test]
    fn a_string_literal_edited_by_both_sides_is_not_composable() {
        let b = "\tprivate static final String VERSION = \"4.0.0-beta-1-SNAPSHOT\";\n";
        let o = "\tprivate static final String VERSION = \"4.0.0-beta-2-SNAPSHOT\";\n";
        let t = "\tprivate static final String VERSION = \"4.0.0-beta-1\";\n";
        assert!(
            expression_merge(b, o, t).is_none(),
            "composed a version neither side released"
        );
    }

    #[test]
    fn an_insertion_interior_to_a_replacement_is_not_separated() {
        let b = "\t\tString timestamp = DATE_FORMAT_14_FORMATTER.format(date);\n";
        let o = "\t\tString timestamp = DATE_FORMAT_14_FORMATTER.get().format(date);\n";
        let t = "\t\tString timestamp = ArchiveUtils.get14DigitDate(date);\n";
        assert!(
            expression_merge(b, o, t).is_none(),
            "composed ArchiveUtils.get().get14DigitDate"
        );
    }

    #[test]
    fn two_rewordings_of_one_comment_line_are_not_composable() {
        let b = "   * @throws java.lang.IllegalArgumentException if the String value is not a legal Base64 encoded value\n";
        let o = "   * @throws java.lang.IllegalArgumentException if the value is not a legal Base64 encoded String\n";
        let t = "   * @throws java.lang.IllegalArgumentException if the String is not a legal Base64 encoded value\n";
        assert!(
            expression_merge(b, o, t).is_none(),
            "composed a sentence neither side wrote"
        );
    }

    #[test]
    fn a_field_rename_beside_an_unrelated_type_change_composes() {
        let b = "\tprivate @Mock Event<ControllerMethod> event;\n";
        let o = "\tprivate @Mock Event<ControllerMethod> controllerMethodEvent;\n";
        let t = "\tprivate @Mock Event<ControllerFound> event;\n";
        let c = expression_merge(b, o, t).expect("composes");
        assert_eq!(
            c.text,
            "\tprivate @Mock Event<ControllerFound> controllerMethodEvent;\n"
        );
    }

    #[test]
    fn a_log_level_change_beside_a_message_fix_composes() {
        let b = "            getLog().info( \"android.device parameter not set\" );\n";
        let o = "            getLog().debug( \"android.device parameter not set\" );\n";
        let t = "            getLog().info( \"android.devices parameter not set\" );\n";
        let c = expression_merge(b, o, t).expect("composes");
        assert_eq!(
            c.text,
            "            getLog().debug( \"android.devices parameter not set\" );\n"
        );
    }

    #[test]
    fn two_arguments_added_to_one_call_are_not_composable() {
        let b = "    f(a, b);\n";
        let o = "    f(x, a, b);\n";
        let t = "    f(a, b, d);\n";
        assert!(
            expression_merge(b, o, t).is_none(),
            "composed a four-argument call nobody wrote"
        );
    }

    #[test]
    fn two_arguments_rewritten_in_one_call_compose() {
        let b = "    f(alpha, beta);\n";
        let o = "    f(gamma, beta);\n";
        let t = "    f(alpha, delta);\n";
        let c = expression_merge(b, o, t).expect("composes");
        assert_eq!(c.text, "    f(gamma, delta);\n");
    }

    #[test]
    fn adjacent_substitutions_compose() {
        let b = "    let x = a + b;\n";
        let o = "    let x = c + b;\n";
        let t = "    let x = a - b;\n";
        let c = expression_merge(b, o, t).expect("composes");
        assert_eq!(c.text, "    let x = c - b;\n");
    }

    #[test]
    fn an_insertion_anchored_to_a_deleted_token_is_refused() {
        let b = "    let x = alpha.beta(q);\n";
        let o = "    let x = alpha.mid().beta(q);\n";
        let t = "    let x = gamma.delta(q);\n";
        assert!(expression_merge(b, o, t).is_none());
    }

    #[test]
    fn a_reindent_beside_an_edit_is_refused() {
        let b = "\t    player.equip(new Item(x));\n";
        let o = "\t    player.equip(itemRepo.get(x));\n";
        let t = "            player.equip(new Item(x));\n";
        assert!(expression_merge(b, o, t).is_none());
    }

    #[test]
    fn an_insertion_does_not_double_the_gap_at_its_seam() {
        let b = "    FUNCAST.thrw(ii, Type.BLN, str);\n";
        let o = "    FUNCAST.thrw(ii, AtomType.BLN, str);\n";
        let t = "    throw FUNCAST.thrw(ii, Type.BLN, str);\n";
        let c = expression_merge(b, o, t).expect("composes");
        assert_eq!(c.text, "    throw FUNCAST.thrw(ii, AtomType.BLN, str);\n");
    }

    #[test]
    fn the_expression_merge_is_a_function_of_its_inputs() {
        let b = "    return (var.type != null ? var.type.cast(v, ctx, input) : v).iter(ctx);\n";
        let o = "    return (var.type != null ? var.type.cast(v, ctx, input) : v).iter();\n";
        let t = "    return (ret != null ? ret.cast(v, ctx, input) : v).iter(ctx);\n";
        let first = expression_merge(b, o, t).map(|c| c.text);
        for _ in 0..24 {
            assert_eq!(first, expression_merge(b, o, t).map(|c| c.text));
        }
        
        let swapped = expression_merge(b, t, o).map(|c| c.text);
        assert_eq!(first, swapped, "composition depends on which side is ours");
    }

    #[test]
    fn every_composed_token_comes_from_some_side() {
        let b = "    return f(a, b, c);\n";
        let o = "    return f(a2, b, c);\n";
        let t = "    return f(a, b, c2);\n";
        let c = expression_merge(b, o, t).expect("composes");
        let toks = tokenize(&c.text).unwrap();
        for t2 in &toks {
            let s = tok_text(&c.text, t2);
            assert!(
                b.contains(s) || o.contains(s) || t.contains(s),
                "token {s:?} came from nowhere"
            );
        }
    }
}
