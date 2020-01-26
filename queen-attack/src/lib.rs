#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank > -1 && rank < 8 && file > -1 && file < 8 {
            return Some(Self { rank, file });
        }
        None
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        (self.position.file - other.position.file).abs()
            == (self.position.rank - other.position.rank).abs()
            || self.position.file == other.position.file
            || self.position.rank == other.position.rank
    }
}
