use alloc::vec::Vec;
use risc0_zkvm::{
    serde::{from_slice, to_vec},
    declare_syscall
};
use serde::Serialize;

#[cfg(feature = "guest")]
use risc0_zkvm::guest::env;

// board instructions
declare_syscall!(CHESS_PASS_PLAYERS);
declare_syscall!(CHESS_PASS_TURN);
declare_syscall!(CHESS_GET_TURN);

// player instructions
declare_syscall!(PLAYER_GET_TURN);
declare_syscall!(PLAYER_PASS_MOVE);

// instruction format to pass between host and guest
// should be generalizable to other applications
#[derive(Debug, Clone, Serialize)]
pub struct Instruction {
    code: u8,
    data: Vec<u32>,
    spent: bool,
}

impl Instruction {
    pub fn new<T: serde::Serialize>(code: u8, data: &T) -> Self {
        Instruction {
            code,
            data: to_vec(data).unwrap(),
            spent: false,
        }
    }

    pub fn code(&self) -> u8 {
        self.code
    }

    #[cfg(feature = "guest")]
    pub fn execute<T: serde::de::DeserializeOwned>(&mut self) -> T {
        assert!(!self.spent, "instruction already spent");
        
        let data = self.data.as_slice();
        let call = match self.code {
            0 => CHESS_PASS_PLAYERS,
            1 => CHESS_PASS_TURN,
            2 => CHESS_GET_TURN,
            3 => PLAYER_PASS_MOVE,
            4 => PLAYER_GET_TURN,
            _ => panic!("invalid instruction code"),
        };

        self.spent = true;
        from_slice(&env::send_recv_slice::<u32, u8>(call, data)).unwrap()
    }
}
