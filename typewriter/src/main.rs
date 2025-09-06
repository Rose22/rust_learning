/* 
 * my classic typewriter example
 * that i always use to get used to new coding languages
 */

use std::{io, io::Write, env, fs, thread, time};

fn type_it(content: &str) {
    for char in content.chars() {
        // print the char
        print!("{}", char);
        // flush stdout so that it actually shows up
        io::stdout().flush().expect("ERROR: failed to flush?!");
        // sleep for a short amount of time
        thread::sleep(time::Duration::from_millis(10));
    }
}

fn main() {
    // get commandline arguments for use
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("usage: {} <filename>", &args[0]);
        std::process::exit(0);
    }

    // read the target file into a string
    let str_to_be_typed = fs::read_to_string(&args[1])
        .unwrap_or_else(|e| {
            /*
             * unwrap_or_else extracts the value from the returned Result
             * if the function succeeded. but otherwise, it runs the code
             * inside this statement
             */
            println!("could not read file! reason: {e}");
            std::process::exit(1);
        }
    );

    // actually do the typing
    type_it(&str_to_be_typed);
}
