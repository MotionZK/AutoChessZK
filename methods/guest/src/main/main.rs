#![no_main]
#![no_std]  

risc0_zkvm::guest::entry!(main);
/// main entry module for the guest application.

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


