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

    find_matches(&content, &args.pattern, &mut std::io::stdout());

    Ok(())
}

fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line);
        }
    }
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("mek\n", "mek", &mut result);
    assert_eq!(result, b"mek\n");

    // find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    // assert_eq!(result, b"lorem ipsum\n");
}
