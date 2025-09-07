use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("usage: {0} <path>", args[0]);
        process::exit(1);
    }

    for file in fs::read_dir(&args[1]).expect("invalid directory") {
        let file = file.unwrap();

        println!("{}", file.file_name().into_string().unwrap());
    }
}
