use clap::Parser;
use std::path::PathBuf;
use std::process;

mod app;
mod file_tree;
mod markdown;
mod state;
mod style;
mod widgets;

#[derive(Parser)]
#[command(name = "readmd", about = "TUI markdown viewer")]
struct Cli {
    /// Directory to browse (defaults to current directory)
    #[arg(default_value = ".")]
    path: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    let path = match cli.path.canonicalize() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error resolving path '{}': {}", cli.path.display(), e);
            process::exit(1);
        }
    };

    let mut state = state::AppState::new(&path);

    if let Err(e) = app::run(&mut state) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
