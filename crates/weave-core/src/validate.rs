
use std::collections::BTreeSet;

use sem_core::parser::graph::EntityGraph;
use sem_core::parser::registry::ParserRegistry;

#[derive(Debug, Clone)]
pub struct SemanticWarning {
    
    pub entity_name: String,
    pub entity_type: String,
    pub file_path: String,
    
    pub kind: WarningKind,
    
    pub related: Vec<RelatedEntity>,
}

#[derive(Debug, Clone)]
pub enum WarningKind {
    
    DependencyAlsoModified,
    
    DependentAlsoModified,
    
    ParseFailedAfterMerge,
    
    CompositionLicensed {
        
        ours_binds: Vec<String>,
        
        theirs_binds: Vec<String>,
    },
    
    ConflictFrameDuplicate {
        
        line: String,
        
        found: usize,
        
        allowed: usize,
    },
    
    SiblingCoChange {
        
        ours_added: Vec<String>,
        
        ours_changed: Vec<String>,
        
        theirs_added: Vec<String>,
        
        theirs_changed: Vec<String>,
    },
}

#[derive(Debug, Clone)]
pub struct RelatedEntity {
    pub name: String,
    pub entity_type: String,
    pub file_path: String,
}

pub fn validate_merge(
    repo_root: &std::path::Path,
    file_paths: &[String],
    modified_entities: &[ModifiedEntity],
    registry: &ParserRegistry,
) -> Vec<SemanticWarning> { panic!("STUB: not implemented") }

#[derive(Debug, Clone)]
pub struct ModifiedEntity {
    pub name: String,
    pub file_path: String,
}

impl std::fmt::Display for SemanticWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { panic!("STUB: not implemented") }
}

pub fn co_change_side_phrase(side: &str, added: &[String], changed: &[String]) -> String { panic!("STUB: not implemented") }

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    fn setup_test_repo() -> TempDir {
        let dir = TempDir::new().unwrap();

        let ts_content = r#"export function validateInput(input: string): boolean {
    return input.length > 0;
}

export function processData(input: string): string {
    if (!validateInput(input)) {
        throw new Error("invalid");
    }
    return input.toUpperCase();
}

export function unrelated(): number {
    return 42;
}
"#;
        let ts_path = dir.path().join("module.ts");
        let mut f = std::fs::File::create(&ts_path).unwrap();
        f.write_all(ts_content.as_bytes()).unwrap();

        dir
    }

    #[test]
    fn test_no_warnings_single_entity() {
        let dir = setup_test_repo();
        let registry = sem_core::parser::plugins::create_default_registry();
        let warnings = validate_merge(
            dir.path(),
            &["module.ts".to_string()],
            &[ModifiedEntity {
                name: "unrelated".to_string(),
                file_path: "module.ts".to_string(),
            }],
            &registry,
        );
        assert!(warnings.is_empty(), "Single entity should have no warnings");
    }

    #[test]
    fn test_warning_when_caller_and_callee_both_modified() {
        let dir = setup_test_repo();
        let registry = sem_core::parser::plugins::create_default_registry();
        let warnings = validate_merge(
            dir.path(),
            &["module.ts".to_string()],
            &[
                ModifiedEntity {
                    name: "validateInput".to_string(),
                    file_path: "module.ts".to_string(),
                },
                ModifiedEntity {
                    name: "processData".to_string(),
                    file_path: "module.ts".to_string(),
                },
            ],
            &registry,
        );
        assert!(
            !warnings.is_empty(),
            "Should warn when caller and callee both modified. Warnings: {:?}",
            warnings
        );
        
        let has_dep_warning = warnings.iter().any(|w| {
            w.entity_name == "processData"
                && matches!(w.kind, WarningKind::DependencyAlsoModified)
                && w.related.iter().any(|r| r.name == "validateInput")
        });
        assert!(
            has_dep_warning,
            "Should warn that processData depends on validateInput"
        );
    }

    #[test]
    fn test_no_warning_unrelated_entities() {
        let dir = setup_test_repo();
        let registry = sem_core::parser::plugins::create_default_registry();
        let warnings = validate_merge(
            dir.path(),
            &["module.ts".to_string()],
            &[
                ModifiedEntity {
                    name: "validateInput".to_string(),
                    file_path: "module.ts".to_string(),
                },
                ModifiedEntity {
                    name: "unrelated".to_string(),
                    file_path: "module.ts".to_string(),
                },
            ],
            &registry,
        );
        
        let cross_warnings: Vec<_> = warnings
            .iter()
            .filter(|w| {
                (w.entity_name == "validateInput"
                    && w.related.iter().any(|r| r.name == "unrelated"))
                    || (w.entity_name == "unrelated"
                        && w.related.iter().any(|r| r.name == "validateInput"))
            })
            .collect();
        assert!(
            cross_warnings.is_empty(),
            "Unrelated entities should not trigger cross-warnings"
        );
    }
}
