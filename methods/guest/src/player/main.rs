#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
#![no_std]  // std support is experimental


use chess_core;
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

// guest code acts as a player, receiving Turns from the host and returning move indexes
// to the host. The host will then make the move and send the next Turn to the guest.
// The guest will then return the next move index, and so on.
pub fn main() {
}

/// This function is called by the host to initialize the guest.
/// It is called once before the first call to `main`, taking input
/// from the host to configure and initialize the guest program.
fn init() {
    
}


