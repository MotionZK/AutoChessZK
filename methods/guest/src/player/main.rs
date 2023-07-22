#![no_main]
#![no_std] 

use risc0_zkvm::guest::env;
use chess_core::{instruction::Instruction, types::PlayerState, chess::Turn};

risc0_zkvm::guest::entry!(main);

// guest code acts as a player, receiving Turns from the host and returning move indexes
// to the host. The host will then make the move and send the next Turn to the guest.
// The guest will then return the next move index, and so on.
pub fn main() {
    // initialize the guest, state should always point to the same location
    // in memory, so we can use a static reference
    let mut state = init();
    loop {
        // read the next turn from the host
        // TODO: syscalls/instructions for receiving data mid-loop
        state.ranker().set_turn(env::read::<Turn>());

        // get the best move from the ranker
        let best_move = state.ranker.best_move().unwrap().0;

        // create and execute play instruction
        let mut inx = Instruction::new(1, &best_move);

        if inx.execute::<u32>() != 0 {
            return
        }
    }
}

fn init() -> PlayerState {
    let state = PlayerState::init();
    state
}

