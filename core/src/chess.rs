use alloc::{borrow::ToOwned, vec::Vec};
use shakmaty::{Chess, Position, MoveList};

pub use shakmaty::Move;

pub type ColorBitboard = u64;
pub type RoleBitboard = [u64; 6];
// simple move sent to the player. player will just send the index of the move
// and the game module will apply its version with all information to the board
pub struct SimpleMove {
    pub from: Option<u32>, // player module will convert to Square
    pub to: u32,
    pub capture: bool,
}

impl From<Move> for SimpleMove {
    fn from(m: Move) -> Self {
        SimpleMove {
            from: match m.from() {
                Some(x) => Some(x.into()),
                None => None,
            },
            to: m.to().into(),
            capture: m.is_capture(),
        }
    }
}

// since we want a fog of war, only return player pieces and valid moves
// for the player
pub struct Turn {
    position: (ColorBitboard, Option<RoleBitboard>), // convert to color and role bitboards
    moves: Vec<SimpleMove>
}

impl Turn
{
    pub fn get_position(&self) -> &(ColorBitboard, Option<RoleBitboard>) {
        &self.position
    }

    pub fn get_moves(&self) -> &Vec<SimpleMove> {
        &self.moves
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
            move_list.push(SimpleMove {
                from: Some(m.from().unwrap().into()),
                to: m.to().into(),
                capture: m.is_capture(),
            });
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
    
    // generate_turn will generate a turn for the player, giving
    // them context of just their color's pieces and valid moves
    pub fn generate_turn(&mut self) -> Turn {

        let color_bb: ColorBitboard = self.position.us().0;
        self.find_moves();

        Turn {
            position: (color_bb, None),
            moves: self.simple_moves(),
        }
    }
}
