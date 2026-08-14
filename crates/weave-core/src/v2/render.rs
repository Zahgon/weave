
use std::collections::{HashMap, HashSet};

use super::plan::PlannedItem;
use super::resolve::Resolved;
use super::types::*;
use crate::region::FileRegion;

pub(crate) struct Interstitials<'a> {
    pub merged: &'a HashMap<String, String>,
    
    lead_by_entity: HashMap<String, Vec<String>>,
    
    trailing: Vec<String>,
    
    separator: String,
}

impl<'a> Interstitials<'a> {
    
    pub(crate) fn new(
        merged: &'a HashMap<String, String>,
        regions: &[&[FileRegion]],
        emitted: &HashSet<String>,
    ) -> Self { panic!("STUB: not implemented") }

    fn lead_keys(&self, triple: &Triple, arena: &Arena) -> Vec<&str> { panic!("STUB: not implemented") }
}

fn dominant_gap(regions: &[FileRegion]) -> String { panic!("STUB: not implemented") }

pub(crate) fn emitted_src_ids(
    arena: &Arena,
    triples: &[Triple],
    resolved: &[Resolved],
    items: &[PlannedItem],
) -> HashSet<String> { panic!("STUB: not implemented") }

pub(crate) fn render(
    arena: &Arena,
    triples: &[Triple],
    resolved: &[Resolved],
    items: &[PlannedItem],
    interstitials: &Interstitials<'_>,
) -> String { panic!("STUB: not implemented") }
