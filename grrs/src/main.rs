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

use indicatif;


fn main() {
    let pb = indicatif::ProgressBar::new(100);
    for i in 0..100 {
        // doMekMek
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");
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