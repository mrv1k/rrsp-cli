// use clap::Parser;

// #[derive(Parser)]
// struct CLI {
//     pattern: String,
//     path: std::path::PathBuf
// }

// fn main() {
//     let args = CLI::parse();
//     let content = std::fs::read_to_string(&args.path).expect("could not read file");

//     for line in content.lines() {
//         if line.contains(&args.pattern) {
//             println!("{}", line);
//         }
//     }
// }

#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<(), CustomError> {
    let path = "mek.mek";
    let content = std::fs::read_to_string(path)
        .map_err(|err| CustomError(format!("Error reading `{}`: {}", path, err)))?;
    // println!("{}", content);
    println!("file content: {}", content);
return Ok(());
}

// #[derive(Debug)]
// struct CustomError(String);

// fn main() -> Result<(), CustomError> {
//     let path = "test.txt";
//     let content = std::fs::read_to_string(path)
//         .map_err(|err| CustomError(format!("Error reading `{}`: {}", path, err)))?;
//     println!("file content: {}", content);
//     Ok(())
// }