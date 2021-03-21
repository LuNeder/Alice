use std::env;
use text_io::scan;
use std::fs;

fn main() {
    println!("Not ready yet. Use branch alice-python instead.");
    // Get command line input and save it as an args variable
    let args: String = env::args().collect();
    // Checks what mode was asked for in command line and runs it
    if args.contains("help") {
        alice_help(); // Help mode
    } else {
        // Asks how many seconds between lines and save it as a t variable
        println!("How many seconds between lines?");
        let t: i32;
        scan!("{}", t);
        // Prints the t variable
        println!("{} seconds between lines", t);
        if args.contains("loop") {alice_loop(t);}
        else {alice(t)};

    };
}

// Help mode
fn alice_help() {
    println!("help");
}

// Loop mode
fn alice_loop(t: i32) {
    println!("loop ({} seconds between lines)", t);
}

//Regular mode
fn alice(t: i32) {
    println!("alice ({} seconds between lines)", t);
}
