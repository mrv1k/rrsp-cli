use clap::Parser;
use anyhow::{Context, Result};

#[derive(Parser)]
struct CLI {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = CLI::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    grrs::find_matches(&content, &args.pattern, &mut std::io::stdout());

    Ok(())
}


