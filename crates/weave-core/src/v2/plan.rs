
use std::collections::{HashMap, HashSet};

use super::resolve::Resolved;
use super::types::*;

#[derive(Debug)]
pub(crate) struct PlannedItem {
    pub anchor: Anchor,
    
    pub triple: usize,
}

#[derive(Debug)]
pub(crate) struct Placement {
    pub items: Vec<PlannedItem>,
    pub order_conflict: Option<OrderConflict>,
}

fn order_preserving_union(ours: &[usize], theirs: &[usize]) -> (Vec<usize>, bool) { panic!("STUB: not implemented") }

fn gap_order(
    arena: &Arena,
    triples: &[Triple],
    staged: &[(usize, Key, u32, u8)],
) -> HashMap<usize, u32> { panic!("STUB: not implemented") }

pub(crate) fn plan(
    arena: &Arena,
    triples: &[Triple],
    resolved: &[Resolved],
    order: EntityOrder,
) -> Placement { panic!("STUB: not implemented") }

#[cfg(test)]
mod tests {
    use super::super::match_phase::{build_arena, match_phase, RawEntity};
    use super::*;
    use crate::conflict::MarkerFormat;
    use crate::v2::classify::classify;
    use crate::v2::resolve::{resolve, ResolveCtx};

    fn raw(name: &str) -> RawEntity {
        RawEntity {
            name: name.to_string(),
            entity_type: "function".into(),
            parent: None,
            content: format!("def {name}():\n    return \"{name}\"\n"),
            src_id: format!("f::{name}"),
        }
    }
    fn raws(names: &[&str]) -> Vec<RawEntity> {
        names.iter().map(|n| raw(n)).collect()
    }

    fn order(b: &[&str], o: &[&str], t: &[&str]) -> Vec<String> {
        placement(b, o, t, EntityOrder::Unordered).0
    }

    fn placement(
        b: &[&str],
        o: &[&str],
        t: &[&str],
        order: EntityOrder,
    ) -> (Vec<String>, Option<OrderConflict>) {
        let (arena, claims) = build_arena(raws(b), raws(o), raws(t));
        let m = match_phase(arena, claims);
        let cells: Vec<_> = m.triples.iter().map(|tr| classify(&m.arena, tr)).collect();
        let fmt = MarkerFormat::default();
        let ctx = ResolveCtx {
            marker_format: &fmt,
            indent_sensitive: true,
            decorators_compose: true,
            base_all: &[],
            ours_all: &[],
            theirs_all: &[],
            host: &crate::host::Host::default(),
        };
        let resolved: Vec<_> = m
            .triples
            .iter()
            .zip(cells.iter())
            .map(|(tr, c)| resolve(&m.arena, tr, *c, &ctx))
            .collect();
        let placed = plan(&m.arena, &m.triples, &resolved, order);
        (
            placed
                .items
                .iter()
                .map(|p| p.anchor.key.name.clone())
                .collect(),
            placed.order_conflict,
        )
    }

    fn raw_typed(name: &str, ty: &str) -> RawEntity {
        RawEntity {
            name: name.to_string(),
            entity_type: ty.into(),
            parent: None,
            content: format!("{ty} {name}\n"),
            src_id: format!("{ty}::{name}"),
        }
    }

    fn order_typed(b: &[(&str, &str)], o: &[(&str, &str)], t: &[(&str, &str)]) -> Vec<String> {
        let mk = |v: &[(&str, &str)]| v.iter().map(|(n, ty)| raw_typed(n, ty)).collect();
        let (arena, claims) = build_arena(mk(b), mk(o), mk(t));
        let m = match_phase(arena, claims);
        let cells: Vec<_> = m.triples.iter().map(|tr| classify(&m.arena, tr)).collect();
        let fmt = MarkerFormat::default();
        let ctx = ResolveCtx {
            marker_format: &fmt,
            indent_sensitive: false,
            decorators_compose: true,
            base_all: &[],
            ours_all: &[],
            theirs_all: &[],
            host: &crate::host::Host::default(),
        };
        let resolved: Vec<_> = m
            .triples
            .iter()
            .zip(cells.iter())
            .map(|(tr, c)| resolve(&m.arena, tr, *c, &ctx))
            .collect();
        plan(&m.arena, &m.triples, &resolved, EntityOrder::Unordered)
            .items
            .iter()
            .map(|p| p.anchor.key.name.clone())
            .collect()
    }

    #[test]
    fn two_additions_one_side_wrote_together_keep_that_side_s_order() {
        
        let out = order_typed(
            &[("head", "function"), ("tail", "function")],
            &[
                ("head", "function"),
                ("WORKTREE", "variable"),
                ("isWorktree", "function"),
                ("tail", "function"),
            ],
            &[("head", "function"), ("tail", "function")],
        );
        assert_eq!(out, vec!["head", "WORKTREE", "isWorktree", "tail"]);
    }

    #[test]
    fn additions_no_version_orders_still_fall_back_to_the_key() {
        
        let fwd = order(&["a"], &["a", "zed"], &["a", "mid"]);
        let rev = order(&["a"], &["a", "mid"], &["a", "zed"]);
        assert_eq!(fwd, rev);
        assert_eq!(fwd, vec!["a", "mid", "zed"]);
    }

    #[test]
    fn placement_does_not_depend_on_which_side_is_ours() {
        
        let fwd = order(&["a", "b"], &["a", "b", "zed"], &["a", "b", "mid"]);
        let rev = order(&["a", "b"], &["a", "b", "mid"], &["a", "b", "zed"]);
        assert_eq!(fwd, rev, "placement is peer-relative");
        assert_eq!(fwd, vec!["a", "b", "mid", "zed"]);
    }

