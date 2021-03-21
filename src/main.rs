// main.rs
// Copyright 2020~2021 Lu Neder
//
// This work may be distributed and/or modified under the
// conditions of the LaTeX Project Public License, either version 1.3
// of this license or (at your option) any later version.
// The latest version of this license is in
//   http://www.latex-project.org/lppl.txt
// and version 1.3 or later is part of all distributions of LaTeX
// version 2005/12/01 or later.
//
// This work has the LPPL maintenance status `maintained'.
//
// The Current Maintainer of this work is Lu Neder.
//
// This work consists of the files main.rs, README.md and the included original.txt example.

//Alice requires an original.txt file on the directory you're running it from.
//Alice will read the file and then type it (a line followed by an enter, then the next line).
//This way you can send the file contnent on a chat, for example, each line on a message.
//You can also tell Alice to wait some time between a line and the other.
//The first line will be written 10 seconds after you run Alice and tell the time between a line and the other,
//this way you have time to select the place where you want to input your text.
//"Alice" was just the 1st name I found on Google lol

use std::env;
use text_io::scan;
use std::fs;
use std::str;
use std::{thread, time};
use simulate;
use simulate::Key;
use chrono::prelude::*;

fn main() {
    let ver: String = "3.0.0".to_string(); //sets ver vaiable to current Alice version

    // Get command line input and save it as an args variable
    let args: String = env::args().collect();

    // Checks if the mode selected in command line was help and runs it or not
    if args.contains("help") {

        alice_help(ver); // Help mode

    } else {

        // Asks how many seconds between lines and save it as a t variable
        println!("How many seconds between lines?");
        let t: i32;
        scan!("{}", t);
        // Prints the t variable
        println!("{} seconds between lines", t);

        // Open and divide original.txt by lines
        let file = "original.txt";
        let open = fs::read_to_string(file)
            .expect("Something went wrong reading the file");
        let mut divided: Vec<&str> = open.lines().collect();



        // Checks selected mode in command line and runs it
        if args.contains("loop") {alice_loop(t, divided);} // Loop mode
        else {alice(t, divided)}; // Regular mode

    };
}

// Help mode, prints help and the version ver
fn alice_help(ver: String) {
    println!("Alice {} Help", ver);
    println!("--help: Help mode: show this message");
    println!("--loop: Loop mode: restart the file from beginning after it's over");
    println!("No args: Normal mode: stop once file ends");
}

// Loop mode
fn alice_loop(t: i32, divided: Vec<&str>) {

    println!("loop ({} seconds between lines)", t);

    // Waits 10 seconds before running
    let tensec = time::Duration::from_secs(10);
    thread::sleep(tensec);

    // Loops Alice
    loop {
        let mut divided2 = divided.clone(); // Clones the divided variable because the program won't compile otherwise
        alice(t, divided2); // run Alice regular mode, inside the loop
    }
}

//Regular mode
fn alice(t: i32, divided: Vec<&str>) {

    // If the args contains loop, does not wait 10 seconds since alice_loop() alreadi did that. If it's regular mode, waits 10 seconds
    let args: String = env::args().collect();
    if args.contains("loop") {println!("LOOP")} else {
        println!("alice ({} seconds between lines)", t);
        let tensec = time::Duration::from_secs(10);
        thread::sleep(tensec);
    }

    // Converts t into u64
    let tttt = t as u64;

    // Repeats with each line of the file
    for i in divided {

        let halfsec = time::Duration::from_millis(500); // Set halfsec variable to 500 ms

        simulate::type_str(i).unwrap(); // types the line

        thread::sleep(halfsec); // Waits halfsec (500 ms)

        simulate::send(Key::Enter).unwrap(); // Press enter

        let now = Local::now();
        println!("{} - {:#?}", i, now); // prints in terminal the linee that was just typed and the time it was sent

        let z = time::Duration::from_secs(tttt);
        thread::sleep(z); // Waits  the time that the user input at the start of the program
    }
}
