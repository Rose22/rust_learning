use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    // check if enough args
    if args.len() < 2 {
        println!("usage: {0} <path>", args[0]);
        process::exit(1);
    }

    // get dir contents
    let dir = fs::read_dir(&args[1]).unwrap_or_else(|e| {
        println!("ERROR: {e}");
        process::exit(1);
    });

    // list files in dir
    for file in dir {
        let file = file.expect("error reading file");
        println!("{}", file.file_name().into_string().expect("error getting filename"));
    }
}
