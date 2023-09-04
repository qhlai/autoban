// mod common;

// mod server;
// mod client;
// mod insecure;
// use clap::Parser;
// use server;

use std::io::prelude::*;
pub fn pause() {
    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}
#[test] 
fn test(){
    // server_main();
}
