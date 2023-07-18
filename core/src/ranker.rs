// basic move ranker for chess. this is not supposed to be good, just fast.
// given a list of moves, it will return a list of moves sorted by how good they are.
// a total of 100 points are spread between different factors as weights.
// the factors are:
// 1. material gain/loss
// 2. forward movement
// 3. distance to center

use crate::chess::{Turn, SimpleMove};

pub struct Ranker {
    weights: [u8; 3],
    turn: Option<Turn>,
}

impl Ranker {
    /// new from weight array. checks if weights add up to 100.
    pub fn new_from_weights(weights: [u8; 3]) -> Ranker {
        assert_eq!(Ranker::is_valid_weights(weights), true);
        Ranker {
            weights,
            turn: None,
        }
    }

    /// check if a weight array is valid.
    pub fn is_valid_weights(weights: [u8; 3]) -> bool {
        let mut sum = 0;
        for i in weights.iter() {
            sum += i;
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
    
    /// score the move with the ranker's weights.
    pub fn score_move(&self, m: &SimpleMove) -> i32 {
        //TODO: potential bitshift optimization, maybe not worth 
        let (mut material, mut forward, mut center) = (0, 0, 0);
        
        // material gain with value of captured piece, or 0 if no capture
        material += m.capture_value(); 

        //forward movement
        forward += m.rank_travel();
        
        // center progress
        center += m.center_travel();

        // calculate score with weights, using integer math
        ((material * self.weights[0] as i32) / 100) +
        ((forward * self.weights[1] as i32) / 100) +
        ((center * self.weights[2] as i32) / 100)
    }

    /// return the index of the best move in the turn's move list 
    /// a reference to the move itself.
    pub fn best_move(&self) -> Option<(usize, &SimpleMove)> {
        match self.turn() {
            Some(t) => {
                let mut best = 0;
                let mut best_score = 0;
                for (i, m) in t.get_moves().iter().enumerate() {
                    let score = self.score_move(m);
                    if score > best_score {
                        best = i;
                        best_score = score;
                    }
                }
                Some((best, &t.get_moves()[best]))
            },
            None => None,
        }
    }

    pub fn turn(&self) -> Option<&Turn> {
        self.turn.as_ref()
    }

    pub fn set_turn(&mut self, turn: Turn) {
        self.turn = Some(turn);
    }

}
