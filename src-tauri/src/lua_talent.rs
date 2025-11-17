use anyhow::{Context, Result};
use full_moon::ast::{Expression, Field, Stmt, TableConstructor, Var};
use std::collections::HashMap;
use std::path::Path;

/// Represents a single talent loadout entry
#[derive(Debug, Clone, PartialEq)]
pub struct TalentLoadout {
    pub icon: i64,
    pub name: String,
    pub text: String,
}

impl TalentLoadout {
    pub fn new(name: String, text: String) -> Self {
        Self {
            icon: 0,
            name,
            text,
        }
    }

    /// Check if this is an auto-generated talent (has _ARCT suffix)
    pub fn is_auto_generated(&self) -> bool {
        self.name.ends_with("_ARCT")
    }
}

/// Represents all talent loadouts for a class
/// Organized by specialization index (1-4)
pub type ClassTalents = HashMap<u8, Vec<TalentLoadout>>;

/// Manager for reading and writing TalentLoadoutsEx.lua files
pub struct LuaTalentManager {
    /// All talents organized by class name (e.g., "WARRIOR", "MAGE")
    talents: HashMap<String, ClassTalents>,
}

impl LuaTalentManager {
    /// Create a new empty manager
    pub fn new() -> Self {
        Self {
            talents: HashMap::new(),
        }
    }

    /// Load talents from a TalentLoadoutsEx.lua file
    pub fn load_from_file(path: impl AsRef<Path>) -> Result<Self> {
        let content = std::fs::read_to_string(path.as_ref())
            .context("Failed to read TalentLoadoutsEx.lua file")?;

        Self::parse_lua(&content)
    }

    /// Parse Lua content into talent structure
    fn parse_lua(content: &str) -> Result<Self> {
        let ast = full_moon::parse(content).context("Failed to parse Lua file")?;

        let mut talents: HashMap<String, ClassTalents> = HashMap::new();

        // Find the TalentLoadoutEx table assignment
        for stmt in ast.nodes().stmts() {
            if let Stmt::Assignment(assignment) = stmt {
                // Check if this is assigning to TalentLoadoutEx
                let var_names: Vec<_> = assignment
                    .variables()
                    .iter()
                    .filter_map(|v| {
                        if let Var::Name(name) = v {
                            Some(name.token().to_string())
                        } else {
                            None
                        }
                    })
                    .collect();

                if var_names.contains(&"TalentLoadoutEx".to_string()) {
                    // Parse the table value
                    if let Some(Expression::TableConstructor(table)) = assignment.expressions().iter().next() {
                        talents = Self::parse_talent_table(table)?;
                    }
                }
            }
        }

        Ok(Self { talents })
    }

    /// Parse the main talent table (class -> specs -> talents)
    fn parse_talent_table(table: &TableConstructor) -> Result<HashMap<String, ClassTalents>> {
        let mut result = HashMap::new();

        for field in table.fields() {
            if let Field::ExpressionKey { key, value, .. } = field {
                // Get class name (e.g., "WARRIOR")
                if let Expression::String(string_token) = key {
                    let class_name = string_token.token().to_string().replace("\"", "");

                    // Skip OPTION key
                    if class_name == "OPTION" {
                        continue;
                    }

                    // Parse spec tables
                    if let Expression::TableConstructor(spec_table) = value {
                        if let Ok(class_talents) = Self::parse_class_talents(spec_table) {
                            result.insert(class_name, class_talents);
                        }
                    }
                }
            }
        }

        Ok(result)
    }

    /// Parse all specs for a class
    fn parse_class_talents(spec_table: &TableConstructor) -> Result<ClassTalents> {
        let mut result = HashMap::new();

        for field in spec_table.fields() {
            if let Field::ExpressionKey { key, value, .. } = field {
                // Get spec index (e.g., 1, 2, 3)
                if let Expression::Number(num) = key {
                    if let Ok(spec_index) = num.token().to_string().parse::<u8>() {
                        // Parse talent list for this spec
                        if let Expression::TableConstructor(talent_table) = value {
                            let talents = Self::parse_talent_list(talent_table)?;
                            result.insert(spec_index, talents);
                        }
                    }
                }
            }
        }

        Ok(result)
    }

