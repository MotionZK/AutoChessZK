// basic move ranker for chess. this is not supposed to be good, just fast.
// given a list of moves, it will return a list of moves sorted by how good they are.
// a total of 100 points are spread between different factors as weights.
// the factors are:
// 1. material gain/loss
// 2. forward movement
// 3. distance to center

use crate::chess::{Turn, SimpleMove};

struct Ranker {
    weights: [u8; 3],
    turn: Option<Turn>,
}

impl Ranker {
    /// new from weight array. checks if weights add up to 100.
    pub fn new_from_weights(weights: [u8; 3]) -> Ranker {
        assert_eq!(Ranker::is_valid_weights(weights), true);
        Ranker {
            weights: weights,
            turn: None,
        }
    }

    /// check if a weight array is valid.
    pub fn is_valid_weights(weights: [u8; 3]) -> bool {
        let mut sum = 0;
        for i in 0..3 {
            sum += weights[i];
        }
        sum == 100
    }

    /// self check if weights are valid.
    pub fn is_valid(&self) -> bool {
        Ranker::is_valid_weights(self.weights)
    }

    /// set a weight with index, split remaining points evenly to other weights.
    pub fn set_weight(&mut self, index: usize, weight: u8) {
        assert!(index < 3);
        assert!(weight <= 100);

        let r1 = (100 - weight) / 2;
        let mut r2 = Some((100 - weight) - r1);

        for i in 0..3 {
            if i == index {
                self.weights[i] = weight;
            } else {
                match r2 {
                    Some(x) => {
                        self.weights[i] = x;
                        r2 = None;
                    },
                    None => {
                        self.weights[i] = r1;
                    },
                } 
            }
        }
    }

    pub fn weights(&self) -> [u8; 3] {
        self.weights
    }
    
    //score a Move. Move is shakmaty::Move made available to us by the chess crate.
    //points are awarded based on weight criteria.
    pub fn score_move(&self, m: &SimpleMove) -> u8 {
        let mut score = 0; 
        //material gain/loss
        if m.capture {
            score += self.weights[0];
        }
        //forward movement - count how many squares forward the piece moved
        //TODO: use the actual API
        match m.from {
            Some(x) => {
                let from_rank = x / 8;
                let to_rank = m.to / 8;
                
                // add ranks forward to score
                if from_rank < to_rank {
                    score += self.weights[1] * (to_rank - from_rank) as u8;
                }
            },
            None => {},
        }
        score
    }


    
}