    #[test]
    fn an_addition_stays_next_to_the_entity_it_was_written_after() {
        let out = order(
            &["a", "b", "c"],
            &["a", "helper", "b", "c"],
            &["a", "b", "c"],
        );
        assert_eq!(out, vec!["a", "helper", "b", "c"]);
    }

    #[test]
    fn a_one_sided_reorder_is_honoured() {
        let out = order(&["a", "b", "c"], &["c", "b", "a"], &["a", "b", "c"]);
        assert_eq!(out, vec!["c", "b", "a"]);
    }

    #[test]
    fn a_one_sided_reorder_is_honoured_in_an_order_effectful_language_too() {
        
        let (out, conflict) = placement(
            &["a", "b", "c"],
            &["a", "c", "b"],
            &["a", "b", "c"],
            EntityOrder::Effectful,
        );
        assert_eq!(out, vec!["a", "c", "b"]);
        assert!(conflict.is_none());
    }

    #[test]
    fn two_compatible_reorders_are_unioned_where_order_is_semantics() {
        
        let (out, conflict) = placement(
            &["a", "b", "c", "d"],
            &["b", "c", "d", "a"],
            &["a", "c", "b", "d"],
            EntityOrder::Effectful,
        );
        assert!(conflict.is_some(), "b/c disagree, so this cell conflicts");
        
        let mut sorted = out.clone();
        sorted.sort();
        assert_eq!(sorted, vec!["a", "b", "c", "d"]);
    }

    #[test]
    fn contradictory_order_of_shared_entities_conflicts_where_order_is_semantics() {
        let (_, conflict) = placement(
            &["a", "b", "c"],
            &["a", "c", "b"],
            &["b", "a", "c"],
            EntityOrder::Effectful,
        );
        let c = conflict.expect("contradictory shared order must be reported");
        assert_eq!(c.ours, vec!["a", "c", "b"]);
        assert_eq!(c.theirs, vec!["b", "a", "c"]);
    }

    #[test]
    fn the_same_contradiction_is_deterministic_and_silent_where_order_is_not_semantics() {
        let (out, conflict) = placement(
            &["a", "b", "c"],
            &["a", "c", "b"],
            &["b", "a", "c"],
            EntityOrder::Unordered,
        );
        assert!(conflict.is_none());
        assert_eq!(out, vec!["a", "b", "c"], "falls back to the shared order");
    }

    #[test]
    fn placement_is_still_direction_free_when_both_sides_reorder() {
        for order in [EntityOrder::Unordered, EntityOrder::Effectful] {
            let (fwd, cf) = placement(&["a", "b", "c"], &["a", "c", "b"], &["b", "a", "c"], order);
            let (rev, cr) = placement(&["a", "b", "c"], &["b", "a", "c"], &["a", "c", "b"], order);
            assert_eq!(cf.is_some(), cr.is_some(), "{order:?}: verdict flipped");
            let (mut f, mut r) = (fwd, rev);
            f.sort();
            r.sort();
            assert_eq!(f, r, "{order:?}: entity multiset depends on direction");
        }
    }

    #[test]
    fn an_addition_after_a_deleted_neighbour_stays_where_it_was_written() {
        let out = order(
            &["alpha", "bravo", "charlie", "delta"],
            &["alpha", "charlie", "delta"],
            &["alpha", "bravo", "inserted", "charlie", "delta"],
        );
        assert_eq!(out, vec!["alpha", "inserted", "charlie", "delta"]);
    }

    #[test]
    fn a_deletion_is_not_a_reorder() {
        
        let out = order(
            &["a", "b", "c", "d"],
            &["a", "b", "c", "d"],
            &["a", "c", "d"],
        );
        assert_eq!(out, vec!["a", "c", "d"]);
    }

    #[test]
    fn a_survivor_missing_from_the_chosen_order_keeps_its_base_neighbourhood() {
        
        let (arena, claims) = build_arena(
            raws(&["a", "b", "c", "d"]),
            raws(&["d", "a", "c"]),
            raws(&["a", "b", "c", "d"]),
        );
        let m = match_phase(arena, claims);
        let cells: Vec<_> = m.triples.iter().map(|tr| classify(&m.arena, tr)).collect();
        let fmt = MarkerFormat::default();
        let ctx = ResolveCtx {
            marker_format: &fmt,
            indent_sensitive: true,
            decorators_compose: true,
            base_all: &[],
            ours_all: &[],
            theirs_all: &[],
            host: &crate::host::Host::default(),
        };
        let mut resolved: Vec<_> = m
            .triples
            .iter()
            .zip(cells.iter())
            .map(|(tr, c)| resolve(&m.arena, tr, *c, &ctx))
            .collect();
        
        for (t, r) in m.triples.iter().zip(resolved.iter_mut()) {
            if m.arena.get(t.representative()).name() == "b" {
                r.disposition = Disposition::Emit {
                    text: "def b():\n".into(),
                    name: "b".into(),
                };
            }
        }
        let placed = plan(&m.arena, &m.triples, &resolved, EntityOrder::Unordered);
        let names: Vec<String> = placed
            .items
            .iter()
            .map(|p| p.anchor.key.name.clone())
            .collect();
        assert_eq!(names, vec!["d", "a", "b", "c"]);
    }

    #[test]
    fn the_language_table_names_the_families_it_claims() {
        for p in ["a.c", "a.h", "lib.cpp", "m.ml", "m.mli", "s.sh", "q.sql"] {
            assert_eq!(EntityOrder::of_path(p), EntityOrder::Effectful, "{p}");
        }
        for p in ["a.py", "a.rs", "a.ts", "a.go", "a.java"] {
            assert_eq!(EntityOrder::of_path(p), EntityOrder::Unordered, "{p}");
        }
    }
}
