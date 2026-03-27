#![allow(missing_docs)]

pub mod templates;

use std::fs;
use std::path::Path;

pub struct ProjectGenerator {
    project_name: String,
    template_type: TemplateType,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TemplateType {
    #[default]
    Basic,
    Counter,
    TodoApp,
}

impl TemplateType {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Basic => "basic",
            Self::Counter => "counter",
            Self::TodoApp => "todo-app",
        }
    }

    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "basic" => Some(Self::Basic),
            "counter" => Some(Self::Counter),
            "todo-app" => Some(Self::TodoApp),
            _ => None,
        }
    }

    #[must_use]
    pub const fn all() -> &'static [Self] {
        &[Self::Basic, Self::Counter, Self::TodoApp]
    }
}

impl ProjectGenerator {
    #[must_use]
    pub fn new(project_name: &str, template_type: TemplateType) -> Self {
        Self {
            project_name: project_name.to_string(),
            template_type,
        }
    }

    /// # Errors
    /// Returns an error if the target directory already exists or file operations fail.
    pub fn generate(&self, target_dir: &Path) -> anyhow::Result<()> {
        let project_dir = target_dir.join(&self.project_name);

        if project_dir.exists() {
            anyhow::bail!("Directory '{}' already exists", project_dir.display());
        }

        fs::create_dir_all(&project_dir)?;
        fs::create_dir_all(project_dir.join("src"))?;

        let context = TemplateContext {
            project_name: &self.project_name,
            template_type: self.template_type,
        };

        let cargo_toml = templates::render_cargo_toml(&context)?;
        fs::write(project_dir.join("Cargo.toml"), cargo_toml)?;

        let main_rs = templates::render_main_rs(&context)?;
        fs::write(project_dir.join("src/main.rs"), main_rs)?;

        Ok(())
    }

    #[must_use]
    pub fn project_name(&self) -> &str {
        &self.project_name
    }

    #[must_use]
    pub const fn template_type(&self) -> TemplateType {
        self.template_type
    }
}

pub struct TemplateContext<'a> {
    pub project_name: &'a str,
    pub template_type: TemplateType,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_type_conversion() {
        assert_eq!(TemplateType::parse("basic"), Some(TemplateType::Basic));
        assert_eq!(TemplateType::parse("counter"), Some(TemplateType::Counter));
        assert_eq!(TemplateType::parse("todo-app"), Some(TemplateType::TodoApp));
        assert_eq!(TemplateType::parse("invalid"), None);
    }

    #[test]
    fn test_template_type_as_str() {
        assert_eq!(TemplateType::Basic.as_str(), "basic");
        assert_eq!(TemplateType::Counter.as_str(), "counter");
        assert_eq!(TemplateType::TodoApp.as_str(), "todo-app");
    }

    #[test]
    fn test_generator_basic() {
        let temp_dir = tempfile::tempdir().unwrap();
        let generator = ProjectGenerator::new("test-project", TemplateType::Basic);

        generator.generate(temp_dir.path()).unwrap();

        assert!(temp_dir.path().join("test-project").exists());
        assert!(temp_dir.path().join("test-project/Cargo.toml").exists());
        assert!(temp_dir.path().join("test-project/src/main.rs").exists());
    }

    #[test]
    fn test_generator_rejects_existing_dir() {
        let temp_dir = tempfile::tempdir().unwrap();
        let project_dir = temp_dir.path().join("existing-project");
        fs::create_dir_all(&project_dir).unwrap();

        let generator = ProjectGenerator::new("existing-project", TemplateType::Basic);

        assert!(generator.generate(temp_dir.path()).is_err());
    }

    #[test]
    fn test_cargo_toml_content() {
        let context = TemplateContext {
            project_name: "my-app",
            template_type: TemplateType::Basic,
        };

        let content = templates::render_cargo_toml(&context).unwrap();
        assert!(content.contains("name = \"my-app\""));
        assert!(content.contains("ctui"));
    }

    #[test]
    fn test_main_rs_content() {
        let context = TemplateContext {
            project_name: "my-app",
            template_type: TemplateType::Counter,
        };

        let content = templates::render_main_rs(&context).unwrap();
        assert!(content.contains("Counter"));
    }
}
