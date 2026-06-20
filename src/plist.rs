use std::fs;
use std::path::PathBuf;

use anyhow::Result;

pub const LABEL: &str = "io.github.candy12t.kanade";
const TEMPLATE: &str = include_str!("../templates/agent.plist");

pub fn plist_path() -> Result<PathBuf> {
    let home = std::env::var("HOME")?;
    Ok(PathBuf::from(home)
        .join("Library")
        .join("LaunchAgents")
        .join(format!("{LABEL}.plist")))
}

pub fn render(exec: &str, log: &str) -> String {
    TEMPLATE
        .replace("{label}", LABEL)
        .replace("{exec}", exec)
        .replace("{log}", log)
}

pub fn write(content: &str) -> Result<()> {
    let path = plist_path()?;
    fs::write(path, content)?;
    Ok(())
}
pub fn remove() -> Result<()> {
    match fs::remove_file(plist_path()?) {
        Ok(()) => Ok(()),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(err) => Err(err.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_replaces_all_placeholders() {
        let out = render("/usr/local/bin/kanade", "/tmp/kanade.log");
        assert!(out.contains(LABEL));
        assert!(out.contains("/usr/local/bin/kanade"));
        assert!(out.contains("/tmp/kanade.log"));
    }

    #[test]
    fn render_leaves_no_placeholders() {
        let out = render("/usr/local/bin/kanade", "/tmp/kanade.log");
        assert!(!out.contains("{label}"));
        assert!(!out.contains("{exec}"));
        assert!(!out.contains("{log}"));
    }
}
