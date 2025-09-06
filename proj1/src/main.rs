fn first_func(arg: &str) -> bool {
    println!("omg its a rusty function! and the argument was: {}", arg);
    return true;
}

fn main() {
    // simple variable demo
    let name = "Rosa";

    println!("Hello, world!");
    println!("My name is {}", name);

    /*
    loop {
        println!("This is like while (true), but rustier!");
    }
    */

    /*
    while true {
        println!("This is actually while true!");
    }
    */

    // for loop
    for i in 1..10 {
        println!("For loops are pretty simple. count is {}", i);
    }

    // functions!
    let success = first_func("rustiness");
    println!("{}", success);

    // strings are weird in rust..
    let mystr = String::from("test");
    println!("{}", mystr);

    // immutable array!
    let my_arr = ["sushi", "tomato soup", "sweet potato fries", "carrots", "cheese"];
    for food in my_arr {
        println!("{}", food);
    }

    println!("{:?}", my_arr);

    let mut my_vec = vec!["this", "array", "can", "be"];
    my_vec.push("modified");

    // did, you, know
    my_vec.insert(0, "know");
    my_vec.insert(0, "you");
    my_vec.insert(0, "did");

    for word in my_vec {
        print!("{} ", word);
        print!("\n");
    }
}
