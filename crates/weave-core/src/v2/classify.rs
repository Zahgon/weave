
use super::types::*;

pub(crate) fn action(arena: &Arena, triple: &Triple, side: Side) -> Action { panic!("STUB: not implemented") }

fn rank(a: Action) -> u8 { panic!("STUB: not implemented") }

pub(crate) fn classify(arena: &Arena, triple: &Triple) -> Cell { panic!("STUB: not implemented") }

#[cfg(test)]
mod tests {
    use super::super::match_phase::{build_arena, match_phase, RawEntity};
    use super::*;
    use std::collections::HashSet;

    fn raw(name: &str, body: &str) -> RawEntity {
        RawEntity {
            name: name.to_string(),
            entity_type: "function".to_string(),
            parent: None,
            content: format!("def {name}():\n{body}"),
            src_id: format!("f::{name}"),
        }
    }

    type Scenario = (&'static str, Vec<RawEntity>, Vec<RawEntity>, Vec<RawEntity>);

    fn cells(b: Vec<RawEntity>, o: Vec<RawEntity>, t: Vec<RawEntity>) -> Vec<Cell> {
        let (arena, claims) = build_arena(b, o, t);
        let m = match_phase(arena, claims);
        m.triples.iter().map(|t| classify(&m.arena, t)).collect()
    }

    const BODY: &str = "    for item in fetch():\n        record(item)\n    return done()\n";
    const EDIT_A: &str =
        "    touched_a()\n    for item in fetch():\n        record(item)\n    return done()\n";
    const EDIT_B: &str =
        "    touched_b()\n    for item in fetch():\n        record(item)\n    return done()\n";

    #[test]
    fn every_cell_the_universe_can_reach_is_reached_symmetrically() {
        
        let scenarios: Vec<Scenario> = vec![
            (
                "unchanged",
                vec![raw("a", BODY)],
                vec![raw("a", BODY)],
                vec![raw("a", BODY)],
            ),
            (
                "edit one side",
                vec![raw("a", BODY)],
                vec![raw("a", EDIT_A)],
                vec![raw("a", BODY)],
            ),
            (
                "edit both divergent",
                vec![raw("a", BODY)],
                vec![raw("a", EDIT_A)],
                vec![raw("a", EDIT_B)],
            ),
            (
                "edit both convergent",
                vec![raw("a", BODY)],
                vec![raw("a", EDIT_A)],
                vec![raw("a", EDIT_A)],
            ),
            (
                "delete vs edit",
                vec![raw("a", BODY)],
                vec![],
                vec![raw("a", EDIT_A)],
            ),
            ("delete both", vec![raw("a", BODY)], vec![], vec![]),
            (
                "rename vs unchanged",
                vec![raw("a", BODY)],
                vec![raw("a_v2", BODY)],
                vec![raw("a", BODY)],
            ),
            (
                "rename both divergent names",
                vec![raw("a", BODY)],
                vec![raw("a_v2", BODY)],
                vec![raw("a_alt", BODY)],
            ),
            (
                "added one side",
                vec![raw("a", BODY)],
                vec![raw("a", BODY), raw("z", BODY)],
                vec![raw("a", BODY)],
            ),
            (
                "added both divergent",
                vec![raw("a", BODY)],
                vec![raw("a", BODY), raw("z", EDIT_A)],
                vec![raw("a", BODY), raw("z", EDIT_B)],
            ),
        ];
        for (label, b, o, t) in scenarios {
            let fwd = cells(clone_raws(&b), clone_raws(&o), clone_raws(&t));
            let rev = cells(clone_raws(&b), clone_raws(&t), clone_raws(&o));
            let fwd_flipped: HashSet<String> =
                fwd.iter().map(|c| format!("{:?}", c.flip())).collect();
            let rev_set: HashSet<String> = rev.iter().map(|c| format!("{c:?}")).collect();
            assert_eq!(
                fwd_flipped, rev_set,
                "classification is not symmetric for {label}: {fwd:?} vs {rev:?}"
            );
        }
    }

    fn clone_raws(v: &[RawEntity]) -> Vec<RawEntity> {
        v.iter()
            .map(|r| RawEntity {
                name: r.name.clone(),
                entity_type: r.entity_type.clone(),
                parent: r.parent.clone(),
                content: r.content.clone(),
                src_id: r.src_id.clone(),
            })
            .collect()
    }

    #[test]
    fn the_edit_pair_is_discriminated_by_convergence() {
        assert_eq!(
            cells(
                vec![raw("a", BODY)],
                vec![raw("a", EDIT_A)],
                vec![raw("a", EDIT_A)]
            ),
            vec![Cell::EditBothConvergent]
        );
        assert_eq!(
            cells(
                vec![raw("a", BODY)],
                vec![raw("a", EDIT_A)],
                vec![raw("a", EDIT_B)]
            ),
            vec![Cell::EditBothDivergent]
        );
    }

    #[test]
    fn convergent_renames_are_not_a_rename_rename_conflict() {
        
        assert_eq!(
            cells(
                vec![raw("a", BODY)],
                vec![raw("a_v2", BODY)],
                vec![raw("a_v2", BODY)]
            ),
            vec![Cell::RenameBoth {
                names_converge: true
            }]
        );
    }
}
