use alloc::vec::Vec;
use serde::Serialize;
use risc0_zkvm::{
    serde::to_vec
};

#[cfg(feature = "guest")]
use risc0_zkvm::guest::env;

// instruction format to pass between host and guest
// should be generalizable to other applications
#[derive(Debug, Clone, Serialize)]
pub struct Instruction {
    pub code: u8,
    pub data: Vec<u32>,
}

impl Instruction {
    pub fn new<T: serde::Serialize>(code: u8, data: &T) -> Instruction {
        Instruction {
            code,
            data: to_vec(data).unwrap(),
        }
    }

    #[cfg(feature = "guest")]
    fn commit_to_host<T: serde::Serialize>(&self) {
        env::commit(&self);
    }
}
