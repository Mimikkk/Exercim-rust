const BOARD_SIZE: i32 = 8;

#[derive(Debug)]
pub struct ChessPosition {
    x: i32,
    y: i32,
}

#[derive(Debug)]
pub struct Queen {
    pos: ChessPosition,
    moves: [(i32, i32); 8],
}
fn is_safe(x: i32, y: i32) -> bool {
    BOARD_SIZE > x && x >= 0 && BOARD_SIZE > y && y >= 0
}


impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if !is_safe(rank, file) { return None }
        Some(ChessPosition { x: rank, y: file })
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen {pos: position, moves: [(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)]}
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        if self.pos.x == other.pos.x && self.pos.y == other.pos.y { return false }

        for (m,n) in self.moves.iter() {
            let mut a = self.pos.x - m;
            let mut b = self.pos.y - n;
            while is_safe(a+m,b+n) {
                a += m;
                b += n;
                if a == other.pos.x && b == other.pos.y {return true}
            }
        }
        false
    }
}
