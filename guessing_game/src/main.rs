use std::io;
use std::io::Write;
use rand::Rng;

fn main() {
    // wtf is this mess
    let random_num = rand::rng().random_range(1..=100);

    /*
     * println automatically adds a newline, print does not
     */
    /*
     * println!("hint: number is {random_num}");
     * print!("\n");
     */

    println!("Go ahead, guess!");

    loop {
        // create mutable string (rust defaults to immutable vars)
        let mut guess = String::new();

        print!("guess> ");
        // make it actually print onscreen, since we dont use a newline
        io::stdout().flush().expect("couldnt flush for some reason?!");

        io::stdin().read_line(&mut guess)
            .expect("could not ask for input for some reason??") // error handler
            // why would an OS ever fail to ask for user input?
        ;

        /* 
         * convert to int (i32). seems we have i32 and i64, according to how many bits to store
         * it goes from 8 bits to 128 bits.. seems we have some memory saving potential!
         * changed it to i8 because less memory use
         */
        let guess: i8 = guess.trim().parse().expect("please type a number, silly!");

        if guess == random_num {
            println!("congrats! you got it! yayay!");
            break;
        }
        else if guess < random_num { println!("too small"); }
        else if guess > random_num { println!("too big"); }

        print!("\n");
    }
}
