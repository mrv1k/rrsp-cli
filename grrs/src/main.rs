use clap::Parser;

#[derive(Parser)]
struct CLI {
    pattern: String,
    path: std::path::PathBuf
}

fn main() {
    let args = CLI::parse();
    println!("{}", args.pattern);
    println!("{}", args.path.display());
}