    /// Parse a list of talent loadouts
    fn parse_talent_list(talent_table: &TableConstructor) -> Result<Vec<TalentLoadout>> {
        let mut result = Vec::new();

        for field in talent_table.fields() {
            if let Field::NoKey(expression) = field {
                if let Expression::TableConstructor(loadout_table) = expression {
                    if let Ok(loadout) = Self::parse_single_talent(loadout_table) {
                        result.push(loadout);
                    }
                }
            }
        }

        Ok(result)
    }

    /// Parse a single talent loadout entry
    fn parse_single_talent(loadout_table: &TableConstructor) -> Result<TalentLoadout> {
        let mut icon: i64 = 0;
        let mut name = String::new();
        let mut text = String::new();

        for field in loadout_table.fields() {
            // Handle both ["key"] = value and key = value syntax
            let (key_str, value) = match field {
                Field::ExpressionKey { key, value, .. } => {
                    if let Expression::String(s) = key {
                        (s.token().to_string().trim_matches('"').to_string(), value)
                    } else {
                        continue;
                    }
                }
                Field::NameKey { key, value, .. } => {
                    (key.token().to_string(), value)
                }
                _ => continue,
            };

            match key_str.as_str() {
                "icon" => {
                    if let Expression::Number(num) = value {
                        icon = num.token().to_string().parse().unwrap_or(0);
                    }
                }
                "name" => {
                    if let Expression::String(s) = value {
                        name = s.token().to_string().trim_matches('"').to_string();
                    }
                }
                "text" => {
                    if let Expression::String(s) = value {
                        text = s.token().to_string().trim_matches('"').to_string();
                    }
                }
                _ => {}
            }
        }

        Ok(TalentLoadout { icon, name, text })
    }

    /// Get all talents for a specific class
    #[allow(dead_code)]
    pub fn get_class_talents(&self, class_name: &str) -> Option<&ClassTalents> {
        self.talents.get(class_name)
    }

    /// Get all talents for a specific class and spec
    #[allow(dead_code)]
    pub fn get_spec_talents(&self, class_name: &str, spec_index: u8) -> Option<&Vec<TalentLoadout>> {
        self.talents.get(class_name)?.get(&spec_index)
    }

    /// Set talents for a specific class and spec
    /// This replaces all talents for that spec
    #[allow(dead_code)]
    pub fn set_spec_talents(&mut self, class_name: String, spec_index: u8, talents: Vec<TalentLoadout>) {
        self.talents
            .entry(class_name)
            .or_insert_with(HashMap::new)
            .insert(spec_index, talents);
    }

    /// Remove all auto-generated talents (with _ARCT suffix) for a specific class/spec
    pub fn remove_auto_generated(&mut self, class_name: &str, spec_index: u8) {
        if let Some(class_talents) = self.talents.get_mut(class_name) {
            if let Some(spec_talents) = class_talents.get_mut(&spec_index) {
                spec_talents.retain(|t| !t.is_auto_generated());
            }
        }
    }

    /// Remove all auto-generated talents across all classes and specs
    pub fn remove_all_auto_generated(&mut self) {
        for class_talents in self.talents.values_mut() {
            for spec_talents in class_talents.values_mut() {
                spec_talents.retain(|t| !t.is_auto_generated());
            }
        }
    }

    /// Add a talent to a specific class/spec
    pub fn add_talent(&mut self, class_name: String, spec_index: u8, talent: TalentLoadout) {
        self.talents
            .entry(class_name)
            .or_insert_with(HashMap::new)
            .entry(spec_index)
            .or_insert_with(Vec::new)
            .push(talent);
    }

    /// Write talents to a Lua file
    pub fn write_to_file(&self, path: impl AsRef<Path>) -> Result<()> {
        let lua_content = self.to_lua_string();
        std::fs::write(path.as_ref(), lua_content)
            .context("Failed to write TalentLoadoutsEx.lua file")?;
        Ok(())
    }

    /// Convert talents to Lua string format
    fn to_lua_string(&self) -> String {
        let mut result = String::from("TalentLoadoutEx = {\n");

        // Sort class names for consistent output
        let mut class_names: Vec<_> = self.talents.keys().collect();
        class_names.sort();

        for class_name in class_names {
            let class_talents = &self.talents[class_name];

            result.push_str(&format!("  [\"{}\"] = {{\n", class_name));

            // Sort spec indices
            let mut spec_indices: Vec<_> = class_talents.keys().copied().collect();
            spec_indices.sort();

            for spec_index in spec_indices {
                let talents = &class_talents[&spec_index];

                result.push_str(&format!("    [{}] = {{\n", spec_index));

                for talent in talents {
                    result.push_str(&format!(
                        "      {{ [\"icon\"] = {}, [\"name\"] = \"{}\", [\"text\"] = \"{}\" }},\n",
                        talent.icon, talent.name, talent.text
                    ));
                }

                result.push_str("    },\n");
            }

            result.push_str("  },\n");
        }

        // Add OPTION table (always false for IsEnabledPvp)
        result.push_str("  [\"OPTION\"] = { [\"IsEnabledPvp\"] = false },\n");

        result.push_str("}\n");
        result
    }
}

