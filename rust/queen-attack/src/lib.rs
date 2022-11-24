#[derive(Debug)]
pub struct ChessPosition {
    rank: u8,
    file: u8,
}

#[derive(Debug)]
pub struct Queen(ChessPosition);

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        ((0..Self::NUM_TILES).contains(&rank) && (0..Self::NUM_TILES).contains(&file)).then_some(
            Self {
                rank: rank as u8,
                file: file as u8,
            },
        )
    }

    const NUM_TILES: i32 = 8;
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self(position)
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.0.rank == other.0.rank
            || self.0.file == other.0.file
            || self.0.rank.abs_diff(other.0.rank) == self.0.file.abs_diff(other.0.file)
    }
}
