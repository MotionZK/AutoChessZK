use alloc::{borrow::ToOwned, vec::Vec};
use serde::{Serialize, Deserialize};
use shakmaty::{Chess, Position, MoveList, Square};

pub use shakmaty::Move;

pub type ColorBitboard = u64;
pub type RoleBitboard = [u64; 6];

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SimpleMove {
    from: Option<u8>, // player module will convert to Square
    to: u8,
    capture: Option<u8>,
}

impl SimpleMove {
    pub fn rank_travel(&self) -> i32 {
        let to_rank = Square::try_from(self.to).unwrap().rank();
    
        let from_rank = match self.from {
            Some(x) => Square::try_from(x).unwrap().rank(),
            None => to_rank,
        };

        (to_rank as i32) - (from_rank as i32)
    }

    pub fn file_travel(&self) -> i32 {
        let to_file = Square::try_from(self.to).unwrap().file();
    
        let from_file = match self.from {
            Some(x) => Square::try_from(x).unwrap().file(),
            None => to_file,
        };

        (to_file as i32) - (from_file as i32)
    }

    pub fn center_travel(&self) -> i32 {
        let to = Square::try_from(self.to).unwrap();
        let from = match self.from {
            Some(x) => Square::try_from(x).unwrap(),
            None => to,
        };

        let center = match (from.file() as u8, from.rank() as u8) {
            (3, 3) => Square::D4,
            (3, 4) => Square::D5,
            (4, 3) => Square::E4,
            (4, 4) => Square::E5,
            _ => Square::D4,
        };

        let postdist = to.distance(center);
        let predist = from.distance(center);

        (postdist as i32) - (predist as i32)
    }

    pub fn capture_value(&self) -> i32 {
        self.capture.unwrap_or(0) as i32
    } 
}

impl From<Move> for SimpleMove {
    fn from(m: Move) -> Self {
        SimpleMove {
            from: match m.from() {
                Some(x) => Some(x.into()),
                None => None,
            },
            to: m.to().into(),
            capture: match m.capture() {
                Some(x) => Some(x.into()),
                None => None,
            },
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Turn {
    position: (ColorBitboard, Option<RoleBitboard>), // convert to color and role bitboards
    moves: Vec<SimpleMove>
}

impl Turn
{
    pub fn new(p: (ColorBitboard, Option<RoleBitboard>), m: Vec<SimpleMove>) -> Self {
        Turn {
            position: p,
            moves: m,
        }
    }

    pub fn get_position(&self) -> &(ColorBitboard, Option<RoleBitboard>) {
        &self.position
    }

    pub fn get_moves(&self) -> &Vec<SimpleMove> {
        &self.moves
    }
}

impl IntoIterator for Turn {
    type Item = SimpleMove;
    type IntoIter = ::alloc::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.moves.into_iter()
    }
}

pub struct Board {
    position: Chess,
    moves: MoveList,
}

// main game module on the main contract
impl Board {
    pub fn new() -> Self {
        Board {
            position: Chess::default(),
            moves: MoveList::new(),
        }
    }
    
    fn find_moves(&mut self) {
        self.moves = self.position.legal_moves();
    }

    pub fn simple_moves(&self) -> Vec<SimpleMove> {
        let mut move_list: Vec<SimpleMove> = Vec::new();
        for m in &self.moves {
            move_list.push(m.to_owned().into());
        }
        move_list
    }

    fn get_move_from_index(&self, index: usize) -> Move {
        assert!(index < self.moves.len());
        self.moves[index].to_owned()
    }

    fn apply_move(&mut self, m: &Move) {
        assert!(self.position.is_legal(m));
        self.position.play_unchecked(m);
    }

    pub fn play_move(&mut self, index: usize) {
        let m = self.get_move_from_index(index);
        self.apply_move(&m);
    }
    
    pub fn generate_turn(&mut self) -> Turn {

        let color_bb: ColorBitboard = self.position.us().0;
        self.find_moves();

        Turn {
            position: (color_bb, None),
            moves: self.simple_moves(),
        }
    }

    /// get the side to move's pieces
    pub fn to_bitboard(&self) -> ColorBitboard {
        self.position.us().0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shakmaty::{Role, Square};

    #[test]
    fn test_simple_move() {
        let m = Move::Normal {
            role: Role::Pawn,
            from: Square::E2,
            to: Square::E4,
            capture: None,
            promotion: None,
        };

        let sm: SimpleMove = m.into();

        assert_eq!(sm.from, Some(52));
        assert_eq!(sm.to, 36);
        assert_eq!(sm.capture, None);
    }
}
