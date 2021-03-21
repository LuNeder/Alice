use std::env;
use text_io::scan;
use std::fs;

fn main() {
    println!("Not ready yet. Use branch alice-python instead.");
    // Get command line input and save it as an args variable
    let args: String = env::args().collect();

    // Checks if the mode selected in command line was help and runs it or not
    if args.contains("help") {

        alice_help(); // Help mode

    } else {

        // Asks how many seconds between lines and save it as a t variable
        println!("How many seconds between lines?");
        let t: i32;
        scan!("{}", t);
        // Prints the t variable
        println!("{} seconds between lines", t);

        //
        

        // Checks selected mode in command line and runs it
        if args.contains("loop") {alice_loop(t);} // Loop mode
        else {alice(t)}; // Regular mode

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
