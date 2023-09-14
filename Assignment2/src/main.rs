#![allow(unused)]
#![allow(dead_code)]

use std::collections::HashMap;
use clap::Parser;

#[derive(Parser,Debug)]
struct CmdPath {
    #[clap(long,short)]
    p: String,

    #[clap(long,short)]
    i: String,

    #[clap(long,short)]
    o: String,
}

mod example1;
mod example2;

fn main() {
    
    //taking path from command:
    let cmdpath: Pth = CmdPath::parse();

    //compare the path & call func() accordingly
    if cmdpath.p == "example1" {
        example1::text_to_byte(&cmdpath);
    } else if cmdpath.p == "example2" {
        example2::byte_to_text(&cmdpath);
    }
    
}








