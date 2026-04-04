use super::{Component, ComponentData};
use crate::config::types::ComponentId;
use crate::core::input::{InputData, TranscriptEntry};
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

const DEFAULT_CONTEXT_LIMIT: u32 = 200_000;

#[derive(Default)]
pub struct ContextWindowComponent;

impl ContextWindowComponent {
    pub fn new() -> Self {
        Self
    }

    fn get_context_limit(model_id: &str) -> u32 {
        // Simple heuristic based on model ID patterns
        if model_id.contains("1m") || model_id.contains("1000k") {
            1_000_000
        } else {
            DEFAULT_CONTEXT_LIMIT
        }
    }
}

impl Component for ContextWindowComponent {
    fn collect(&self, input: &InputData) -> Option<ComponentData> {
        let context_limit = Self::get_context_limit(&input.model.id);
        let tokens_opt = parse_transcript_usage(&input.transcript_path);

        let (pct_display, tok_display) = match tokens_opt {
            Some(tokens) => {
                let rate = (tokens as f64 / context_limit as f64) * 100.0;
                let pct = if rate.fract() == 0.0 {
                    format!("{:.0}%", rate)
                } else {
                    format!("{:.1}%", rate)
                };
                let tok = if tokens >= 1000 {
                    let k = tokens as f64 / 1000.0;
                    if k.fract() == 0.0 {
                        format!("{}k", k as u32)
                    } else {
                        format!("{:.1}k", k)
                    }
                } else {
                    tokens.to_string()
                };
                (pct, tok)
            }
            None => ("-".into(), "-".into()),
        };

        let mut metadata = HashMap::new();
        if let Some(tokens) = tokens_opt {
            let rate = (tokens as f64 / context_limit as f64) * 100.0;
            metadata.insert("tokens".into(), tokens.to_string());
            metadata.insert("percentage".into(), rate.to_string());
        } else {
            metadata.insert("tokens".into(), "-".into());
            metadata.insert("percentage".into(), "-".into());
        }
        metadata.insert("limit".into(), context_limit.to_string());

        Some(ComponentData {
            primary: format!("{} \u{b7} {} tokens", pct_display, tok_display),
            secondary: String::new(),
            metadata,
        })
    }

    fn id(&self) -> ComponentId {
        ComponentId::ContextWindow
    }
}

fn parse_transcript_usage(transcript_path: &str) -> Option<u32> {
    let path = Path::new(transcript_path);

    if let Some(usage) = try_parse_transcript_file(path) {
        return Some(usage);
    }

    if !path.exists()
        && let Some(usage) = try_find_from_project_history(path)
    {
        return Some(usage);
    }

    None
}

fn try_parse_transcript_file(path: &Path) -> Option<u32> {
    let file = fs::File::open(path).ok()?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .unwrap_or_default();

    if lines.is_empty() {
        return None;
    }

    // Check if last line is a summary
    let last_line = lines.last()?.trim();
    if let Ok(entry) = serde_json::from_str::<TranscriptEntry>(last_line)
        && entry.r#type.as_deref() == Some("summary")
        && let Some(leaf_uuid) = &entry.leaf_uuid
    {
        let project_dir = path.parent()?;
        return find_usage_by_leaf_uuid(leaf_uuid, project_dir);
    }

    // Find last assistant message
    for line in lines.iter().rev() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Ok(entry) = serde_json::from_str::<TranscriptEntry>(line)
            && entry.r#type.as_deref() == Some("assistant")
            && let Some(message) = &entry.message
            && let Some(raw_usage) = &message.usage
        {
            return Some(raw_usage.clone().normalize().display_tokens());
        }
    }

    None
}

fn find_usage_by_leaf_uuid(leaf_uuid: &str, project_dir: &Path) -> Option<u32> {
    for entry in fs::read_dir(project_dir).ok()? {
        let path = entry.ok()?.path();
        if path.extension().and_then(|s| s.to_str()) != Some("jsonl") {
            continue;
        }
        if let Some(usage) = search_uuid_in_file(&path, leaf_uuid) {
            return Some(usage);
        }
    }
    None
}

fn search_uuid_in_file(path: &Path, target_uuid: &str) -> Option<u32> {
    let file = fs::File::open(path).ok()?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .unwrap_or_default();

    for line in &lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Ok(entry) = serde_json::from_str::<TranscriptEntry>(line)
            && entry.uuid.as_deref() == Some(target_uuid)
        {
            if entry.r#type.as_deref() == Some("assistant") {
                if let Some(message) = &entry.message
                    && let Some(raw_usage) = &message.usage
                {
                    return Some(raw_usage.clone().normalize().display_tokens());
                }
            } else if entry.r#type.as_deref() == Some("user")
                && let Some(parent_uuid) = &entry.parent_uuid
            {
                return find_assistant_by_uuid(&lines, parent_uuid);
            }
            break;
        }
    }
    None
}

fn find_assistant_by_uuid(lines: &[String], target_uuid: &str) -> Option<u32> {
    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Ok(entry) = serde_json::from_str::<TranscriptEntry>(line)
            && entry.uuid.as_deref() == Some(target_uuid)
            && entry.r#type.as_deref() == Some("assistant")
            && let Some(message) = &entry.message
            && let Some(raw_usage) = &message.usage
        {
            return Some(raw_usage.clone().normalize().display_tokens());
        }
    }
    None
}

fn try_find_from_project_history(transcript_path: &Path) -> Option<u32> {
    let project_dir = transcript_path.parent()?;
    let mut session_files: Vec<PathBuf> = Vec::new();

    for entry in fs::read_dir(project_dir).ok()? {
        let path = entry.ok()?.path();
        if path.extension().and_then(|s| s.to_str()) == Some("jsonl") {
            session_files.push(path);
        }
    }

    session_files.sort_by_key(|p| {
        fs::metadata(p)
            .and_then(|m| m.modified())
            .unwrap_or(std::time::UNIX_EPOCH)
    });
    session_files.reverse();

    for session_path in &session_files {
        if let Some(usage) = try_parse_transcript_file(session_path) {
            return Some(usage);
        }
    }
    None
}
