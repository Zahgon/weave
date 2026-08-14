
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub(crate) struct Idx(pub u32);

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Side {
    Ours,
    Theirs,
}

impl Side {
    #[must_use]
    pub(crate) fn flip(self) -> Side { panic!("STUB: not implemented") }
}

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub(crate) struct Key {
    pub parent: Option<String>,
    pub entity_type: String,
    pub name: String,
    pub ordinal: u32,
}

#[derive(Debug, Clone)]
pub(crate) struct Entity {
    pub key: Key,
    
    pub content: String,
    
    pub body_hash: u64,
    
    pub position: u32,
    
    pub src_id: String,
}

impl Entity {
    pub(crate) fn name(&self) -> &str { panic!("STUB: not implemented") }
    pub(crate) fn entity_type(&self) -> &str { panic!("STUB: not implemented") }
}

#[derive(Debug, Default)]
pub(crate) struct Arena {
    entities: Vec<Entity>,
}

impl Arena {
    pub(crate) fn from_entities(entities: Vec<Entity>) -> Self { panic!("STUB: not implemented") }
    pub(crate) fn get(&self, idx: Idx) -> &Entity { panic!("STUB: not implemented") }
    pub(crate) fn len(&self) -> usize { panic!("STUB: not implemented") }
}

macro_rules! claim_type {
    ($name:ident) => {
        #[derive(Debug, PartialEq, Eq)]
        pub(crate) struct $name(Idx);
        impl $name {
            pub(crate) fn new(idx: Idx) -> Self {
                Self(idx)
            }
            pub(crate) fn idx(&self) -> Idx {
                self.0
            }
        }
    };
}

claim_type!(BaseClaim);
claim_type!(OursClaim);
claim_type!(TheirsClaim);

#[derive(Debug)]
pub(crate) struct Claims {
    pub(crate) base: Vec<BaseClaim>,
    pub(crate) ours: Vec<OursClaim>,
    pub(crate) theirs: Vec<TheirsClaim>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Link {
    
    Absent,
    
    Identity,
    
    BodyHash,
    
    Signature { similarity: f64 },
    
    CallSiteCorroborated { similarity: f64 },
    
    Added,
}

impl Link {
    
