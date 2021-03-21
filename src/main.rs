use std::env;
use text_io::scan;

fn main() {
    println!("Not ready yet. Use branch alice-python instead.");
    // Get command line input and save it as an args variable
    let args: String = env::args().collect();
    // Checks what mode was asked for in command line and runs it
    if args.contains("help") {
        alice_help(); // Help mode
    } else if args.contains("loop") {
        // Asks how many seconds between lines and save it as a t variable
        println!("How many seconds between lines?");
        let t: i32;
        scan!("{}", t);
        // Prints the t variable
        println!("{} seconds between lines", t);
        alice_loop(); // Loop mode
    } else {
        // Asks how many seconds between lines and save it as a t variable
        println!("How many seconds between lines?");
        let t: i32;
        scan!("{}", t);
        // Prints the t variable
        println!("{} seconds between lines", t);
        alice(); // Regular mode
    };
}

// Help mode
fn alice_help() {
    println!("help");
}

// Loop mode
fn alice_loop() {
    println!("loop");
}

//Regular mode
fn alice() {
    println!("alice");
}
