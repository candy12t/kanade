use anyhow::{Result, anyhow};
use clap::{Parser, Subcommand};

use crate::{agent, listener, plist};

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
    /// Show the launchd agent staus
    Status,
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Run => listener::listen(),
        Commands::Install => install(),
        Commands::Uninstall => uninstall(),
        Commands::Restart => restart(),
        Commands::Status => status(),
    }
}

fn install() -> Result<()> {
    let exec = std::env::current_exe()?;
    let content = plist::render(&exec.to_string_lossy(), LOG_PATH);
    plist::write(&content)?;

    if agent::is_loaded() {
        agent::bootout()?;
    }
    agent::bootstrap(&plist::plist_path()?)?;

    println!("kanade: installed and started");
    Ok(())
}

fn uninstall() -> Result<()> {
    if agent::is_loaded() {
        agent::bootout()?;
    }
    plist::remove()?;

    println!("kanade: uninstalled");
    Ok(())
}

fn restart() -> Result<()> {
    if !agent::is_loaded() {
        return Err(anyhow!("not installed. run `kanade install` first"));
    }
    agent::kickstart()?;

    println!("kanade: restarted");
    Ok(())
}

fn status() -> Result<()> {
    match agent::state() {
        agent::State::Running { pid } => match pid {
            Some(pid) => println!("launchd agent: running (pid {pid})"),
            None => println!("launchd agent: running"),
        },
        agent::State::NotRunning => {
            println!("launchd agent: loaded (not running)");
        }
        agent::State::NotInstalled => {
            println!("launchd agent: not installed");
        }
    }
    Ok(())
}
