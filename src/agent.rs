use std::path::Path;
use std::process::{Command, Stdio};

use anyhow::{Result, anyhow};

use crate::plist;

unsafe extern "C" {
    fn getuid() -> u32;
}

fn uid() -> u32 {
    unsafe { getuid() }
}

fn domain_target() -> String {
    format!("gui/{}", uid())
}

fn service_target() -> String {
    format!("gui/{}/{}", uid(), plist::LABEL)
}

pub fn bootstrap(plist_path: &Path) -> Result<()> {
    let domain = domain_target();
    let path = plist_path.to_string_lossy();
    run("bootstrap", &[domain.as_str(), path.as_ref()])
}

pub fn bootout() -> Result<()> {
    let target = service_target();
    run("bootout", &[target.as_str()])
}

pub fn kickstart() -> Result<()> {
    let target = service_target();
    run("kickstart", &["-k", target.as_str()])
}

pub fn is_loaded() -> bool {
    Command::new("launchctl")
        .arg("print")
        .arg(service_target())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn run(subcommand: &str, args: &[&str]) -> Result<()> {
    let output = Command::new("launchctl")
        .arg(subcommand)
        .args(args)
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(anyhow!(format!(
            "launchctl {subcommand} failed: {}",
            stderr.trim()
        )))
    }
}

pub enum State {
    NotInstalled,
    Running { pid: Option<u32> },
    NotRunning,
}

pub fn state() -> State {
    let output = Command::new("launchctl")
        .arg("print")
        .arg(service_target())
        .output();
    let output = match output {
        Ok(output) if output.status.success() => output,
        _ => return State::NotInstalled,
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    if stdout.lines().any(|line| line.trim() == "state = running") {
        let pid = stdout
            .lines()
            .find_map(|line| line.trim().strip_prefix("pid = "))
            .and_then(|value| value.trim().parse().ok());
        State::Running { pid }
    } else {
        State::NotRunning
    }
}
