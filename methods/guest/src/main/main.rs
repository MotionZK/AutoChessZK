#![no_main]
#![no_std]  

// TODO: fix this mess
use risc0_zkvm::guest::env;
use chess_core::{instruction::Instruction, types::{PlayerState, PlayerConfig}, chess::Turn};
use chess_core::types::GameState;
use chess_core::chess::Board;

risc0_zkvm::guest::entry!(main);

/// guest code that acts as the main module.
/// controls the board and the game loop. sends turns to the host and receives
/// move indexes from the host/other (player) modules.
pub fn main() {
    let mut state = init();
    
    loop {
    
    }
}

/// This function is called by the host to initialize the guest.
/// It is called once before the first call to `main`, taking input
/// from the host to configure and initialize the guest program.
fn init() -> GameState {
    let state = GameState::init();
    state
}
