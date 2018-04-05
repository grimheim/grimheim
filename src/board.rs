//! Grimheim board
//!
//! I honestly have no idea if this is going to work, but I needed to write
//! something to get started, this is probably going to be a good spot to
//! refactor when I am more comforable.
//!
//! Each tile is 2 offset, (this allows for a 1 offset to interlock rows)
//! TODO: I dislike the current board constructor, this should be fixed
//! TODO: Should sanity check to make sure offsets are even/odd

#[derive(Debug, PartialEq)]
struct Tile { }

impl Default for Tile {
    fn default() -> Self {
        Tile { }
    }
}

#[derive(Debug)]
struct BoardRow {
    tiles: Vec<Tile>,
    offset: u8,
}

impl BoardRow {
    fn new(offset: u8, length: u8) -> Self {
        let mut tiles: Vec<Tile> = Vec::with_capacity(length as usize);
        for _ in 0..length {
            tiles.push(Tile::default());
        }
        BoardRow {
            tiles,
            offset,
        }
    }
}

#[derive(Debug)]
struct Board {
    rows: Vec<BoardRow>
}

impl Board {
    fn new(shape: &[(u8, u8)]) -> Self {
        let mut rows: Vec<BoardRow> = Vec::with_capacity(shape.len());
        for i in 0..shape.len() {
            rows.push(BoardRow::new(shape[i].0, shape[i].1));
        }
        Board {
            rows
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_tile() {
        let t = Tile::default();
        assert_eq!(t, Tile { });
    }

    #[test]
    fn new_boardrow() {
        let r = BoardRow::new(0, 10);
        assert_eq!(r.offset, 0);
        assert_eq!(r.tiles.len(), 10);
    }

    #[test]
    fn new_board() {
        let rows = [(0, 10), (2, 8), (4, 4)];
        let b = Board::new(&rows);
        for (i, row) in rows.iter().enumerate() {
            assert_eq!(b.rows[i].offset, row.0);
            assert_eq!(b.rows[i].tiles.len(), row.1 as usize);
        }
    }
}
