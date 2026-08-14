
use sem_core::model::entity::SemanticEntity;

use super::types::*;
use crate::binding::replace_at_word_boundaries;
use crate::conflict::{
    classify_conflict, ConflictComplexity, ConflictKind, EntityConflict, MarkerFormat,
};
use crate::merge::ResolutionStrategy;

pub(crate) struct ResolveCtx<'a> {
    pub marker_format: &'a MarkerFormat,
    
    pub indent_sensitive: bool,
    
    pub decorators_compose: bool,
    pub base_all: &'a [SemanticEntity],
    pub ours_all: &'a [SemanticEntity],
    pub theirs_all: &'a [SemanticEntity],
    
    pub host: &'a crate::host::Host,
}

pub(crate) struct Resolved {
    pub disposition: Disposition,
    pub strategy: ResolutionStrategy,
    
    rename: Option<(String, String)>,
    
    pub tally: Tally,
}

impl Resolved {
    
    pub(crate) fn rename(&self) -> Option<&(String, String)> { panic!("STUB: not implemented") }

    pub(crate) fn revised(
        disposition: Disposition,
        strategy: ResolutionStrategy,
        tally: Tally,
    ) -> Self { panic!("STUB: not implemented") }
}

pub(crate) fn resolve(
    arena: &Arena,
    triple: &Triple,
    cell: Cell,
    ctx: &ResolveCtx<'_>,
) -> Resolved { panic!("STUB: not implemented") }

#[allow(clippy::too_many_arguments)]
fn intra_entity(
    arena: &Arena,
    triple: &Triple,
    ctx: &ResolveCtx<'_>,
    base_rc: &str,
    ours_rc: &str,
    theirs_rc: &str,
    output: Option<Idx>,
    rename: Option<(Side, String, String)>,
) -> Resolved { panic!("STUB: not implemented") }

pub(crate) fn inner_merge(
    arena: &Arena,
    triple: &Triple,
    ctx: &ResolveCtx<'_>,
    base_rc: &str,
    ours_rc: &str,
    theirs_rc: &str,
    license: crate::statement::License,
    evidence: &mut Vec<crate::statement::LicensedGap>,
) -> Option<crate::container::InnerMergeResult> { panic!("STUB: not implemented") }

fn region_start_line(entity: &SemanticEntity, region: &str) -> usize { panic!("STUB: not implemented") }

pub(crate) fn conflict_disposition(
    conflict: EntityConflict,
    ctx: &ResolveCtx<'_>,
    strategy: ResolutionStrategy,
) -> Resolved { panic!("STUB: not implemented") }