impl Default for LuaTalentManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_lua() -> String {
        r#"TalentLoadoutEx = {
  ["WARRIOR"] = {
    [1] = {
      { ["icon"] = 132355, ["name"] = "My Arms Build", ["text"] = "warrior/arms/ABC123" },
      { ["icon"] = 0, ["name"] = "R-heroic-sikran_ARCT", ["text"] = "warrior/arms/XYZ789" },
    },
    [2] = {
      { ["icon"] = 132347, ["name"] = "My Fury Build", ["text"] = "warrior/fury/DEF456" },
    },
  },
  ["MAGE"] = {
    [3] = {
      { ["icon"] = 135846, ["name"] = "M+-ara-kara_ARCT", ["text"] = "mage/frost/GHI789" },
    },
  },
  ["OPTION"] = { ["IsEnabledPvp"] = false },
}"#.to_string()
    }

    #[test]
    fn test_parse_lua() {
        let lua = create_test_lua();
        let manager = LuaTalentManager::parse_lua(&lua).unwrap();

        // Check warrior talents
        let warrior_talents = manager.get_class_talents("WARRIOR").unwrap();
        assert_eq!(warrior_talents.len(), 2); // 2 specs

        let arms_talents = warrior_talents.get(&1).unwrap();
        assert_eq!(arms_talents.len(), 2); // 2 talents

        assert_eq!(arms_talents[0].name, "My Arms Build");
        assert_eq!(arms_talents[1].name, "R-heroic-sikran_ARCT");

        // Check mage talents
        let frost_talents = manager.get_spec_talents("MAGE", 3).unwrap();
        assert_eq!(frost_talents.len(), 1);
        assert_eq!(frost_talents[0].name, "M+-ara-kara_ARCT");
    }

    #[test]
    fn test_is_auto_generated() {
        let auto = TalentLoadout::new("R-heroic-sikran_ARCT".to_string(), "test".to_string());
        assert!(auto.is_auto_generated());

        let manual = TalentLoadout::new("My Build".to_string(), "test".to_string());
        assert!(!manual.is_auto_generated());
    }

    #[test]
    fn test_remove_auto_generated() {
        let lua = create_test_lua();
        let mut manager = LuaTalentManager::parse_lua(&lua).unwrap();

        // Remove auto-generated from warrior arms
        manager.remove_auto_generated("WARRIOR", 1);

        let arms_talents = manager.get_spec_talents("WARRIOR", 1).unwrap();
        assert_eq!(arms_talents.len(), 1); // Only manual build remains
        assert_eq!(arms_talents[0].name, "My Arms Build");
    }

    #[test]
    fn test_add_talent() {
        let mut manager = LuaTalentManager::new();

        let talent = TalentLoadout::new(
            "R-normal-broodtwister_ARCT".to_string(),
            "warrior/arms/TEST123".to_string(),
        );

        manager.add_talent("WARRIOR".to_string(), 1, talent);

        let talents = manager.get_spec_talents("WARRIOR", 1).unwrap();
        assert_eq!(talents.len(), 1);
        assert_eq!(talents[0].name, "R-normal-broodtwister_ARCT");
    }

    #[test]
    fn test_to_lua_string() {
        let mut manager = LuaTalentManager::new();

        let talent1 = TalentLoadout {
            icon: 132355,
            name: "Build 1".to_string(),
            text: "warrior/arms/ABC".to_string(),
        };

        manager.add_talent("WARRIOR".to_string(), 1, talent1);

        let lua_string = manager.to_lua_string();

        assert!(lua_string.contains("TalentLoadoutEx"));
        assert!(lua_string.contains("[\"WARRIOR\"]"));
        assert!(lua_string.contains("[1]"));
        assert!(lua_string.contains("Build 1"));
        assert!(lua_string.contains("warrior/arms/ABC"));
        assert!(lua_string.contains("[\"OPTION\"]"));
    }
}
