use chrono::Utc;
use codex_utils_home_dir::find_codex_home;
use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use tracing::warn;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExperienceItem {
    pub id: String,
    pub title: String,
    pub status: String, // "ongoing" | "completed" | "expired"
    pub start_date: String, // "YYYY-MM-DD"
    pub end_date: String,   // "YYYY-MM-DD"
    pub summary: String,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ExperienceStore;

impl ExperienceStore {
    pub fn experiences_file_path() -> std::io::Result<PathBuf> {
        let home = find_codex_home()?;
        Ok(home.join("experiences.json").to_path_buf())
    }

    /// Load experiences from `~/.dopax/experiences.json` and auto-purge expired ones.
    pub fn load_and_purge() -> Vec<ExperienceItem> {
        let path = match Self::experiences_file_path() {
            Ok(p) => p,
            Err(err) => {
                warn!("failed to resolve dopax home for experiences: {err}");
                return Vec::new();
            }
        };

        if !path.exists() {
            return Vec::new();
        }

        let contents = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(err) => {
                warn!("failed to read experiences.json: {err}");
                return Vec::new();
            }
        };

        let items: Vec<ExperienceItem> = match serde_json::from_str(&contents) {
            Ok(items) => items,
            Err(err) => {
                warn!("failed to parse experiences.json: {err}");
                return Vec::new();
            }
        };

        let today = Utc::now().naive_utc().date().format("%Y-%m-%d").to_string();

        let mut changed = false;
        let mut active_items = Vec::new();

        for mut item in items {
            // Auto-expiration check: if end_date < today or status == "completed"/"expired", mark or filter.
            if item.status == "ongoing" && !item.end_date.is_empty() && item.end_date < today {
                item.status = "expired".to_string();
                changed = true;
            }

            if item.status == "ongoing" {
                active_items.push(item);
            } else {
                changed = true;
            }
        }

        if changed {
            if let Err(err) = Self::save_all(&path, &active_items) {
                warn!("failed to save purged experiences: {err}");
            }
        }

        active_items
    }

    pub fn save_all(path: &PathBuf, items: &[ExperienceItem]) -> std::io::Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(items)?;
        fs::write(path, json)?;
        Ok(())
    }

    pub fn add(item: ExperienceItem) -> std::io::Result<ExperienceItem> {
        let path = Self::experiences_file_path()?;
        let mut items = Self::load_and_purge();
        items.retain(|i| i.id != item.id);
        items.push(item.clone());
        Self::save_all(&path, &items)?;
        Ok(item)
    }

    pub fn update(item: ExperienceItem) -> std::io::Result<ExperienceItem> {
        Self::add(item)
    }

    pub fn delete(id: &str) -> std::io::Result<bool> {
        let path = Self::experiences_file_path()?;
        let mut items = Self::load_and_purge();
        let len_before = items.len();
        items.retain(|i| i.id != id);
        let removed = items.len() < len_before;
        if removed {
            Self::save_all(&path, &items)?;
        }
        Ok(removed)
    }

    pub fn format_developer_context() -> String {
        let items = Self::load_and_purge();
        let today = Utc::now().naive_utc().date().format("%Y-%m-%d").to_string();

        if items.is_empty() {
            return format!("<current_time>\nToday: {today}\n</current_time>");
        }

        let mut lines = Vec::new();
        lines.push(format!("<current_time>\nToday: {today}\n</current_time>"));
        lines.push("<active_experiences>".to_string());
        for item in items {
            lines.push(format!(
                "- [{}] {} ({} ~ {}): {}\n  Summary: {}",
                item.id, item.title, item.start_date, item.end_date, item.status, item.summary
            ));
        }
        lines.push("</active_experiences>".to_string());
        lines.join("\n")
    }
}
