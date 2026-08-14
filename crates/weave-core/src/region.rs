use sem_core::model::entity::SemanticEntity;

#[derive(Debug, Clone)]
pub enum FileRegion {
    Entity(EntityRegion),
    Interstitial(InterstitialRegion),
}

impl FileRegion {
    pub(crate) fn content(&self) -> &str { panic!("STUB: not implemented") }

    #[cfg(test)]
    fn key(&self) -> &str {
        match self {
            FileRegion::Entity(e) => &e.entity_id,
            FileRegion::Interstitial(i) => &i.position_key,
        }
    }

    #[cfg(test)]
    fn is_entity(&self) -> bool {
        matches!(self, FileRegion::Entity(_))
    }
}

#[derive(Debug, Clone)]
pub struct EntityRegion {
    pub entity_id: String,
    pub entity_name: String,
    pub entity_type: String,
    pub content: String,
    pub start_line: usize,
    pub end_line: usize,
}

#[derive(Debug, Clone)]
pub struct InterstitialRegion {
    
    pub position_key: String,
    pub content: String,
}

pub fn extract_regions(content: &str, entities: &[SemanticEntity]) -> Vec<FileRegion> { panic!("STUB: not implemented") }

fn find_leading_comment_start(lines: &[&str], entity_start: usize, min_line: usize) -> usize { panic!("STUB: not implemented") }

fn join_lines(lines: &[&str]) -> String { panic!("STUB: not implemented") }

#[cfg(test)]
mod tests {
    use super::*;
    use sem_core::parser::plugins::create_default_registry;

    #[test]
    fn test_extract_regions_typescript() {
        let content = r#"import { foo } from 'bar';

export function hello() {
    return "hello";
}

export function world() {
    return "world";
}
"#;

        let registry = create_default_registry();
        let plugin = registry.get_plugin("test.ts").unwrap();
        let entities = plugin.extract_entities(content, "test.ts");

        assert!(
            !entities.is_empty(),
            "Should extract entities from TypeScript"
        );

        let regions = extract_regions(content, &entities);

        assert!(
            regions.len() >= 2,
            "Should have multiple regions, got {}",
            regions.len()
        );

        let entity_regions: Vec<_> = regions
            .iter()
            .filter_map(|r| match r {
                FileRegion::Entity(e) => Some(e),
                _ => None,
            })
            .collect();

        let entity_names: Vec<&str> = entity_regions
            .iter()
            .map(|e| e.entity_name.as_str())
            .collect();
        assert!(
            entity_names.contains(&"hello"),
            "Should find hello function, got {:?}",
            entity_names
        );
        assert!(
            entity_names.contains(&"world"),
            "Should find world function, got {:?}",
            entity_names
        );
    }

    #[test]
    fn test_comment_bundling_jsdoc() {
        
        let content = r#"import { foo } from 'bar';

/**
 * Greets a person by name.
 * @param name - The person's name
 */
export function hello(name: string) {
    return `Hello, ${name}!`;
}

export function world() {
    return "world";
}
"#;

        let registry = create_default_registry();
        let plugin = registry.get_plugin("test.ts").unwrap();
        let entities = plugin.extract_entities(content, "test.ts");

        let _hello = entities
            .iter()
            .find(|e| e.name == "hello")
            .expect("Should find hello");
        let regions = extract_regions(content, &entities);

        let hello_region = regions
            .iter()
            .find(|r| match r {
                FileRegion::Entity(e) => e.entity_name == "hello",
                _ => false,
            })
            .expect("Should find hello region");

        assert!(
            hello_region.content().contains("/**"),
            "hello region should include JSDoc comment. Content: {:?}",
            hello_region.content(),
        );
        assert!(
            hello_region.content().contains("@param name"),
            "hello region should include JSDoc @param. Content: {:?}",
            hello_region.content(),
        );

        let interstitials: Vec<_> = regions.iter().filter(|r| !r.is_entity()).collect();
        for inter in &interstitials {
            assert!(
                !inter.content().contains("/**") || !inter.content().contains("@param"),
                "Interstitial should not contain the bundled JSDoc. Key: {:?}, Content: {:?}",
                inter.key(),
                inter.content(),
            );
        }
    }

    #[test]
    fn test_comment_bundling_rust_doc() {
        let content = r#"use std::io;

/// Adds two numbers together.
///
/// # Examples
/// ```
/// assert_eq!(add(1, 2), 3);
/// ```
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}
"#;

        let registry = create_default_registry();
        let plugin = registry.get_plugin("test.rs").unwrap();
        let entities = plugin.extract_entities(content, "test.rs");

        let regions = extract_regions(content, &entities);
        let add_region = regions
            .iter()
            .find(|r| match r {
                FileRegion::Entity(e) => e.entity_name == "add",
                _ => false,
            })
            .expect("Should find add region");

        assert!(
            add_region.content().contains("/// Adds two numbers"),
            "add region should include Rust doc comment. Content: {:?}",
            add_region.content(),
        );
    }

    #[test]
    fn test_extract_regions_no_entities() {
        let content = "just some text\nno code here\n";
        let regions = extract_regions(content, &[]);
        assert_eq!(regions.len(), 1);
        assert!(!regions[0].is_entity());
        assert_eq!(regions[0].content(), content);
    }
}
