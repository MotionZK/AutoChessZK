use crate::ranker::Ranker;
use crate::chess::{Board, Turn};
use crate::instruction::Instruction;

use alloc::vec::Vec;
use serde::{Serialize, Deserialize};
use risc0_zkvm::guest::env;

/// weights type to wrap [u8; 3]
pub type Weights = [u8; 3];

/// move index type to wrap u8
pub type MoveIndex = u8;

/// player config
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PlayerConfig {
    weights: Weights,
    id: u8,
}

impl PlayerConfig {
    pub fn new(weights: Weights, id: u8) -> PlayerConfig {
        // validate weights
        let mut sum = 0;
        for i in weights.iter() {
            sum += i;
        }
        assert_eq!(sum, 100);

        PlayerConfig {
            weights,
            id,
        }
    }

    pub fn weights(&self) -> Weights {
        self.weights
    }

    pub fn id(&self) -> u8 {
        self.id
    }
}

pub struct PlayerState {
    id: u8,
    pub ranker: Ranker
}

impl PlayerState {
    /// new from player config read from env
    pub fn init() -> Self {
        let config: PlayerConfig = env::read();
        let ranker = Ranker::new_from_weights(config.weights());
        PlayerState {
            id: config.id(),
            ranker,
        }
    }

    pub fn ranker(&mut self) -> &mut Ranker {
        &mut self.ranker
    }

    pub fn id(&self) -> u8 {
        self.id
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GameConfig {
    pub player1: PlayerConfig,
    pub player2: PlayerConfig,
}

impl GameConfig {
    pub fn new(player1: PlayerConfig, player2: PlayerConfig) -> GameConfig {
        GameConfig {
            player1,
            player2,
        }
    }
}

pub struct GameState {
    pub board: Board,
    pub turn: Turn,
    player_ids: [u8; 2],
}

/// initializing game state should also instruct host to initialize player states
impl GameState {

    pub fn init() -> Self {
        let config: GameConfig = env::read();
        let player_ids = [config.player1.id(), config.player2.id()];
        let board = Board::new();
        let turn = Turn::new((board.to_bitboard(), None), Vec::new());

        Instruction::new(0, &config.player1).execute::<u32>();
        Instruction::new(0, &config.player2).execute::<u32>();

        GameState {
            board,
            turn,
            player_ids,
        }
    }
}
