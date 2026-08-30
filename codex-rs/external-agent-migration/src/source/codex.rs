use super::InstructionSourceGroup;
use super::is_non_empty_text_file;
use crate::RewriteProfile;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

pub struct CodexSource;

impl CodexSource {
    pub const CONFIG_DIR: &'static str = ".codex";
    pub const MIGRATION_SOURCE: &'static str = "codex";
    pub const SETTINGS_FILE: &'static str = "config.toml";
    pub const CONFIG_MD: &'static str = "AGENTS.md";
    // Dopax is codex-compatible, so no term rewriting is needed: the doc file
    // name already matches AGENTS.md and there are no source-specific product
    // terms to translate.
    pub const REWRITE_PROFILE: RewriteProfile = RewriteProfile::new(Self::CONFIG_MD, &[]);

    pub fn repo_instruction_source_groups(
        repo_root: &Path,
    ) -> io::Result<Vec<InstructionSourceGroup>> {
        for candidate in [
            repo_root.join(Self::CONFIG_MD),
            repo_root.join(Self::CONFIG_DIR).join(Self::CONFIG_MD),
        ] {
            if is_non_empty_text_file(&candidate)? {
                return Ok(vec![InstructionSourceGroup {
                    scope: repo_root.to_path_buf(),
                    sources: vec![candidate],
                }]);
            }
        }
        Ok(Vec::new())
    }

    pub fn home_instruction_sources(external_agent_home: &Path) -> io::Result<Vec<PathBuf>> {
        let path = external_agent_home.join(Self::CONFIG_MD);
        Ok(is_non_empty_text_file(&path)?
            .then_some(path)
            .into_iter()
            .collect())
    }

    pub fn read_instruction_source(path: &Path) -> io::Result<String> {
        fs::read_to_string(path)
    }
}

#[cfg(test)]
#[path = "codex_tests.rs"]
mod tests;