    #[cfg(test)]
    pub(crate) fn is_rename(&self) -> bool {
        matches!(
            self,
            Link::BodyHash | Link::Signature { .. } | Link::CallSiteCorroborated { .. }
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Relational {
    
    RehomedInto {
        deleted: Idx,
        side: Side,
        homes: Vec<Idx>,
        coverage: f64,
    },
    
    ExtractedInto {
        source: Idx,
        side: Side,
        host: Idx,
        extracted: Vec<Idx>,
        moved: Vec<String>,
    },
}

#[derive(Debug)]
pub struct Triple {
    pub(crate) base: Option<BaseClaim>,
    pub(crate) ours: Option<OursClaim>,
    pub(crate) theirs: Option<TheirsClaim>,
    
    #[allow(dead_code)]
    pub(crate) evidence: Evidence,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Evidence {
    pub ours_link: Link,
    pub theirs_link: Link,
}

impl Triple {
    pub(crate) fn base_idx(&self) -> Option<Idx> { panic!("STUB: not implemented") }
    pub(crate) fn ours_idx(&self) -> Option<Idx> { panic!("STUB: not implemented") }
    pub(crate) fn theirs_idx(&self) -> Option<Idx> { panic!("STUB: not implemented") }
    pub(crate) fn side_idx(&self, side: Side) -> Option<Idx> { panic!("STUB: not implemented") }
    
    pub(crate) fn representative(&self) -> Idx { panic!("STUB: not implemented") }
}

#[derive(Debug)]
pub struct Matching {
    pub(crate) arena: Arena,
    pub(crate) triples: Vec<Triple>,
    pub(crate) relational: Vec<Relational>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Action {
    Added,
    
    Absent,
    Deleted,
    Unchanged,
    Edited,
    Renamed,
    RenameEdited,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cell {
    
    AddedOneSide {
        adder: Side,
    },
    
    AddedBothConvergent,
    
    AddedBothDivergent,

    DeletedBoth,
    DeleteVsUnchanged {
        deleter: Side,
    },
    DeleteVsEdit {
        deleter: Side,
    },
    DeleteVsRename {
        deleter: Side,
    },
    DeleteVsRenameEdit {
        deleter: Side,
    },

    UnchangedBoth,
    EditOneSide {
        editor: Side,
    },
    EditBothConvergent,
    EditBothDivergent,

    RenameVsUnchanged {
        renamer: Side,
    },
    RenameEditVsUnchanged {
        renamer: Side,
    },
    RenameVsEdit {
        renamer: Side,
    },
    RenameEditVsEdit {
        renamer: Side,
        bodies_converge: bool,
    },

    RenameBoth {
        names_converge: bool,
    },
    RenameEditVsRename {
        rename_editor: Side,
        names_converge: bool,
    },
    RenameEditBoth {
        names_converge: bool,
        bodies_converge: bool,
    },
}

impl Cell {
    
    pub const ALL: [Cell; 38] = {
        use Side::{Ours, Theirs};
        [
            Cell::AddedOneSide { adder: Ours },
            Cell::AddedOneSide { adder: Theirs },
            Cell::AddedBothConvergent,
            Cell::AddedBothDivergent,
            Cell::DeletedBoth,
            Cell::DeleteVsUnchanged { deleter: Ours },
            Cell::DeleteVsUnchanged { deleter: Theirs },
            Cell::DeleteVsEdit { deleter: Ours },
            Cell::DeleteVsEdit { deleter: Theirs },
            Cell::DeleteVsRename { deleter: Ours },
            Cell::DeleteVsRename { deleter: Theirs },
            Cell::DeleteVsRenameEdit { deleter: Ours },
            Cell::DeleteVsRenameEdit { deleter: Theirs },
            Cell::UnchangedBoth,
            Cell::EditOneSide { editor: Ours },
            Cell::EditOneSide { editor: Theirs },
            Cell::EditBothConvergent,
            Cell::EditBothDivergent,
            Cell::RenameVsUnchanged { renamer: Ours },
            Cell::RenameVsUnchanged { renamer: Theirs },
            Cell::RenameEditVsUnchanged { renamer: Ours },
            Cell::RenameEditVsUnchanged { renamer: Theirs },
            Cell::RenameVsEdit { renamer: Ours },
            Cell::RenameVsEdit { renamer: Theirs },
            Cell::RenameEditVsEdit {
                renamer: Ours,
                bodies_converge: true,
            },
            Cell::RenameEditVsEdit {
                renamer: Ours,
                bodies_converge: false,
            },
            Cell::RenameEditVsEdit {
                renamer: Theirs,
                bodies_converge: true,
            },
            Cell::RenameEditVsEdit {
                renamer: Theirs,
                bodies_converge: false,
            },
            Cell::RenameBoth {
                names_converge: true,
            },
            Cell::RenameBoth {
                names_converge: false,
            },
            Cell::RenameEditVsRename {
                rename_editor: Ours,
                names_converge: true,
            },
            Cell::RenameEditVsRename {
                rename_editor: Ours,
                names_converge: false,
            },
            Cell::RenameEditVsRename {
                rename_editor: Theirs,
                names_converge: true,
            },
            Cell::RenameEditVsRename {
                rename_editor: Theirs,
                names_converge: false,
            },
            Cell::RenameEditBoth {
                names_converge: true,
                bodies_converge: true,
            },
            Cell::RenameEditBoth {
                names_converge: true,
                bodies_converge: false,
            },
            Cell::RenameEditBoth {
                names_converge: false,
                bodies_converge: true,
            },
            Cell::RenameEditBoth {
                names_converge: false,
                bodies_converge: false,
            },
        ]
    };

    #[cfg(test)]
    #[must_use]
    pub(crate) fn flip(self) -> Cell {
        match self {
            Cell::AddedOneSide { adder } => Cell::AddedOneSide {
                adder: adder.flip(),
            },
            Cell::DeleteVsUnchanged { deleter } => Cell::DeleteVsUnchanged {
                deleter: deleter.flip(),
            },
            Cell::DeleteVsEdit { deleter } => Cell::DeleteVsEdit {
                deleter: deleter.flip(),
            },
            Cell::DeleteVsRename { deleter } => Cell::DeleteVsRename {
                deleter: deleter.flip(),
            },
            Cell::DeleteVsRenameEdit { deleter } => Cell::DeleteVsRenameEdit {
                deleter: deleter.flip(),
            },
            Cell::EditOneSide { editor } => Cell::EditOneSide {
                editor: editor.flip(),
            },
            Cell::RenameVsUnchanged { renamer } => Cell::RenameVsUnchanged {
                renamer: renamer.flip(),
            },
            Cell::RenameEditVsUnchanged { renamer } => Cell::RenameEditVsUnchanged {
                renamer: renamer.flip(),
            },
            Cell::RenameVsEdit { renamer } => Cell::RenameVsEdit {
                renamer: renamer.flip(),
            },
            Cell::RenameEditVsEdit {
                renamer,
                bodies_converge,
            } => Cell::RenameEditVsEdit {
                renamer: renamer.flip(),
                bodies_converge,
            },
            Cell::RenameEditVsRename {
                rename_editor,
                names_converge,
            } => Cell::RenameEditVsRename {
                rename_editor: rename_editor.flip(),
                names_converge,
            },
            
            other => other,
        }
    }

    pub fn actions(self) -> (Action, Action) { panic!("STUB: not implemented") }

    pub fn is_trivially_clean(self) -> bool { panic!("STUB: not implemented") }
}

#[derive(Debug)]
pub(crate) enum Disposition {
    
    Emit { text: String, name: String },
    
    Drop,
    
    Conflict {
        text: String,
        conflict: crate::conflict::EntityConflict,
    },
}

impl Disposition {
    pub(crate) fn is_conflict(&self) -> bool { panic!("STUB: not implemented") }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Tally {
    Unchanged,
    OursOnly,
    TheirsOnly,
    BothMerged,
    AddedOurs,
    AddedTheirs,
    Deleted,
    Conflicted,
}

impl Tally {
    
    pub(crate) fn only(side: Side) -> Tally { panic!("STUB: not implemented") }
    
    pub(crate) fn added(side: Side) -> Tally { panic!("STUB: not implemented") }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Anchor {
    
    pub after_base: u32,
    
    pub tier: u8,
    
    pub within: u32,
    pub key: Key,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum EntityOrder {
    
    Unordered,
    
    Effectful,
}

impl EntityOrder {
    
    pub(crate) fn of_path(file_path: &str) -> EntityOrder { panic!("STUB: not implemented") }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct OrderConflict {
    pub base: Vec<String>,
    pub ours: Vec<String>,
    pub theirs: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn all_cases_are_distinct() {
        
        let unique: HashSet<String> = Cell::ALL.iter().map(|c| format!("{c:?}")).collect();
        assert_eq!(unique.len(), 38, "Cell::ALL has duplicates");
        assert_eq!(Cell::ALL.len(), 38);
    }

    #[test]
    fn swapping_sides_twice_is_identity() {
        for cell in Cell::ALL {
            assert_eq!(
                cell.flip().flip(),
                cell,
                "swapping sides twice changed the case at {cell:?}"
            );
        }
    }

    #[test]
    fn swapping_sides_maps_cases_onto_cases() {
        let all: HashSet<String> = Cell::ALL.iter().map(|c| format!("{c:?}")).collect();
        let flipped: HashSet<String> = Cell::ALL
            .iter()
            .map(|c| format!("{:?}", c.flip()))
            .collect();
        assert_eq!(all, flipped, "swap produced a case outside the set");
    }

    #[test]
    fn swapping_sides_swaps_the_action_pair() {
        for cell in Cell::ALL {
            let (o, t) = cell.actions();
            let (fo, ft) = cell.flip().actions();
            assert_eq!((o, t), (ft, fo), "actions disagree with swap at {cell:?}");
        }
    }

    #[test]
    fn every_action_pair_has_a_case() {
        use Action::*;
        
        let mut seen: HashSet<(Action, Action)> = HashSet::new();
        for cell in Cell::ALL {
            seen.insert(cell.actions());
        }
        for a in [Deleted, Unchanged, Edited, Renamed, RenameEdited] {
            for b in [Deleted, Unchanged, Edited, Renamed, RenameEdited] {
                assert!(seen.contains(&(a, b)), "no case for ({a:?}, {b:?})");
            }
        }
        assert!(seen.contains(&(Added, Added)));
    }
}
