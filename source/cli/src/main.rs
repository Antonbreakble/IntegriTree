use clap::Parser;
use integritree_cli::{run, Args};

fn main() {
    let args = Args::parse();

    match run(args) {
        Ok(message) => println!("{message}"),
        Err(error) => {
            eprintln!("Ошибка: {error}");
            std::process::exit(1);
        }
    }
}