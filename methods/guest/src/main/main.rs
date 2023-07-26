#![no_main]
#![no_std]  

// TODO: fix this mess
use risc0_zkvm::guest::env;
use chess_core::{instruction::Instruction, types::{PlayerState, PlayerConfig}, chess::Turn};
use chess_core::types::GameState;
use chess_core::chess::{Board, Outcome};

risc0_zkvm::guest::entry!(main);

/// guest code that acts as the main module.
/// controls the board and the game loop. sends turns to the host and receives
/// move indexes from the host/other (player) modules.
pub fn main() {
    let mut state: GameState = init();

    loop {
        let turn = state.board.generate_turn();
        // send turn to player
        Instruction::new(1, &turn).execute::<u32>();
        // receive move from player
        let player_move = Instruction::new(2, &turn).execute::<u32>(); //usize is probably a bad idea
        // really, these should be a single instruction, but with Motion Modules™
        // we don't play by anyone's rules
        state.board.play_move(player_move as usize);

        if state.board.is_game_over() {
            commit_outcome(&state);
            return
        }
    }
}

/// This function is called by the host to initialize the guest.
/// It is called once before the first call to `main`, taking input
/// from the host to configure and initialize the guest program.
fn init() -> GameState {
    let state = GameState::init();
    state
}

/// Parse the outcome of the game and commit it to the host.
pub fn commit_outcome(state: &GameState) {
    let outcome = state.board.outcome();
    env::commit(outcome);
}
