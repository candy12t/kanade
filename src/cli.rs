use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::{listener, plist};

const LOG_PATH: &str = "/tmp/kanade.log";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run in foreground. Ctrl-C to exit
    Run,
    /// Install and start the launchd agent (auto-start at login)
    Install,
    /// Stop and remove the launchd agent
    Uninstall,
    /// Restart the launchd agent (after an update or granting permission)
    Restart,
    /// Show permission and agent
    Status,
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Run => listener::listen(),
        Commands::Install => install(),
        Commands::Uninstall => uninstall(),
        Commands::Restart => Ok(()),
        Commands::Status => Ok(()),
    }
}

fn install() -> Result<()> {
    let exec = std::env::current_exe()?;
    let content = plist::render(&exec.to_string_lossy(), LOG_PATH);
    plist::write(&content)?;

    println!("kanade: installed and started");
    Ok(())
}

fn uninstall() -> Result<()> {
    plist::remove()?;

    println!("kanade: uninstalled");
    Ok(())
}
