use super::*;
use crate::InstructionSourceGroup;
use std::io::Write;
use tempfile::TempDir;

#[test]
fn codex_config_consts_point_at_codex_home() {
    assert_eq!(CodexSource::CONFIG_DIR, ".codex");
    assert_eq!(CodexSource::MIGRATION_SOURCE, "codex");
    assert_eq!(CodexSource::SETTINGS_FILE, "config.toml");
    assert_eq!(CodexSource::CONFIG_MD, "AGENTS.md");
}

#[test]
fn codex_rewrite_profile_is_noop_for_agents_md() {
    // Dopax is codex-compatible: the doc file name already matches AGENTS.md
    // and there are no product terms to rewrite.
    let profile = CodexSource::REWRITE_PROFILE;
    assert_eq!(profile.doc_file_name(), "AGENTS.md");
    assert!(profile.term_variants().is_empty());
    let content = "see AGENTS.md for the Codex workflow";
    assert_eq!(profile.rewrite(content), content);
}

#[test]
fn repo_instruction_source_groups_finds_agents_md_in_repo_root() {
    let root = TempDir::new().expect("tempdir");
    fs::write(root.path().join("AGENTS.md"), "# repo instructions").expect("write AGENTS.md");
    assert_eq!(
        CodexSource::repo_instruction_source_groups(root.path()).expect("groups"),
        vec![InstructionSourceGroup {
            scope: root.path().to_path_buf(),
            sources: vec![root.path().join("AGENTS.md")],
        }]
    );
}

#[test]
fn repo_instruction_source_groups_finds_agents_md_in_dot_codex() {
    let root = TempDir::new().expect("tempdir");
    let codex_dir = root.path().join(".codex");
    fs::create_dir_all(&codex_dir).expect("create .codex");
    fs::write(codex_dir.join("AGENTS.md"), "# project instructions").expect("write AGENTS.md");
    assert_eq!(
        CodexSource::repo_instruction_source_groups(root.path()).expect("groups"),
        vec![InstructionSourceGroup {
            scope: root.path().to_path_buf(),
            sources: vec![codex_dir.join("AGENTS.md")],
        }]
    );
}

#[test]
fn repo_instruction_source_groups_prefers_repo_root_agents_md() {
    let root = TempDir::new().expect("tempdir");
    let codex_dir = root.path().join(".codex");
    fs::create_dir_all(&codex_dir).expect("create .codex");
    fs::write(root.path().join("AGENTS.md"), "# repo instructions").expect("write repo AGENTS.md");
    fs::write(codex_dir.join("AGENTS.md"), "# project instructions")
        .expect("write .codex AGENTS.md");
    assert_eq!(
        CodexSource::repo_instruction_source_groups(root.path()).expect("groups"),
        vec![InstructionSourceGroup {
            scope: root.path().to_path_buf(),
            sources: vec![root.path().join("AGENTS.md")],
        }]
    );
}

#[test]
fn repo_instruction_source_groups_returns_empty_without_agents_md() {
    let root = TempDir::new().expect("tempdir");
    assert_eq!(
        CodexSource::repo_instruction_source_groups(root.path()).expect("groups"),
        Vec::<InstructionSourceGroup>::new()
    );
}

#[test]
fn home_instruction_sources_finds_home_agents_md() {
    let root = TempDir::new().expect("tempdir");
    fs::write(root.path().join("AGENTS.md"), "# home instructions").expect("write AGENTS.md");
    assert_eq!(
        CodexSource::home_instruction_sources(root.path()).expect("home sources"),
        vec![root.path().join("AGENTS.md")]
    );
}

#[test]
fn home_instruction_sources_skips_empty_agents_md() {
    let root = TempDir::new().expect("tempdir");
    fs::write(root.path().join("AGENTS.md"), "").expect("write empty AGENTS.md");
    assert_eq!(
        CodexSource::home_instruction_sources(root.path()).expect("home sources"),
        Vec::<PathBuf>::new()
    );
}

#[test]
fn read_instruction_source_reads_file() {
    let root = TempDir::new().expect("tempdir");
    let path = root.path().join("AGENTS.md");
    let mut file = fs::File::create(&path).expect("create file");
    file.write_all(b"# instructions").expect("write");
    assert_eq!(
        CodexSource::read_instruction_source(&path).expect("read"),
        "# instructions"
    );
}
