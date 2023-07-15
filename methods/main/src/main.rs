#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
#![no_std]  // std support is experimental


use chess_core;
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

pub fn main() {

}

fn init() {
    
}


