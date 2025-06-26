use anyhow::{Context, Result};
use clap::Parser;
use std::env;
use std::path::PathBuf;

use gittop::App;

#[derive(Parser)]
#[command(name = "gittop")]
#[command(about = "A real-time Git repository monitoring tool", long_about = None)]
#[command(version)]
struct Cli {
    /// Path to the Git repository (defaults to current directory)
    #[arg(value_name = "PATH")]
    path: Option<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    let repo_path = match cli.path {
        Some(path) => path,
        None => env::current_dir().context("Failed to get current directory")?,
    };

    let mut app = App::new(repo_path)
        .context("Failed to initialize application")?;
    
    app.run()
        .context("Application error")?;

    Ok(())
}
