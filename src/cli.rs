use clap::{Error, Parser, Subcommand};

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

pub fn run() -> Result<(), Error> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Run => Ok(()),
        Commands::Install => Ok(()),
        Commands::Uninstall => Ok(()),
        Commands::Restart => Ok(()),
        Commands::Status => Ok(()),
    }
}
