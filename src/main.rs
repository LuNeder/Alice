use std::env;

fn main() {
    println!("Not ready yet. Use branch alice-python instead.");
    let args: Vec<String> = env::args().collect();
    if args.contains("help") {
        alice_help();
    } else if args.contains("loop") {
        alice_loop();
    } else {
        alice();
    };

}
fn alice_help() {
    println!("help");
}
fn alice_loop() {
    println!("loop");
}
fn alice() {
    println!("alice");
}
