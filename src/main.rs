mod cli;
mod key_emitter;
mod listener;
mod permission;
mod tap_resolver;

fn main() {
    if let Err(err) = cli::run() {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}
