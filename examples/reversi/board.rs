use std::ops::Deref;

use turtle::Color;

use tiles::{Tiles, Position};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    A,
    B,
}

impl Piece {
    pub fn name(self) -> &'static str {
        match self {
            Piece::A => "red",
            Piece::B => "blue",
        }
    }

    pub fn other(self) -> Self {
        match self {
            Piece::A => Piece::B,
            Piece::B => Piece::A,
        }
    }

    pub fn color(self) -> Color {
        match self {
            Piece::A => "#f44336".into(),
            Piece::B => "#2196F3".into(),
        }
    }
}

fn valid_moves_for(tiles: &Tiles, piece: Piece) -> Vec<Position> {
    // ALGORITHM: Go through all rows, columns and diagonals. Look through each row forwards and
    // backwards. Find an empty tile followed by the other piece. If you can then find another
    // `piece` before finding an empty tile, the empty tile is a valid move.

    let other = piece.other();
    //TODO: Convert each row into an iterator of (pos, tile)
    let rows = search_row(tiles.rows().enumerate().map(|(i, r)| r.enumerate().map(|(j, c)| ((i, j), c))))
        .chain(search_row(tiles.cols().enumerate().map(|(j, c)| c.enumerate().map(|(i, r)| ((i, j), c)))))
        .chain(tiles.diagonals_tlbr())
        .chain(tiles.diagonals_trbl())
        .collect();
    for pieces in rows {
        let potential_move = None;
    }
}

//TODO: Change return type into iterator
fn search_row<R, C>(row: R) -> Vec<Position>
    where R: Iterator<Item=C>,
          C: Iterator<Item=(Position, Option<Piece>)> {
}

#[derive(Debug)]
pub struct Board {
    current: Piece,
    /// None - empty tile
    /// Some(Piece::A) - occupied by piece A
    /// Some(Piece::B) - occupied by piece B
    ///
    /// Each array in Board is a row of the board
    tiles: Tiles,
    valid_moves: Vec<Position>
}

impl Deref for Board {
    type Target = Tiles;

    fn deref(&self) -> &Self::Target {
        &self.tiles
    }
}

impl Board {
    pub fn new() -> Self {
        let mut tiles: Tiles = Default::default();
        tiles[3][3] = Some(Piece::A);
        tiles[3][4] = Some(Piece::B);
        tiles[4][3] = Some(Piece::B);
        tiles[4][4] = Some(Piece::A);
        let current = Piece::A;
        let valid_moves = valid_moves_for(&tiles, current);

        Self {
            current,
            tiles,
            valid_moves,
        }
    }

    pub fn current(&self) -> Piece {
        self.current
    }

    pub fn valid_moves(&self) -> &[Position] {
        &self.valid_moves
    }

    pub fn is_valid_move(&self, position: &Position) -> bool {
        self.valid_moves.contains(position)
    }

    pub fn play_piece(&mut self, pos: Position) {
        if self.is_valid_move(&pos) {
            assert!(self[pos.0][pos.1].is_none(), "Valid move was not an empty tile!");
            self.tiles[pos.0][pos.1] = Some(self.current);
            self.flip_tiles(pos);

            self.valid_moves = vec![]; //TODO
            self.current = self.current.other();
        }
        else {
            unreachable!("Game should check for whether a valid move was used before playing it");
        }
    }

    fn flip_tiles(&mut self, start: Position) {
        unimplemented!()
    }
}
