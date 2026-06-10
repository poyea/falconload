use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use super::types::Config;

pub fn load_config(path: &str) -> io::Result<Config> {
    let resolved = resolve_config_path(path)?;
    let content = fs::read_to_string(resolved)?;
    toml::from_str(&content).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

fn resolve_config_path(path: &str) -> io::Result<PathBuf> {
    let trimmed = path.trim();
    let unquoted = trimmed.trim_matches(|c| c == '\'' || c == '"');
    let candidate = PathBuf::from(unquoted);

    if candidate.is_file() {
        return Ok(candidate);
    }

    // Also check the config/ directory
    let from_config_dir = Path::new("config").join(&candidate);
    if candidate.parent().is_none() {
        if from_config_dir.is_file() {
            return Ok(from_config_dir);
        }
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        format!(
            "Config file not found. Tried '{}' and '{}'.",
            candidate.display(),
            from_config_dir.display()
        ),
    ))
}
