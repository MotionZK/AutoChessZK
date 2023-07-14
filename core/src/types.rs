/// weights type to wrap [u8; 3]
pub type Weights = [u8; 3];

/// move index type to wrap u8
pub type MoveIndex = u8;

/// player config
#[derive(Debug, Clone, Copy)]
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

