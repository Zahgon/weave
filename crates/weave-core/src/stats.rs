
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::conflict::MergeStats;

#[derive(Serialize, Deserialize, Default)]
pub struct WeaveLifetimeStats {
    pub version: u32,
    pub first_run: Option<String>,
    pub last_run: Option<String>,
    pub total_merges: u64,
    pub total_entities_processed: u64,
    pub conflicts_auto_resolved: u64,
    pub conflicts_unresolved: u64,
    pub confidence_very_high: u64,
    pub confidence_high: u64,
    pub confidence_medium: u64,
    pub confidence_conflict: u64,
}

pub fn default_path(home: Option<&str>) -> Option<PathBuf> { panic!("STUB: not implemented") }

fn now_iso() -> String { panic!("STUB: not implemented") }

impl WeaveLifetimeStats {
    
    pub fn load(path: &Path) -> Self { panic!("STUB: not implemented") }

    pub fn record_merge(mut self, stats: &MergeStats) -> Self { panic!("STUB: not implemented") }

    pub fn save(&self, path: &Path) -> bool { panic!("STUB: not implemented") }
}
