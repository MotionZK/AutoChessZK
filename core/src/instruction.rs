use alloc::vec::Vec;
use risc0_zkvm::{
    serde::{from_slice, to_vec},
    declare_syscall
};
use serde::Serialize;

#[cfg(feature = "guest")]
use risc0_zkvm::guest::env;

declare_syscall!(PLAYER_PLAY_MOVE);
declare_syscall!(CHESS_PASS_TURN);

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

        let result = match self.code {
            1 => {
               env::send_recv_slice::<u32, u32>(PLAYER_PLAY_MOVE, &self.data.as_slice())
            },
            2 => {
               env::send_recv_slice::<u32, u32>(CHESS_PASS_TURN, &self.data.as_slice())
            },
            _ => panic!("invalid instruction code"),
        };
        
        self.spent = true;
        from_slice(&result).unwrap()
    }
}
